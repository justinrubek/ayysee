use crate::{
    codegen::CodeGenerator,
    error::{Error, Result},
    stack::Stack,
    util::{stack_pop, stack_push},
    Location, Pass,
};

use ayysee_parser::ast::{Expr, Value};
use stationeers_mips::{
    instructions::{Arithmetic, FlowControl, Instruction, Stack as StackInstruction},
    types::{Number, Register},
};

/// Emits code that evaluates an expression and pushes the result onto the stack.
pub(crate) fn generate_expr(
    expr: &Expr,
    stack: &mut Stack,
    codegen: &mut CodeGenerator,
    pass: Pass,
) -> Result<()> {
    match expr {
        Expr::Identifier(identifier) => {
            codegen.add_comment_line(format!("expr identifier {identifier:?}"));

            // Check if the identifier refers to a constant
            if let Some(value) = codegen.get_constant(identifier) {
                generate_expr(&Expr::Constant(value), stack, codegen, pass)?;
                return Ok(());
            }

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
