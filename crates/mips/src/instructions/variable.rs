use crate::types::{Register, RegisterOrNumber};

/// Instructions for variable selection
pub enum VariableSelection {
    /// Register = 1 if abs(a-b) <= max(c*max(abs(a), abs(b)), float.epsilon*8) else 0
    ///
    /// sap r? a(r?|num) b(r?|num) c(r?|num)
    SelectApproximatelyEqual {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Register = 1 if abs(a) <= float.epsilon*8 else 0
    ///
    /// sapz r? a(r?|num) b(r?|num)
    SelectApproximatelyZero {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if device is not set else 0
    ///
    /// sdns r? d?
    SelectDeviceNotSet {
        register: Register,
        d: RegisterOrNumber,
    },
    /// Register = 1 if device is set else 0
    ///
    /// sdse r? d?
    SelectDeviceSet {
        register: Register,
        d: RegisterOrNumber,
    },
    /// Register = b if a != 0 else c
    ///
    /// select r? a(r?|num) b(r?|num) c(r?|num)
    Select {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Register = 1 if a == b else 0
    ///
    /// seq r? a(r?|num) b(r?|num)
    SelectEqual {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a == 0 else 0
    ///
    /// seqz r? a(r?|num)
    SelectEqualZero {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = 1 if a >= b else 0
    ///
    /// sge r? a(r?|num) b(r?|num)
    SelectGreaterOrEqual {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a >= 0 else 0
    ///
    /// sgez r? a(r?|num)
    SelectGreaterOrEqualZero {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = 1 if a > b else 0
    ///
    /// sgt r? a(r?|num) b(r?|num)
    SelectGreaterThan {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a > 0 else 0
    ///
    /// sgtz r? a(r?|num)
    SelectGreaterThanZero {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = 1 if a <= b else 0
    ///
    /// sle r? a(r?|num) b(r?|num)
    SelectLessOrEqual {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a <= 0 else 0
    ///
    /// slez r? a(r?|num)
    SelectLessOrEqualZero {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = 1 if a < b else 0
    ///
    /// slt r? a(r?|num) b(r?|num)
    SelectLessThan {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a < 0 else 0
    ///
    /// sltz r? a(r?|num)
    SelectLessThanZero {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = 1 if abs(a-b) > max(c*max(abs(a), abs(b)), float.epsilon*8) else 0
    ///
    /// sna r? a(r?|num) b(r?|num) c(r?|num)
    SelectNotApproximatelyEqual {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Register = 1 if abs(a) > float.epsilon*8 else 0
    ///
    /// snaz r? a(r?|num) b(r?|num)
    SelectNotApproximatelyZero {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a != b else 0
    ///
    /// sne r? a(r?|num) b(r?|num)
    SelectNotEqual {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = 1 if a != 0 else 0
    ///
    /// snez r? a(r?|num)
    SelectNotEqualZero {
        register: Register,
        a: RegisterOrNumber,
    },
}

impl std::fmt::Display for VariableSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableSelection::SelectApproximatelyEqual { register, a, b, c } => {
                write!(f, "sap {register} {a} {b} {c}")
            }
            VariableSelection::SelectApproximatelyZero { register, a, b } => {
                write!(f, "sapz {register} {a} {b}")
            }
            VariableSelection::SelectDeviceNotSet { register, d } => {
                write!(f, "sdns {register} {d}")
            }
            VariableSelection::SelectDeviceSet { register, d } => {
                write!(f, "sdse {register} {d}")
            }
            VariableSelection::Select { register, a, b, c } => {
                write!(f, "select {register} {a} {b} {c}")
            }
            VariableSelection::SelectEqual { register, a, b } => {
                write!(f, "seq {register} {a} {b}")
            }
            VariableSelection::SelectEqualZero { register, a } => {
                write!(f, "seqz {register} {a}")
            }
            VariableSelection::SelectGreaterOrEqual { register, a, b } => {
                write!(f, "sge {register} {a} {b}")
            }
            VariableSelection::SelectGreaterOrEqualZero { register, a } => {
                write!(f, "sgez {register} {a}")
            }
            VariableSelection::SelectGreaterThan { register, a, b } => {
                write!(f, "sgt {register} {a} {b}")
            }
            VariableSelection::SelectGreaterThanZero { register, a } => {
                write!(f, "sgtz {register} {a}")
            }
            VariableSelection::SelectLessOrEqual { register, a, b } => {
                write!(f, "sle {register} {a} {b}")
            }
            VariableSelection::SelectLessOrEqualZero { register, a } => {
                write!(f, "slez {register} {a}")
            }
            VariableSelection::SelectLessThan { register, a, b } => {
                write!(f, "slt {register} {a} {b}")
            }
            VariableSelection::SelectLessThanZero { register, a } => {
                write!(f, "sltz {register} {a}")
            }
            VariableSelection::SelectNotApproximatelyEqual { register, a, b, c } => {
                write!(f, "sna {register} {a} {b} {c}")
            }
            VariableSelection::SelectNotApproximatelyZero { register, a, b } => {
                write!(f, "snaz {register} {a} {b}")
            }
            VariableSelection::SelectNotEqual { register, a, b } => {
                write!(f, "sne {register} {a} {b}")
            }
            VariableSelection::SelectNotEqualZero { register, a } => {
                write!(f, "snez {register} {a}")
            }
        }
    }
}
