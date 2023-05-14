use crate::types::{Number, Register, RegisterOrNumber};

/// An enum representing miscellaneous Stationeers MIPS instructions.
/// These instructions are not part of any other category.
pub enum Misc {
    /// Labels register or device reference with name. device references affect what shows on the
    /// screws on the IC base
    ///
    /// alias str r?|d?
    Alias { name: String, target: String },
    /// Creates a label that will be replaced throughout the program with the provided value
    ///
    /// define str num
    Define { name: String, value: Number },
    /// Halt and catch fire
    ///
    /// hcf
    Halt,
    /// Register = a
    ///
    /// move r? a(r?|num)
    Move {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Pause execution for a seconds
    ///
    /// sleep a(r?|num)
    Sleep { a: RegisterOrNumber },
    /// Pause execution until the next tick
    ///
    /// yield
    Yield,
    /// Marks a location in the program
    ///
    /// label:
    Label { name: String },
}

impl std::fmt::Display for Misc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Misc::Alias { name, target } => write!(f, "alias {name} {target}"),
            Misc::Define { name, value } => write!(f, "define {name} {value}"),
            Misc::Halt => write!(f, "hcf"),
            Misc::Move { register, a } => write!(f, "move {register} {a}"),
            Misc::Sleep { a } => write!(f, "sleep {a}"),
            Misc::Yield => write!(f, "yield"),
            Misc::Label { name } => write!(f, "{name}:"),
        }
    }
}
