use crate::types::{Register, RegisterOrNumber};

/// Boolean logic instructions.
pub enum Logic {
    /// Register = 1 if a != 0 and b != 0 else 0
    ///
    /// and r? a(r?|num) b(r?|num)
    And {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a == 0 and b == 0 else 0
    ///
    /// nor r? a(r?|num) b(r?|num)
    Nor {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a and/or b != 0 else 0
    ///
    /// or r? a(r?|num) b(r?|num)
    Or {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if either a or b is nonzero, otherwise 0
    ///
    /// xor r? a(r?|num) b(r?|num)
    Xor {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
}

impl std::fmt::Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logic::And { register, a, b } => write!(f, "and {register} {a} {b}"),
            Logic::Nor { register, a, b } => write!(f, "nor {register} {a} {b}"),
            Logic::Or { register, a, b } => write!(f, "or {register} {a} {b}"),
            Logic::Xor { register, a, b } => write!(f, "xor {register} {a} {b}"),
        }
    }
}
