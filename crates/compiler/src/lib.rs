use std::collections::HashMap;

use ayysee_parser::ast::{Block, Expr, Identifier, Statement, Value};
use stationeers_mips::{
    instructions::{Arithmetic, FlowControl, Instruction, Misc, Stack as StackInstruction},
    types::{Number, Register},
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

struct CodeGenerator {
    instructions: Vec<Instruction>,
    labels: HashMap<String, i32>,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
        }
    }

    fn add_instruction(&mut self, instruction: stationeers_mips::instructions::Instruction) {
        self.instructions.push(instruction);
    }

    fn add_instructions(&mut self, instructions: Vec<stationeers_mips::instructions::Instruction>) {
        self.instructions.extend(instructions);
    }

    fn prepend_instruction(&mut self, instruction: stationeers_mips::instructions::Instruction) {
        self.instructions.insert(0, instruction);
    }

    fn add_label(&mut self, label: String) {
        /* Alternative implementation which inserts a label instruction:
        self.add_instruction(Instruction::from(Misc::Label {
            name: label.clone(),
        }));
        self.labels
            .insert(label, self.instructions.len() as i32);
        // */
        self.labels
            .insert(label, self.instructions.len() as i32 + 1);
    }

    fn has_label(&self, label: &str) -> bool {
        self.labels.contains_key(label)
    }

    fn get_code(&self) -> String {
        self.instructions
            .iter()
            .map(|instruction| format!("{}", instruction))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

/* stack_push! macro:
*  usage: stack_push!(codegen, Number::Int(0));
*  expands to:
*
* codegen.add_instruction(StackInstruction::Push {
*   a: Number::Int(0).into(),
* }.into());
*/
macro_rules! stack_push {
    ($codegen:ident, $value:expr) => {
        $codegen.add_instruction(StackInstruction::Push { a: $value.into() }.into());
    };
}

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

struct Stack {
    rsp_offset: i32,
    locals: HashMap<String, i32>,
    saved_registers: Vec<Register>,
}

impl Stack {
    fn new() -> Self {
        Self {
            rsp_offset: 0,
            locals: HashMap::new(),
            saved_registers: Vec::new(),
        }
    }

    fn allocate_local(&mut self, name: String, codegen: &mut CodeGenerator) {
        self.rsp_offset += 1;
        self.locals.insert(name, self.rsp_offset);
        stack_push!(codegen, Number::Int(0));
    }

    fn deallocate_local(&mut self, name: String) {
        self.rsp_offset -= 1;
        self.locals.remove(&name);
    }

    fn save_register(&mut self, register: Register, codegen: &mut CodeGenerator) {
        self.rsp_offset += 1;
        self.saved_registers.push(register);
        stack_push!(codegen, register);
    }

    fn restore_register(&mut self, register: Register, codegen: &mut CodeGenerator) {
        self.rsp_offset -= 1;
        self.saved_registers.pop();
        stack_pop!(codegen, register);
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

    for statement in &program.statements {
        generate_code(statement, &mut stack, &mut codegen, Pass::Second)?;
    }

    // ensure the existance of a main function
    if !codegen.has_label("main") {
        return Err(Error::UndefinedMain);
    }
    // Add instructions to exit program
    // TODO: instead, just set ra to len(instructions) + 2
    let last_line = codegen.instructions.len() as i32 + 2;
    // Add instructions to call main function
    let main_line = codegen.labels.get("main").ok_or(Error::UndefinedMain)?;
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

    Ok(codegen.get_code())
}

fn generate_expr(
    expr: &Expr,
    stack: &mut Stack,
    codegen: &mut CodeGenerator,
    pass: Pass,
) -> Result<()> {
    match expr {
        Expr::Identifier(identifier) => {
            if let Some(offset) = stack.locals.get(identifier.as_ref()) {
                // load the value value of the identifier from memory and push it onto the stack

                // adjust the stack pointer to be at the location of the local variable
                codegen.add_instruction(Instruction::from(Arithmetic::Subtract {
                    register: Register::Sp.into(),
                    a: Register::Sp.into(),
                    b: Number::Int(*offset as i32).into(),
                }));

                // peek the value from the stack
                codegen.add_instruction(Instruction::from(StackInstruction::Peek {
                    register: Register::R0,
                }));
                // restore the stack pointer to its original value
                codegen.add_instruction(Instruction::from(Arithmetic::Add {
                    register: Register::Sp.into(),
                    a: Register::Sp.into(),
                    b: Number::Int(*offset as i32).into(),
                }));
                // push the value onto the stack
                stack_push!(codegen, Register::R0);

                Ok(())
            } else {
                Err(Error::UndefinedVariable(identifier.to_string()))
            }
        }
        Expr::Constant(value) => {
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
                ayysee_parser::ast::BinaryOpcode::Greater => todo!(),
                ayysee_parser::ast::BinaryOpcode::GreaterEquals => todo!(),
                ayysee_parser::ast::BinaryOpcode::Lower => todo!(),
                ayysee_parser::ast::BinaryOpcode::LowerEquals => todo!(),
            }

            // push the result of the operation onto the stack
            stack_push!(codegen, Register::R0);

            Ok(())
        }
        Expr::UnaryOp(op, operand) => {
            // call `generate_expr` for the operand
            // pop the result of the operand off the stack and perform the operation
            todo!();
        }
    }
}

/*
pub enum Statement {
    Assignment {
        identifier: Identifier,
        expression: Box<Expr>,
    },
    Definition {
        identifier: Identifier,
        expression: Box<Expr>,
    },
    Alias {
        identifier: Identifier,
        alias: Identifier,
    },
    Constant(String),
    Function {
        identifier: Identifier,
        parameters: Vec<Identifier>,
        body: Block,
    },
    FunctionCall {
        identifier: Identifier,
        arguments: Vec<Box<Expr>>,
    },
    Block(Block),
}
*/

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
            if !stack.locals.contains_key(identifier.as_ref()) {
                return Err(Error::UndefinedVariable(identifier.to_string()));
            }

            generate_expr(expression, stack, codegen, pass)?;

            // Due to the above check, this should never fail
            if let Some(offset) = stack.locals.get(identifier.as_ref()) {
                // generate code for value expression

                // Now we need to store the result of the expression in the local variable.
                // Stationeers MIPS doesn't have a store instruction that takes an immediate offset,
                // so we need to do some stack pointer arithmetic to get the correct address.
                // First, we need to pop the result of the expression off the stack.
                // Then, we need to adjust the stack pointer to the correct offset for the local variable.
                // To store it, we will use Stack::Push which will increment the stack pointer (by
                // 1 word).
                // Then, we will increment the stack pointer by the offset of the local variable.
                // TODO: ensure there isn't an off-by-one error here

                // pop the result of the expression off the stack
                stack_pop!(codegen, Register::R0);

                // adjust the stack pointer to the correct offset for the local variable
                codegen.add_instruction(Instruction::from(Arithmetic::Subtract {
                    register: Register::Sp.into(),
                    a: Register::Sp.into(),
                    b: Number::Int(*offset as i32).into(),
                }));

                // store the result of the expression in the local variable
                stack_push!(codegen, Register::R0);

                // restore the stack pointer
                codegen.add_instruction(Instruction::from(Arithmetic::Add {
                    register: Register::Sp.into(),
                    a: Register::Sp.into(),
                    b: Number::Int(*offset as i32 - 1).into(),
                }));
            }

            Ok(())
        }
        Statement::Definition {
            identifier,
            expression,
        } => {
            // generate code for value expression
            generate_expr(expression, stack, codegen, pass)?;

            // Allocate space for local variable
            if let Pass::Second = pass {
                stack.allocate_local(identifier.to_string(), codegen);
            }

            Ok(())
        }
        Statement::Alias { identifier, alias } => {
            // TODO: We don't need to emit an instruction as long as we track the alias during
            // codegen. This could be made optional to reduce final code size.
            if let Pass::Second = pass {
                codegen.add_instruction(
                    Misc::Alias {
                        name: alias.to_string(),
                        target: identifier.to_string(),
                    }
                    .into(),
                );
            }

            Ok(())
        }
        // Statement::Constant is not currently used
        Statement::Constant(_) => todo!(),
        Statement::Function {
            identifier,
            parameters,
            body,
        } => {
            if let Pass::First = pass {
                codegen.add_label(identifier.to_string());
            }

            if let Pass::Second = pass {
                // function prologue
                // save registers

                let body = Statement::Block(body.clone());

                // allocate locals
                let mut locals = Vec::new();
                find_locals(&body, &mut locals);
                for local in &locals {
                    stack.allocate_local(local.to_string(), codegen);
                }

                // function body
                generate_code(&body, stack, codegen, pass)?;

                // function epilogue

                // restore saved registers

                // deallocate locals
                for local in locals {
                    stack.deallocate_local(local.to_string());
                }

                // return
                codegen.add_instruction(
                    // using beqz instead of j because we need to return to the caller which is
                    // stored in a register
                    FlowControl::BranchEqualZero {
                        a: Number::Int(0).into(),
                        b: stationeers_mips::types::RegisterOrNumber::Register(Register::Ra),
                    }
                    .into(),
                );
            }

            Ok(())
        }
        Statement::FunctionCall {
            identifier,
            arguments,
        } => {
            // pass arguments
            for argument in arguments {
                generate_expr(argument, stack, codegen, pass)?;
            }

            // save registers
            stack.save_register(Register::Ra, codegen);

            // call function
            if let Pass::Second = pass {
                let target_line = codegen.labels[identifier.as_ref()];
                codegen.add_instruction(
                    Instruction::FlowControl(FlowControl::JumpAndLink { a: target_line }).into(),
                );
            }

            // deallocate arguments
            for _ in arguments {
                stack_pop!(codegen, Register::R0);
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
                match *argument.clone() {
                    Expr::Identifier(identifier) => {
                        if !locals.contains(&identifier) {
                            locals.push(identifier.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}
