use crate::{
    codegen::CodeGenerator,
    error::{Error, Result},
    expr::generate_expr,
    stack::Stack,
    util::{assign_variable, function_return, stack_pop, stack_push},
    Location, Pass,
};
use ayysee_parser::ast::{Block, Expr, Identifier, IfStatement, Statement};
use stationeers_mips::{
    instructions::{
        Arithmetic, DeviceIo, FlowControl, Instruction, Misc, Stack as StackInstruction,
    },
    types::{Device, DeviceVariable, Number, Register},
};
use std::str::FromStr;

/// Evaluates a single statement and generates the corresponding MIPS assembly code.
pub(crate) fn generate_statement(
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
        Statement::Constant(identifier, value) => {
            codegen.add_constant(identifier.clone(), *value);

            Ok(())
        }
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
                generate_statement(&body, stack, codegen, pass)?;

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
                    stack_push!(codegen, Number::Int(0));
                    codegen.add_comment(format!("local {local:?}"));
                    stack.allocate_local(local.to_string());
                }

                // function body
                generate_statement(&body, stack, codegen, pass)?;

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
                        generate_statement(statement, stack, codegen, pass)?;
                    }
                }
            }

            Ok(())
        }
        Statement::Loop { body } => {
            let loop_label = stack.new_loop();

            codegen.add_label(loop_label.clone());

            generate_statement(&Statement::Block(body.clone()), stack, codegen, pass)?;

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
                    generate_statement(&Statement::Block(body.clone()), stack, codegen, pass)?;

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
                    generate_statement(&Statement::Block(body.clone()), stack, codegen, pass)?;

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
                    generate_statement(&Statement::Block(else_body.clone()), stack, codegen, pass)?;

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
        Statement::Yield => {
            codegen.add_instruction(Instruction::from(Misc::Yield));

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
