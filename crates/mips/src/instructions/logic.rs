use crate::types::{Register, RegisterOrNumber};

/// Bitwise and logical instructions.
pub enum Logic {
    /// Bitwise AND: result bit is 1 only if both bits are 1
    ///
    /// and r? a(r?|num) b(r?|num)
    And {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Bitwise NOR: result bit is 1 only if both bits are 0
    ///
    /// nor r? a(r?|num) b(r?|num)
    Nor {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Bitwise NOT: flips each bit
    ///
    /// not r? a(r?|num)
    Not {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Bitwise OR: result bit is 1 if either bit is 1
    ///
    /// or r? a(r?|num) b(r?|num)
    Or {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Bitwise XOR: result bit is 1 if bits differ
    ///
    /// xor r? a(r?|num) b(r?|num)
    Xor {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Arithmetic left shift: shifts bits left, fills right with zeros
    ///
    /// sla r? a(r?|num) b(r?|num)
    ShiftLeftArithmetic {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Logical left shift: shifts bits left, fills right with zeros
    ///
    /// sll r? a(r?|num) b(r?|num)
    ShiftLeftLogical {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Arithmetic right shift: shifts bits right, fills left with sign bit
    ///
    /// sra r? a(r?|num) b(r?|num)
    ShiftRightArithmetic {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Logical right shift: shifts bits right, fills left with zeros
    ///
    /// srl r? a(r?|num) b(r?|num)
    ShiftRightLogical {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Extract bit field from source starting at offset for length bits
    ///
    /// ext r? source(r?|num) offset(r?|num) length(r?|num)
    ExtractBits {
        register: Register,
        source: RegisterOrNumber,
        offset: RegisterOrNumber,
        length: RegisterOrNumber,
    },
    /// Insert bit field into register at offset for length bits
    ///
    /// ins r? field(r?|num) offset(r?|num) length(r?|num)
    InsertBits {
        register: Register,
        field: RegisterOrNumber,
        offset: RegisterOrNumber,
        length: RegisterOrNumber,
    },
}

impl std::fmt::Display for Logic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Logic::And { register, a, b } => write!(f, "and {register} {a} {b}"),
            Logic::Nor { register, a, b } => write!(f, "nor {register} {a} {b}"),
            Logic::Not { register, a } => write!(f, "not {register} {a}"),
            Logic::Or { register, a, b } => write!(f, "or {register} {a} {b}"),
            Logic::Xor { register, a, b } => write!(f, "xor {register} {a} {b}"),
            Logic::ShiftLeftArithmetic { register, a, b } => {
                write!(f, "sla {register} {a} {b}")
            }
            Logic::ShiftLeftLogical { register, a, b } => write!(f, "sll {register} {a} {b}"),
            Logic::ShiftRightArithmetic { register, a, b } => {
                write!(f, "sra {register} {a} {b}")
            }
            Logic::ShiftRightLogical { register, a, b } => write!(f, "srl {register} {a} {b}"),
            Logic::ExtractBits {
                register,
                source,
                offset,
                length,
            } => write!(f, "ext {register} {source} {offset} {length}"),
            Logic::InsertBits {
                register,
                field,
                offset,
                length,
            } => write!(f, "ins {register} {field} {offset} {length}"),
        }
    }
}
