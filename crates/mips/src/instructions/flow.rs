use crate::types::RegisterOrNumber;

/// Instructions for flow control, branching, and jumping
pub enum FlowControl {
    /// Branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8)
    ///
    /// bap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    BranchAbsoluteLessThan {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
        d: RegisterOrNumber,
    },
    /// Branch to line c if a !=b and store next line number in ra
    ///
    /// bapal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    BranchAbsoluteLessThanAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
        d: RegisterOrNumber,
    },
    /// Branch to line c if abs(a) <= float.epsilon * 8
    ///
    /// bapz a(r?|num) b(r?|num) c(r?|num)
    BranchAbsoluteZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if abs(a) <= float.epsilon * 8
    ///
    /// bapzal a(r?|num) b(r?|num) c(r?|num)
    BranchAbsoluteZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a == b
    ///
    /// beq a(r?|num) b(r?|num) c(r?|num)
    BranchEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a == b and store next line number in ra
    ///
    /// beqal a(r?|num) b(r?|num) c(r?|num)
    BranchEqualAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line b if a == 0
    ///
    /// beqz a(r?|num) b(r?|num)
    BranchEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line b if a == 0 and store next line number in ra
    ///
    /// beqzal a(r?|num) b(r?|num)
    BranchEqualZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line c if a >= b
    ///
    /// bge a(r?|num) b(r?|num) c(r?|num)
    BranchGreaterOrEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a >= b and store next line number in ra
    ///
    /// bgeal a(r?|num) b(r?|num) c(r?|num)
    BranchGreaterOrEqualAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line b if a >= 0
    ///
    /// bgez a(r?|num) b(r?|num)
    BranchGreaterOrEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line b if a >= 0 and store next line number in ra
    ///
    /// bgezal a(r?|num) b(r?|num)
    BranchGreaterOrEqualZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line c if a > b
    ///
    /// bgt a(r?|num) b(r?|num) c(r?|num)
    BranchGreaterThan {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a > b and store next line number in ra
    ///
    /// bgtal a(r?|num) b(r?|num) c(r?|num)
    BranchGreaterThanAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line b if a > 0
    ///
    /// bgtz a(r?|num) b(r?|num)
    BranchGreaterThanZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line b if a > 0 and store next line number in ra
    ///
    /// bgtzal a(r?|num) b(r?|num)
    BranchGreaterThanZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line c if a <= b
    ///
    /// ble a(r?|num) b(r?|num) c(r?|num)
    BranchLessOrEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a <= b and store next line number in ra
    ///
    /// bleal a(r?|num) b(r?|num) c(r?|num)
    BranchLessOrEqualAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line b if a <= 0
    ///
    /// blez a(r?|num) b(r?|num)
    BranchLessOrEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line b if a <= 0 and store next line number in ra
    ///
    /// blezal a(r?|num) b(r?|num)
    BranchLessOrEqualZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line c if a < b
    ///
    /// blt a(r?|num) b(r?|num) c(r?|num)
    BranchLessThan {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a < b and store next line number in ra
    ///
    /// bltal a(r?|num) b(r?|num) c(r?|num)
    BranchLessThanAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line b if a < 0
    ///
    /// bltz a(r?|num) b(r?|num)
    BranchLessThanZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line b if a < 0 and store next line number in ra
    ///
    /// bltzal a(r?|num) b(r?|num)
    BranchLessThanZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line d if abs(a-b) > max(c*max(abs(a), abs(b)), float.epsilon*8)
    ///
    /// bna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    BranchNotApproximatelyEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
        d: RegisterOrNumber,
    },
    /// Branch to line d if abs(a-b) > max(c*max(abs(a), abs(b)), float.epsilon*8) and store next line number in ra
    ///
    /// bnaal a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    BranchNotApproximatelyEqualAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
        d: RegisterOrNumber,
    },
    /// Branch to line c if abs(a) > float.epsilon*8
    ///
    /// bnaz a(r?|num) b(r?|num) c(r?|num)
    BranchNotApproximatelyZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if abs(a) > float.epsilon*8 and store next line number in ra
    ///
    /// gnazal a(r?|num) b(r?|num) c(r?|num)
    BranchNotApproximatelyZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a != b
    ///
    /// bne a(r?|num) b(r?|num) c(r?|num)
    BranchNotEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line c if a != b and store next line number in ra
    ///
    /// bneal a(r?|num) b(r?|num) c(r?|num)
    BranchNotEqualAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Branch to line b if a != 0
    ///
    /// bnez a(r?|num) b(r?|num)
    BranchNotEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Branch to line b if a != 0 and store next line number in ra
    ///
    /// bnezal a(r?|num) b(r?|num)
    BranchNotEqualZeroAndLink {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Relative branch to line d if abs(a-b) <= max(c*max(abs(a), abs(b)), float.epsilon*8)
    ///
    /// brap a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    RelativeBranchApproximatelyEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
        d: RegisterOrNumber,
    },
    /// Relative branch to line c if abs(a) <= float.epsilon*8
    ///
    /// brapz a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchApproximatelyZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line c if a == b
    ///
    /// breq a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line b if a == 0
    ///
    /// breqz a(r?|num) b(r?|num)
    RelativeBranchEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Relative branch to line c if a >= b
    ///
    /// brge a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchGreaterOrEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line b if a >= 0
    ///
    /// brgez a(r?|num) b(r?|num)
    RelativeBranchGreaterOrEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Relative branch to line c if a > b
    ///
    /// brgt a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchGreaterThan {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line b if a > 0
    ///
    /// brgtz a(r?|num) b(r?|num)
    RelativeBranchGreaterThanZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Relative branch to line c if a <= b
    ///
    /// brle a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchLessOrEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line b if a <= 0
    ///
    /// brlez a(r?|num) b(r?|num)
    RelativeBranchLessOrEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Relative branch to line c if a < b
    ///
    /// brlt a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchLessThan {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line b if a < 0
    ///
    /// brltz a(r?|num) b(r?|num)
    RelativeBranchLessThanZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Relative branch to line d if abs(a-b) > max(c*max(abs(a), abs(b)), float.epsilon*8)
    ///
    /// brna a(r?|num) b(r?|num) c(r?|num) d(r?|num)
    RelativeBranchNotApproximatelyEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
        d: RegisterOrNumber,
    },
    /// Relative branch to line c if abs(a) > float.epsilon*8
    ///
    /// brnaz a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchNotApproximatelyZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line c if a != b
    ///
    /// brne a(r?|num) b(r?|num) c(r?|num)
    RelativeBranchNotEqual {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Relative branch to line b if a != 0
    ///
    /// brnez a(r?|num) b(r?|num)
    RelativeBranchNotEqualZero {
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Jump execution to line a
    ///
    /// j int
    Jump { a: i32 },
    /// Jump execution to line a and store next line number in ra
    ///
    /// jal int
    JumpAndLink { a: i32 },
    /// Relative jump execution to line a
    ///
    /// jr int
    JumpRelative { a: i32 },
}

impl std::fmt::Display for FlowControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlowControl::BranchAbsoluteLessThan { a, b, c, d } => write!(f, "bap {a} {b} {c} {d}"),
            FlowControl::BranchAbsoluteLessThanAndLink { a, b, c, d } => {
                write!(f, "bapal {a} {b} {c} {d}")
            }
            FlowControl::BranchAbsoluteZero { a, b, c } => write!(f, "bapz {a} {b} {c}"),
            FlowControl::BranchAbsoluteZeroAndLink { a, b, c } => write!(f, "bapzal {a} {b} {c}"),
            FlowControl::BranchEqual { a, b, c } => write!(f, "beq {a} {b} {c}"),
            FlowControl::BranchEqualAndLink { a, b, c } => write!(f, "beqal {a} {b} {c}"),
            FlowControl::BranchEqualZero { a, b } => write!(f, "beqz {a} {b}"),
            FlowControl::BranchEqualZeroAndLink { a, b } => write!(f, "beqzal {a} {b}"),
            FlowControl::BranchGreaterOrEqual { a, b, c } => write!(f, "bge {a} {b} {c}"),
            FlowControl::BranchGreaterOrEqualAndLink { a, b, c } => write!(f, "bgeal {a} {b} {c}"),
            FlowControl::BranchGreaterOrEqualZero { a, b } => write!(f, "bgez {a} {b}"),
            FlowControl::BranchGreaterOrEqualZeroAndLink { a, b } => write!(f, "bgezal {a} {b}"),
            FlowControl::BranchGreaterThan { a, b, c } => write!(f, "bgt {a} {b} {c}"),
            FlowControl::BranchGreaterThanAndLink { a, b, c } => write!(f, "bgtal {a} {b} {c}"),
            FlowControl::BranchGreaterThanZero { a, b } => write!(f, "bgtz {a} {b}"),
            FlowControl::BranchGreaterThanZeroAndLink { a, b } => write!(f, "bgtzal {a} {b}"),
            FlowControl::BranchLessOrEqual { a, b, c } => write!(f, "ble {a} {b} {c}"),
            FlowControl::BranchLessOrEqualAndLink { a, b, c } => write!(f, "bleal {a} {b} {c}"),
            FlowControl::BranchLessOrEqualZero { a, b } => write!(f, "blez {a} {b}"),
            FlowControl::BranchLessOrEqualZeroAndLink { a, b } => write!(f, "blezal {a} {b}"),
            FlowControl::BranchLessThan { a, b, c } => write!(f, "blt {a} {b} {c}"),
            FlowControl::BranchLessThanAndLink { a, b, c } => write!(f, "bltal {a} {b} {c}"),
            FlowControl::BranchLessThanZero { a, b } => write!(f, "bltz {a} {b}"),
            FlowControl::BranchLessThanZeroAndLink { a, b } => write!(f, "bltzal {a} {b}"),
            FlowControl::BranchNotApproximatelyEqual { a, b, c, d } => {
                write!(f, "bna {a} {b} {c} {d}")
            }
            FlowControl::BranchNotApproximatelyEqualAndLink { a, b, c, d } => {
                write!(f, "bnaal {a} {b} {c} {d}")
            }
            FlowControl::BranchNotApproximatelyZero { a, b, c } => write!(f, "bnaz {a} {b} {c}"),
            FlowControl::BranchNotApproximatelyZeroAndLink { a, b, c } => {
                write!(f, "bnazal {a} {b} {c}")
            }
            FlowControl::BranchNotEqual { a, b, c } => write!(f, "bne {a} {b} {c}"),
            FlowControl::BranchNotEqualAndLink { a, b, c } => write!(f, "bneal {a} {b} {c}"),
            FlowControl::BranchNotEqualZero { a, b } => write!(f, "bnez {a} {b}"),
            FlowControl::BranchNotEqualZeroAndLink { a, b } => write!(f, "bnezal {a} {b}"),
            FlowControl::RelativeBranchApproximatelyEqual { a, b, c, d } => {
                write!(f, "brap {a} {b} {c} {d}")
            }
            FlowControl::RelativeBranchApproximatelyZero { a, b, c } => {
                write!(f, "brapz {a} {b} {c}")
            }
            FlowControl::RelativeBranchEqual { a, b, c } => write!(f, "breq {a} {b} {c}"),
            FlowControl::RelativeBranchEqualZero { a, b } => write!(f, "breqz {a} {b}"),
            FlowControl::RelativeBranchGreaterOrEqual { a, b, c } => write!(f, "brge {a} {b} {c}"),
            FlowControl::RelativeBranchGreaterOrEqualZero { a, b } => write!(f, "brgez {a} {b}"),
            FlowControl::RelativeBranchGreaterThan { a, b, c } => write!(f, "brgt {a} {b} {c}"),
            FlowControl::RelativeBranchGreaterThanZero { a, b } => write!(f, "brgtz {a} {b}"),
            FlowControl::RelativeBranchLessOrEqual { a, b, c } => write!(f, "brle {a} {b} {c}"),
            FlowControl::RelativeBranchLessOrEqualZero { a, b } => write!(f, "brlez {a} {b}"),
            FlowControl::RelativeBranchLessThan { a, b, c } => write!(f, "brlt {a} {b} {c}"),
            FlowControl::RelativeBranchLessThanZero { a, b } => write!(f, "brltz {a} {b}"),
            FlowControl::RelativeBranchNotApproximatelyEqual { a, b, c, d } => {
                write!(f, "brna {a} {b} {c} {d}")
            }
            FlowControl::RelativeBranchNotApproximatelyZero { a, b, c } => {
                write!(f, "brnaz {a} {b} {c}")
            }
            FlowControl::RelativeBranchNotEqual { a, b, c } => write!(f, "brne {a} {b} {c}"),
            FlowControl::RelativeBranchNotEqualZero { a, b } => write!(f, "brnez {a} {b}"),
            FlowControl::Jump { a } => write!(f, "j {a}"),
            FlowControl::JumpAndLink { a } => write!(f, "jal {a}"),
            FlowControl::JumpRelative { a } => write!(f, "jr {a}"),
        }
    }
}
