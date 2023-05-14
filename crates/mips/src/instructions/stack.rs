use crate::types::{Register, RegisterOrNumber};

/// Instructions for operating on the stack
pub enum Stack {
    /// Register = top of stack
    ///
    /// peek r?
    Peek { register: Register },
    /// Register = top of stack, then pop (decrement sp)
    ///
    /// pop r?
    Pop { register: Register },
    /// Push a onto the stack (increment sp)
    ///
    /// push a(r?|num)
    Push { a: RegisterOrNumber },
}

impl std::fmt::Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stack::Peek { register } => write!(f, "peek {register}"),
            Stack::Pop { register } => write!(f, "pop {register}"),
            Stack::Push { a } => write!(f, "push {a}"),
        }
    }
}
