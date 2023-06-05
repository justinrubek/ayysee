use stationeers_mips::types::Register;

use crate::{
    codegen::CodeGenerator,
    error::{Error, Result},
    stack::Stack,
    statement::generate_statement,
};

pub mod codegen;
pub mod error;
pub mod expr;
pub mod stack;
pub mod statement;
pub mod util;

#[derive(Copy, Clone, Debug)]
/// The pass of the compiler.
/// Multiple passes are used so that the compiler can resolve forward references.
enum Pass {
    /// The first pass, which resolves forward references.
    /// Code will be generated to reserve space for instructions that come later.
    First,
    /// The second pass, which generates the actual code using the information from the first pass.
    Second,
}

#[derive(Copy, Clone, Debug)]
enum Location {
    /// A position on the stack
    Stack(i32),
    /// A register
    Register(Register),
}

/// Converts an entire program into MIPS assembly code.
/// This function is the entry point for the code generation and handles the
/// initial setup of the stack frame and code generator.
pub fn generate_program(program: ayysee_parser::ast::Program) -> Result<String> {
    let mut codegen = CodeGenerator::new();
    let mut stack = Stack::new();

    for statement in &program.statements {
        generate_statement(statement, &mut stack, &mut codegen, Pass::First)?;
    }

    codegen.clear_first_pass();
    stack.clear();

    for statement in &program.statements {
        generate_statement(statement, &mut stack, &mut codegen, Pass::Second)?;
    }

    // ensure the existance of a main function
    if !codegen.has_label("main") {
        return Err(Error::UndefinedMain);
    }
    // Add instructions to exit program
    // let last_line = codegen.instructions.len() as i32 + 2;
    // Add instructions to call main function
    let _main_line = codegen.labels.get("main").ok_or(Error::UndefinedMain)?;

    Ok(codegen.get_code())
}
