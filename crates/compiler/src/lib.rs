use std::collections::HashMap;

use ayysee_parser::ast::Statement;
use stationeers_mips::{
    instructions::{FlowControl, Instruction, Stack as StackInstruction},
    types::{Number, Register},
};

use crate::error::Result;

pub mod error;

struct CodeGenerator {
    instructions: Vec<Instruction>,
}

impl CodeGenerator {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn add_instruction(&mut self, instruction: stationeers_mips::instructions::Instruction) {
        self.instructions.push(instruction);
    }

    fn add_instructions(&mut self, instructions: Vec<stationeers_mips::instructions::Instruction>) {
        self.instructions.extend(instructions);
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
        self.rsp_offset -= 4;
        self.locals.insert(name, self.rsp_offset);
        stack_push!(codegen, Number::Int(0));
    }

    fn save_register(&mut self, register: Register, codegen: &mut CodeGenerator) {
        self.rsp_offset -= 4;
        self.saved_registers.push(register);
        stack_push!(codegen, register);
    }
}

/// Converts an entire program into MIPS assembly code.
/// This function is the entry point for the code generation and handles the
/// initial setup of the stack frame and code generator.
pub fn generate_program(program: ayysee_parser::ast::Program) -> Result<String> {
    let mut codegen = CodeGenerator::new();
    let mut stack = Stack::new();

    for statement in program.statements {
        generate_code(statement, &mut stack, &mut codegen)?;
    }

    Ok(codegen.get_code())
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
    statement: Statement,
    stack: &mut Stack,
    codegen: &mut CodeGenerator,
) -> Result<()> {
    match statement {
        Statement::Assignment {
            identifier,
            expression,
        } => todo!(),
        Statement::Definition {
            identifier,
            expression,
        } => todo!(),
        Statement::Alias { identifier, alias } => todo!(),
        Statement::Constant(_) => todo!(),
        Statement::Function {
            identifier,
            parameters,
            body,
        } => {
            // function prologue

            // allocate locals

            // function body

            // function epilogue

            // restore saved registers

            // deallocate locals

            // return
            todo!()
        }
        Statement::FunctionCall {
            identifier,
            arguments,
        } => {
            // pass arguments

            // save registers
            // No need to save ra since we're using jal
            // stack.save_register(Register::Ra, codegen);

            // call function
            codegen.add_instruction(
                Instruction::FlowControl(FlowControl::JumpAndLink {
                    a: identifier.into(),
                })
                .into(),
            );

            todo!()
        }
        Statement::Block(_) => todo!(),
    }
}
