use std::{collections::HashMap, str::FromStr};

use ayysee_parser::ast::{Block, Expr, Identifier, IfStatement, Statement, Value};
use stationeers_mips::{
    instructions::{
        Arithmetic, DeviceIo, FlowControl, Instruction, Misc, Stack as StackInstruction,
    },
    types::{Device, DeviceVariable, Number, Register},
};

use crate::error::{Error, Result};

pub mod error;

#[derive(Copy, Clone, Debug)]
/// The pass of the compiler.
/// Multiple passes are used so that the compiler can resolve forward references.
enum Pass {
    /// The first pass, which resolves forward references.
    /// Code will still be generated to reserve space for instructions that come later.
    /// However, the second pass will overwrite these instructions with the correct values.
    First,
    Second,
}

#[derive(Copy, Clone, Debug)]
enum Location {
    /// A position on the stack
    Stack(i32),
    /// A register
    Register(Register),
}

struct CodeGenerator {
    instructions: Vec<Instruction>,
    comments: HashMap<i32, String>,

    labels: HashMap<String, i32>,

    /// Device aliases
    devices: HashMap<Identifier, Device>,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            comments: HashMap::new(),
            labels: HashMap::new(),
            devices: HashMap::new(),
        }
    }

    /// Adds an instruction to the list of instructions.
    fn add_instruction(&mut self, instruction: stationeers_mips::instructions::Instruction) {
        self.instructions.push(instruction);
    }

    /// Adds a comment to a given line.
    fn insert_comment(&mut self, comment: String, line: i32) {
        self.comments.insert(line, comment);
    }

    /// Adds a comment to the last instruction.
    fn add_comment(&mut self, comment: String) {
        self.insert_comment(comment, self.instructions.len() as i32 - 1);
    }

    /// Adds a comment on a separate line.
    fn add_comment_line(&mut self, comment: String) {
        self.add_instruction(Instruction::from(Misc::Comment { comment }));
    }
    /// Creates a new label and adds it to the list of labels.
    fn add_label(&mut self, label: String) {
        // implementation that inserts a label instruction:
        self.add_instruction(Instruction::from(Misc::Label {
            name: label.clone(),
        }));
        self.labels.insert(label, self.instructions.len() as i32);
    }

    /// Checks if a label exists.
    fn has_label(&self, label: &str) -> bool {
        self.labels.contains_key(label)
    }

    /// Gets the address of a label.
    /// This should only be called after a pass has been completed to ensure that
    /// the label exists.
    fn get_label(&self, label: &str) -> Result<i32> {
        self.labels
            .get(label)
            .copied()
            .ok_or_else(|| unreachable!("label {} does not exist", label))
    }

    /// Clears out data from the first pass.
    /// This should be called before the second pass.
    fn clear_first_pass(&mut self) {
        self.comments.clear();
        self.instructions.clear();
    }

    // TODO: Rewrites usages of identifiers to refer to the device.
    // This is intended to be used as a lines of code optimization.
    // In all places where a device is referred to via alias, the alias is replaced with the
    // device's true name.
    // fn overwrite_aliases

    /// Combines all of the instructions into a single string.
    /// This string can be executed by the MIPS emulator.
    fn get_code(&self) -> String {
        // Get the comments as a vector of strings matching the instructions vector in length.
        let mut comments: Vec<Option<String>> = vec![None; self.instructions.len()];
        for (line, comment) in self.comments.iter() {
            comments[*line as usize] = Some(comment.clone());
        }

        self.instructions
            .iter()
            .zip(comments)
            .map(|(instruction, comment)| match comment {
                Some(comment) => format!("{instruction} # {comment}"),
                None => format!("{instruction}"),
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Adds an alias for a device.
    fn add_alias(&mut self, alias: Identifier, device: Device) {
        self.devices.insert(alias, device);
    }

    /// Gets the device that a given identifier refers to.
    /// This should only be called after a pass has been completed to ensure that the alias entry
    /// exists.
    fn get_device(&self, identifier: &Identifier) -> Result<Option<Device>> {
        Ok(self.devices.get(identifier).copied())
    }
}

/// Pushes a value onto the stack.
/// This can be any Register or Number.
/// usage: `stack_push!(codegen, Number::Int(0));`
///
/// expands to:
/// ```
/// codegen.add_instruction(StackInstruction::Push {
///     a: Number::Int(0).into(),
/// }.into());
///  ```
macro_rules! stack_push {
    ($codegen:ident, $value:expr) => {
        $codegen.add_instruction(StackInstruction::Push { a: $value.into() }.into());
    };
}

/// Pops a value from the stack into a register.
macro_rules! stack_pop {
    ($codegen:ident, $register:expr) => {
        $codegen.add_instruction(
            StackInstruction::Pop {
                register: $register.into(),
            }
            .into(),
        );
    };
}

/// Cause a function to return to the caller.
macro_rules! function_return {
    ($codegen:ident) => {
        $codegen.add_instruction(
            // using beqz instead of j because we need to return to the caller which is
            // stored in a register
            FlowControl::BranchEqualZero {
                // Specify 0 as the value so that the branch is always taken
                a: Number::Int(0).into(),
                b: Register::Ra.into(),
            }
            .into(),
        );
    };
}

/// Creates instructions in the second pass, but adds a dummy instruction in the first pass.
/// This allows the compiler to reserve space for the instruction in the first pass but then use a
/// value that is only known in the second pass.
/// Usage - `pass_instruction(codegen, pass, {
///     // code to generate in the second pass
#[allow(unused_macros)]
macro_rules! pass_instruction {
    ($codegen:ident, $pass:expr, $code:block) => {
        if let Pass::Second = $pass
            $code
        else {
            $codegen.add_instruction(Instruction::from(Arithmetic::Add {
                register: Register::R0.into(),
                a: Register::R0.into(),
                b: Number::Int(0).into(),
            }));
        }
    };
}

/// Assigns a value to a variable.
/// The variable can be stored on the stack or in a register.
/// If the variable is stored on the stack, the stack pointer will be adjusted in order to store
/// the value.
macro_rules! assign_variable {
    ($codegen:ident, $stack:ident, $location:expr, $value:expr) => {
        match $location {
            Location::Stack(offset) => {
                let offset = -(*offset);

                // adjust the stack pointer
                $codegen.add_instruction(Instruction::from(Arithmetic::Subtract {
                    register: Register::Sp,
                    a: Register::Sp.into(),
                    b: Number::Int(offset).into(),
                }));

                // store the result of the expression in the local variable
                stack_push!($codegen, Register::R0);

                // restore the stack pointer
                $codegen.add_instruction(Instruction::from(Arithmetic::Add {
                    register: Register::Sp,
                    a: Register::Sp.into(),
                    b: Number::Int(offset).into(),
                }));
            }
            Location::Register(register) => {
                // We can't assign a value to a register, so we add 0 to the value and store it in the register.
                $codegen.add_instruction(
                    Arithmetic::Add {
                        register: register.clone().into(),
                        a: $value.into(),
                        b: Number::Int(0).into(),
                    }
                    .into(),
                );
            }
        }
    };
}

/// Utility struct for managing the stack.
struct Stack {
    rsp_offset: i32,
    locals: HashMap<String, Location>,
    saved_registers: Vec<Register>,
    /// Keeps track of the loops that are currently active.
    loops: Vec<String>,

    loop_counter: i32,
    if_counter: i32,
}

impl Stack {
    fn new() -> Self {
        Self {
            rsp_offset: 0,
            locals: HashMap::new(),
            saved_registers: Vec::new(),
            loops: Vec::new(),
            loop_counter: 0,
            if_counter: 0,
        }
    }

    /// Allocates space on the stack for a local variable.
    /// The variable will be initialized to 0.
    fn allocate_local(&mut self, name: String) {
        self.rsp_offset += 1;
        self.locals.insert(name, Location::Stack(self.rsp_offset));
    }

    /// Makes the stack aware of a local variable that has already been allocated.
    /// This will not allocate any space on the stack but will allow the stack to
    /// reference the variable.
    fn allocate_local_at(&mut self, name: String, location: Location) {
        match location {
            Location::Stack(offset) => self
                .locals
                .insert(name, Location::Stack(self.rsp_offset + offset)),
            Location::Register(register) => self.locals.insert(name, Location::Register(register)),
        };
    }

    /// Deallocates a local variable.
    fn deallocate_local(&mut self, name: String) {
        self.rsp_offset -= 1;
        self.locals.remove(&name);
    }

    /// Allocates space on the stack for a saved register.
    fn save_register(&mut self, register: Register, codegen: &mut CodeGenerator) {
        self.rsp_offset += 1;
        self.saved_registers.push(register);
        stack_push!(codegen, register);
    }

    /// Deallocates a saved register and restores its value.
    fn restore_register(&mut self, register: Register, codegen: &mut CodeGenerator) {
        self.rsp_offset -= 1;
        self.saved_registers.pop();
        stack_pop!(codegen, register);
    }

    /// Marks the beginning of a loop.
    fn new_loop(&mut self) -> String {
        let name = format!("loop_{}", self.loop_counter);
        self.loop_counter += 1;
        self.loops.push(name.clone());

        name
    }

    fn new_if(&mut self) -> String {
        let name = format!("if_{}", self.if_counter);
        self.if_counter += 1;

        name
    }

    /// Marks the end of a loop.
    fn end_loop(&mut self) -> Option<String> {
        self.loops.pop()
    }

    /// Clears values between passes.
    fn clear(&mut self) {
        self.loop_counter = 0;
        self.if_counter = 0;
    }
}

/// Converts an entire program into MIPS assembly code.
/// This function is the entry point for the code generation and handles the
/// initial setup of the stack frame and code generator.
pub fn generate_program(program: ayysee_parser::ast::Program) -> Result<String> {
    let mut codegen = CodeGenerator::new();
    let mut stack = Stack::new();

    for statement in &program.statements {
        generate_code(statement, &mut stack, &mut codegen, Pass::First)?;
    }

    codegen.clear_first_pass();
    stack.clear();

    for statement in &program.statements {
        generate_code(statement, &mut stack, &mut codegen, Pass::Second)?;
    }

    // ensure the existance of a main function
    if !codegen.has_label("main") {
        return Err(Error::UndefinedMain);
    }
    // Add instructions to exit program
    // let last_line = codegen.instructions.len() as i32 + 2;
    // Add instructions to call main function
    let _main_line = codegen.labels.get("main").ok_or(Error::UndefinedMain)?;
    /*
    codegen.prepend_instruction(
        FlowControl::Jump {
            a: (*main_line + 1),
        }
        .into(),
    );
    codegen.prepend_instruction(
        Arithmetic::Add {
            register: Register::Ra.into(),
            a: Number::Int(0).into(),
            b: Number::Int(last_line).into(),
        }
        .into(),
    );
    */

    Ok(codegen.get_code())
}

/// Emits code that evaluates an expression and pushes the result onto the stack.
fn generate_expr(
    expr: &Expr,
    stack: &mut Stack,
    codegen: &mut CodeGenerator,
    pass: Pass,
) -> Result<()> {
    match expr {
        Expr::Identifier(identifier) => {
            codegen.add_comment_line(format!("expr identifier {identifier:?}"));

            let identifier_ref: &String = identifier.as_ref();
            if let Some(location) = stack.locals.get(identifier_ref) {
                match location {
                    Location::Register(register) => {
                        // push the value of the register onto the stack
                        stack_push!(codegen, *register);
                    }
                    Location::Stack(offset) => {
                        let offset = -(*offset);
                        // load the value value of the identifier from memory and push it onto the stack

                        // adjust the stack pointer to be at the location of the local variable
                        if offset != 1 {
                            codegen.add_instruction(Instruction::from(Arithmetic::Subtract {
                                register: Register::Sp,
                                a: Register::Sp.into(),
                                b: Number::Int(offset).into(),
                            }));
                            codegen.add_comment(format!(
                                "retieve {identifier:?} from offset {offset}"
                            ));
                        } else {
                            codegen.add_comment_line(format!(
                                "retieve {identifier:?} from offset {offset}"
                            ));
                        }

                        // peek the value from the stack
                        codegen.add_instruction(Instruction::from(StackInstruction::Peek {
                            register: Register::R0,
                        }));

                        if offset != 1 {
                            // restore the stack pointer to its original value
                            codegen.add_instruction(Instruction::from(Arithmetic::Add {
                                register: Register::Sp,
                                a: Register::Sp.into(),
                                b: Number::Int(offset).into(),
                            }));
                        }
                        // push the value onto the stack
                        stack_push!(codegen, Register::R0);
                    }
                }
                Ok(())
            } else {
                Err(Error::UndefinedVariable(identifier.to_string()))
            }
        }
        Expr::Constant(value) => {
            codegen.add_comment_line(format!("expr constant {value:?}"));

            match value {
                Value::Integer(i) => {
                    // push the integer onto the stack
                    stack_push!(codegen, Number::Int(*i as i32));
                }
                Value::Float(f) => {
                    // push the float onto the stack
                    stack_push!(codegen, Number::Float(*f as f32));
                }
                Value::Boolean(b) => {
                    // push the boolean onto the stack
                    stack_push!(codegen, Number::Int(if *b { 1 } else { 0 }));
                }
            }

            Ok(())
        }
        Expr::BinaryOp(left, op, right) => {
            codegen.add_comment_line(format!("expr binary op {op:?}"));

            // recursively call `generate_expr` for the left and right operands
            generate_expr(left, stack, codegen, pass)?;
            generate_expr(right, stack, codegen, pass)?;

            // pop the results of the left and right operands off the stack
            stack_pop!(codegen, Register::R1);
            stack_pop!(codegen, Register::R0);

            // perform operation
            match op {
                ayysee_parser::ast::BinaryOpcode::Add => {
                    codegen.add_instruction(Instruction::from(Arithmetic::Add {
                        register: Register::R0,
                        a: Register::R0.into(),
                        b: Register::R1.into(),
                    }));
                }
                ayysee_parser::ast::BinaryOpcode::Sub => {
                    codegen.add_instruction(Instruction::from(Arithmetic::Subtract {
                        register: Register::R0,
                        a: Register::R0.into(),
                        b: Register::R1.into(),
                    }));
                }
                ayysee_parser::ast::BinaryOpcode::Mul => {
                    codegen.add_instruction(Instruction::from(Arithmetic::Multiply {
                        register: Register::R0,
                        a: Register::R0.into(),
                        b: Register::R1.into(),
                    }));
                }
                ayysee_parser::ast::BinaryOpcode::Div => {
                    codegen.add_instruction(Instruction::from(Arithmetic::Divide {
                        register: Register::R0,
                        a: Register::R0.into(),
                        b: Register::R1.into(),
                    }));
                }
                ayysee_parser::ast::BinaryOpcode::Conj => todo!(),
                ayysee_parser::ast::BinaryOpcode::Disj => todo!(),
                ayysee_parser::ast::BinaryOpcode::Equals => {
                    if let Pass::Second = pass {
                        // Approach: have two sets of instructions that set r0 to either 0 or 1.
                        // Branch to the appropriate set of instructions based on the result of the comparison.
                        let target_line = codegen.instructions.len() + 2;
                        codegen.add_instruction(Instruction::from(FlowControl::BranchEqual {
                            a: Register::R0.into(),
                            b: Register::R1.into(),
                            c: Number::Int(target_line as i32).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(1).into(),
                        }));
                    } else {
                        // Reserve space for second pass by generating placeholder instructions
                        codegen.add_instruction(Instruction::from(FlowControl::BranchEqual {
                            a: Register::R0.into(),
                            b: Register::R1.into(),
                            c: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                    }
                }

                ayysee_parser::ast::BinaryOpcode::NotEquals => todo!(),
                ayysee_parser::ast::BinaryOpcode::Greater => {
                    if let Pass::Second = pass {
                        // Approach: have two sets of instructions that set r0 to either 0 or 1.
                        // Branch to the appropriate set of instructions based on the result of the comparison.
                        let target_line = codegen.instructions.len() + 2;
                        codegen.add_instruction(Instruction::from(
                            FlowControl::BranchGreaterThan {
                                a: Register::R0.into(),
                                b: Register::R1.into(),
                                c: Number::Int(target_line as i32).into(),
                            },
                        ));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(1).into(),
                        }));
                    } else {
                        // Reserve space for second pass by generating placeholder instructions
                        codegen.add_instruction(Instruction::from(
                            FlowControl::BranchGreaterThan {
                                a: Register::R0.into(),
                                b: Register::R1.into(),
                                c: Number::Int(0).into(),
                            },
                        ));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                    }
                }
                ayysee_parser::ast::BinaryOpcode::GreaterEquals => todo!(),
                ayysee_parser::ast::BinaryOpcode::Lower => {
                    if let Pass::Second = pass {
                        // Approach: have two sets of instructions that set r0 to either 0 or 1.
                        // Branch to the appropriate set of instructions based on the result of the comparison.
                        let target_line = codegen.instructions.len() + 2;
                        codegen.add_instruction(Instruction::from(FlowControl::BranchLessThan {
                            a: Register::R0.into(),
                            b: Register::R1.into(),
                            c: Number::Int(target_line as i32).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(1).into(),
                        }));
                    } else {
                        // Reserve space for second pass by generating placeholder instructions
                        codegen.add_instruction(Instruction::from(FlowControl::BranchLessThan {
                            a: Register::R0.into(),
                            b: Register::R1.into(),
                            c: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                        codegen.add_instruction(Instruction::from(Arithmetic::Add {
                            register: Register::R0,
                            a: Number::Int(0).into(),
                            b: Number::Int(0).into(),
                        }));
                    }
                }
                ayysee_parser::ast::BinaryOpcode::LowerEquals => todo!(),
            }

            // push the result of the operation onto the stack
            stack_push!(codegen, Register::R0);

            Ok(())
        }
        Expr::UnaryOp(op, _operand) => {
            codegen.add_comment_line(format!("expr unary op {op:?}"));

            // call `generate_expr` for the operand
            // pop the result of the operand off the stack and perform the operation
            todo!();
        }
    }
}

/// Evaluates a single statement and generates the corresponding MIPS assembly code.
fn generate_code(
    statement: &Statement,
    stack: &mut Stack,
    codegen: &mut CodeGenerator,
    pass: Pass,
) -> Result<()> {
    match statement {
        Statement::Assignment {
            identifier,
            expression,
        } => {
            codegen.add_comment_line(format!("Assignment: {identifier:?} {expression:?}"));

            let identifier_str: &str = identifier.as_ref();
            if !stack.locals.contains_key(identifier_str) {
                return Err(Error::UndefinedVariable(identifier.to_string()));
            }

            generate_expr(expression, stack, codegen, pass)?;

            // pop the result of the expression off the stack
            stack_pop!(codegen, Register::R0);

            // Due to the above check, this should never fail
            if let Some(location) = stack.locals.get(identifier_str) {
                assign_variable!(codegen, stack, location, Register::R0);
            }

            Ok(())
        }
        Statement::Definition {
            identifier,
            expression,
        } => {
            codegen.add_comment_line(format!("Definition: {identifier:?} {expression:?}"));
            // generate code for value expression
            generate_expr(expression, stack, codegen, pass)?;

            // Allocate space for local variable
            stack.allocate_local_at(identifier.to_string(), Location::Stack(-1));

            Ok(())
        }
        Statement::Alias { identifier, alias } => {
            let identifier_ref: &str = identifier.as_ref();
            codegen.add_alias(alias.clone(), Device::from_str(identifier_ref)?);

            // TODO: We don't need to emit an instruction as long as we track the alias during
            // codegen. This could be made optional to reduce final code size.
            codegen.add_instruction(
                Misc::Alias {
                    name: alias.to_string(),
                    target: identifier.to_string(),
                }
                .into(),
            );

            Ok(())
        }
        // Statement::Constant is not currently used
        Statement::Constant(_) => todo!(),

        Statement::Function {
            identifier,
            parameters,
            body,
        } => {
            codegen.add_label(identifier.to_string());
            codegen.add_comment(format!("Function: {identifier:?} {parameters:?}"));

            if let Pass::Second = pass {
                // function prologue

                let body = Statement::Block(body.clone());

                // allocate space for function parameters
                for (i, parameter) in parameters.iter().enumerate() {
                    if i < 4 {
                        // receive parameter from register
                        let register = Register::from(i as u8);
                        stack_push!(codegen, register);
                        codegen.add_comment(format!("parameter {parameter:?} from {register:?}"));
                        stack
                            .allocate_local_at(parameter.to_string(), Location::Register(register));
                    } else {
                        // receive parameter from stack
                        // the stack increases upwards, so we
                        let offset = (parameters.len() - i) as i32;
                        stack.allocate_local_at(parameter.to_string(), Location::Stack(offset));
                        codegen.add_comment_line(format!(
                            "parameter {parameter:?} from stack offset {offset:?}"
                        ));
                    }
                }

                // allocate locals
                let mut locals = Vec::new();
                find_locals(&body, &mut locals);
                for local in &locals {
                    stack_push!(codegen, Number::Int(0));
                    codegen.add_comment(format!("local {local:?}"));
                    stack.allocate_local_at(local.to_string(), Location::Stack(-1));
                }

                // function body
                generate_code(&body, stack, codegen, pass)?;

                // function epilogue

                // deallocate locals
                for local in locals {
                    stack.deallocate_local(local.to_string());
                }

                // deallocate parameters
                for parameter in parameters {
                    stack.deallocate_local(parameter.to_string());
                }
                function_return!(codegen);
            } else {
                // reserve space using placeholder instructions
                // allocate space for function parameters
                for (i, parameter) in parameters.iter().enumerate() {
                    if i < 4 {
                        // receive parameter from register
                        let register = Register::from(i as u8);
                        stack
                            .allocate_local_at(parameter.to_string(), Location::Register(register));
                    } else {
                        // receive parameter from stack
                        let offset = (parameters.len() - i) as i32;
                        stack.allocate_local_at(parameter.to_string(), Location::Stack(offset));
                    }
                }

                let body = Statement::Block(body.clone());

                // allocate locals
                let mut locals = Vec::new();
                find_locals(&body, &mut locals);
                for local in &locals {
                    stack.allocate_local(local.to_string());
                }

                // function body
                generate_code(&body, stack, codegen, pass)?;

                // function epilogue

                // deallocate locals
                for local in locals {
                    stack.deallocate_local(local.to_string());
                }

                // deallocate parameters
                for parameter in parameters {
                    stack.deallocate_local(parameter.to_string());
                }
                function_return!(codegen);
            }

            Ok(())
        }
        Statement::FunctionCall {
            identifier,
            arguments,
        } => {
            // pass arguments
            for (i, argument) in arguments.iter().enumerate() {
                generate_expr(argument, stack, codegen, pass)?;
                if i < 4 {
                    // pass argument as register
                    let register = Register::from(i as u8);
                    stack_pop!(codegen, register);
                } else {
                    // pass argument on the stack
                    // this is already done by generate_expr
                }
            }

            // save registers
            codegen.add_comment_line("saving registers".to_string());
            for register in &[
                Register::Ra,
                Register::R4,
                Register::R5,
                Register::R6,
                Register::R7,
            ] {
                stack.save_register(*register, codegen);
            }

            // call function
            if let Pass::Second = pass {
                let target_line = codegen.get_label(identifier.as_ref())?;
                codegen.add_instruction(FlowControl::JumpAndLink { a: target_line }.into());
                codegen.add_comment(format!("FunctionCall: {identifier:?} {arguments:?}"));
            } else {
                // reserve space for the second pass by adding a placeholder instruction
                codegen.add_instruction(FlowControl::JumpAndLink { a: 0 }.into());
            }

            // restore saved registers
            for register in &[
                Register::R7,
                Register::R6,
                Register::R5,
                Register::R4,
                Register::Ra,
            ] {
                stack.restore_register(*register, codegen);
            }

            // deallocate arguments
            for (i, _arg) in arguments.iter().enumerate() {
                if i >= 4 {
                    stack_pop!(codegen, Register::R0);
                }
            }

            Ok(())
        }
        Statement::Block(block) => {
            match block {
                Block::Statements(statements) => {
                    for statement in statements {
                        generate_code(statement, stack, codegen, pass)?;
                    }
                }
            }

            Ok(())
        }
        Statement::Loop { body } => {
            let loop_label = stack.new_loop();

            codegen.add_label(loop_label.clone());

            generate_code(&Statement::Block(body.clone()), stack, codegen, pass)?;

            // jump back to the start of the loop
            if let Pass::Second = pass {
                let line = codegen.get_label(&loop_label)?;
                codegen.add_instruction(FlowControl::Jump { a: line }.into());
            } else {
                // reserve space for the second pass by adding a placeholder instruction
                codegen.add_instruction(FlowControl::Jump { a: 0 }.into());
            }

            stack.end_loop();

            Ok(())
        }
        Statement::IfStatement(if_statement) => {
            match if_statement {
                IfStatement::If { condition, body } => {
                    // handle if without else
                    // evaluate the condition. If it is false, jump to the end of the if statement
                    generate_expr(condition, stack, codegen, pass)?;

                    // pop the condition from the stack
                    stack_pop!(codegen, Register::R0);

                    let if_label = stack.new_if();
                    let end_label = format!("{}_end", if_label);

                    // jump to the end of the if statement if the condition is false
                    if let Pass::Second = pass {
                        let line = codegen.get_label(&end_label)?;
                        codegen.add_instruction(Instruction::from(FlowControl::BranchEqualZero {
                            a: Register::R0.into(),
                            b: Number::Int(line).into(),
                        }));
                    } else {
                        // reserve space for the second pass by adding a placeholder instruction
                        codegen.add_instruction(Instruction::from(FlowControl::BranchEqualZero {
                            a: Register::R0.into(),
                            b: Number::Int(0).into(),
                        }));
                    }

                    // generate the if body
                    generate_code(&Statement::Block(body.clone()), stack, codegen, pass)?;

                    // add label for end of if statement
                    codegen.add_label(end_label);
                }
                IfStatement::IfElse {
                    condition,
                    body,
                    else_body,
                } => {
                    // handle if with else
                    generate_expr(condition, stack, codegen, pass)?;

                    // pop the condition from the stack
                    stack_pop!(codegen, Register::R0);

                    let if_label = stack.new_if();
                    let else_label = format!("{}_else", if_label);
                    let end_label = format!("{}_end", if_label);

                    // jump to the else statement if the condition is false
                    if let Pass::Second = pass {
                        let line = codegen.get_label(&else_label)?;
                        codegen.add_instruction(Instruction::from(FlowControl::BranchEqualZero {
                            a: Register::R0.into(),
                            b: Number::Int(line).into(),
                        }));
                    } else {
                        // reserve space for the second pass by adding a placeholder instruction
                        codegen.add_instruction(Instruction::from(FlowControl::BranchEqualZero {
                            a: Register::R0.into(),
                            b: Number::Int(0).into(),
                        }));
                    }

                    // generate the if body
                    generate_code(&Statement::Block(body.clone()), stack, codegen, pass)?;

                    // jump to end of if statement
                    if let Pass::Second = pass {
                        let line = codegen.get_label(&end_label)?;
                        codegen.add_instruction(Instruction::from(FlowControl::Jump { a: line }));
                    } else {
                        // reserve space for the second pass by adding a placeholder instruction
                        codegen.add_instruction(Instruction::from(FlowControl::Jump { a: 0 }));
                    }

                    // add label for else statement
                    codegen.add_label(else_label);

                    // generate the else body
                    generate_code(&Statement::Block(else_body.clone()), stack, codegen, pass)?;

                    // add label for end of if statement
                    codegen.add_label(end_label);
                }
            }

            Ok(())
        }
        Statement::DeviceStatement(device_statement) => {
            match device_statement {
                ayysee_parser::ast::DeviceStatement::Read {
                    device,
                    device_variable,
                    local,
                } => {
                    let local: &str = local.as_ref();
                    if !stack.locals.contains_key(local) {
                        return Err(Error::UndefinedVariable(local.to_string()));
                    }

                    if let Pass::Second = pass {
                        let device = codegen.get_device(device)?.unwrap();

                        let variable: &str = device_variable.as_ref();
                        let variable = DeviceVariable::from_str(variable)?;
                        // Load the device variable into a register
                        codegen.add_instruction(Instruction::from(DeviceIo::LoadDeviceVariable {
                            device,
                            variable,
                            register: Register::R0,
                        }));

                        if let Some(location) = stack.locals.get(local) {
                            assign_variable!(codegen, stack, location, Register::R0);
                        }
                    } else {
                        // reserve space for the second pass by adding a placeholder instruction
                        codegen.add_instruction(Instruction::from(DeviceIo::LoadDeviceVariable {
                            device: Device::D0,
                            variable: DeviceVariable::Setting,
                            register: Register::R0,
                        }));

                        if let Some(location) = stack.locals.get(local) {
                            assign_variable!(codegen, stack, location, Register::R0);
                        }
                    }
                }

                ayysee_parser::ast::DeviceStatement::Write {
                    value,
                    device,
                    device_variable,
                } => {
                    generate_expr(value, stack, codegen, pass)?;

                    // pop the value from the stack
                    stack_pop!(codegen, Register::R0);

                    if let Pass::Second = pass {
                        let device = codegen.get_device(device)?.unwrap();

                        let variable: &str = device_variable.as_ref();
                        let variable = DeviceVariable::from_str(variable)?;
                        // Load the device variable into a register
                        codegen.add_instruction(Instruction::from(DeviceIo::StoreDeviceVariable {
                            device,
                            variable,
                            register: Register::R0,
                        }));
                    } else {
                        // reserve space for the second pass by adding a placeholder instruction
                        codegen.add_instruction(Instruction::from(DeviceIo::StoreDeviceVariable {
                            device: Device::D0,
                            variable: DeviceVariable::Setting,
                            register: Register::R0,
                        }));
                    }
                }
            }

            Ok(())
        }
    }
}

/// Finds all of the locals used in a statement
fn find_locals(statement: &Statement, locals: &mut Vec<Identifier>) {
    match statement {
        Statement::Definition { identifier, .. } => {
            if !locals.contains(identifier) {
                locals.push(identifier.clone());
            }
        }
        Statement::Block(block) => match block {
            Block::Statements(statements) => {
                for statement in statements {
                    find_locals(statement, locals);
                }
            }
        },
        Statement::FunctionCall { arguments, .. } => {
            for argument in arguments {
                if let Expr::Identifier(identifier) = *argument.clone() {
                    if !locals.contains(&identifier) {
                        locals.push(identifier.clone());
                    }
                }
            }
        }
        _ => {}
    }
}
