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
/// Usage - `pass_instruction!(codegen, pass, {
///     // code to generate in the second pass
/// })`
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

pub(crate) use assign_variable;
pub(crate) use function_return;
#[allow(unused_imports)]
pub(crate) use pass_instruction;
pub(crate) use stack_pop;
pub(crate) use stack_push;
