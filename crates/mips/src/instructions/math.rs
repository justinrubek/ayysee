use crate::types::{Register, RegisterOrNumber};

/// Instructions for mathematical operations.
pub enum Arithmetic {
    /// Register = abs(a)
    ///
    /// abs r? a(r?|num)
    AbsoluteValue {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = acos(a)
    ///
    /// acos r? a(r?|num)
    ArcCosine {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = a + b
    ///
    /// add r? a(r?|num) b(r?|num)
    Add {
        /// the register to store the result in
        register: Register,
        /// the first operand
        a: RegisterOrNumber,
        /// the second operand
        b: RegisterOrNumber,
    },
    /// Register = asin(a)
    ///
    /// asin r? a(r?|num)
    ArcSine {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = atan(a)
    ///
    /// atan r? a(r?|num)
    ArcTangent {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = ceil(a)
    ///
    /// ceil r? a(r?|num)
    Ceiling {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = cos(a)
    ///
    /// cos r? a(r?|num)
    Cosine {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = a / b
    ///
    /// div r? a(r?|num) b(r?|num)
    Divide {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = exp(a)
    ///
    /// exp r? a(r?|num)
    Exponent {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = floor(a)
    ///
    /// floor r? a(r?|num)
    Floor {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = log(a)
    ///
    /// log r? a(r?|num)
    Logarithm {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = max(a, b)
    ///
    /// max r? a(r?|num) b(r?|num)
    Maximum {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = min(a, b)
    ///
    /// min r? a(r?|num) b(r?|num)
    Minimum {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = a mod b (NOT a % b)
    ///
    /// mod r? a(r?|num) b(r?|num)
    Mod {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = a * b
    ///
    /// mul r? a(r?|num) b(r?|num)
    Multiply {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = a random x with 0 <= x < 1
    ///
    /// rand r?
    Random {
        /// the register to store the result in
        register: Register,
    },
    /// Register = round(a) (round to nearest integer)
    ///
    /// round r? a(r?|num)
    Round {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = sin(a)
    ///
    /// sin r? a(r?|num)
    Sine {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = sqrt(a)
    ///
    /// sqrt r? a(r?|num)
    SquareRoot {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = a - b
    ///
    /// sub r? a(r?|num) b(r?|num)
    Subtract {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = tan(a)
    ///
    /// tan r? a(r?|num)
    Tangent {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = trunc(a) (round towards zero)
    ///
    /// trunc r? a(r?|num)
    Truncate {
        /// the register to store the result in
        register: Register,
        a: RegisterOrNumber,
    },
}

impl std::fmt::Display for Arithmetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arithmetic::AbsoluteValue { register, a } => write!(f, "abs {} {}", register, a),
            Arithmetic::ArcCosine { register, a } => write!(f, "acos {} {}", register, a),
            Arithmetic::Add { register, a, b } => write!(f, "add {} {} {}", register, a, b),
            Arithmetic::ArcSine { register, a } => write!(f, "asin {} {}", register, a),
            Arithmetic::ArcTangent { register, a } => write!(f, "atan {} {}", register, a),
            Arithmetic::Ceiling { register, a } => write!(f, "ceil {} {}", register, a),
            Arithmetic::Cosine { register, a } => write!(f, "cos {} {}", register, a),
            Arithmetic::Divide { register, a, b } => write!(f, "div {} {} {}", register, a, b),
            Arithmetic::Exponent { register, a } => write!(f, "exp {} {}", register, a),
            Arithmetic::Floor { register, a } => write!(f, "floor {} {}", register, a),
            Arithmetic::Logarithm { register, a } => write!(f, "log {} {}", register, a),
            Arithmetic::Maximum { register, a, b } => write!(f, "max {} {} {}", register, a, b),
            Arithmetic::Minimum { register, a, b } => write!(f, "min {} {} {}", register, a, b),
            Arithmetic::Mod { register, a, b } => write!(f, "mod {} {} {}", register, a, b),
            Arithmetic::Multiply { register, a, b } => write!(f, "mul {} {} {}", register, a, b),
            Arithmetic::Random { register } => write!(f, "rand {}", register),
            Arithmetic::Round { register, a } => write!(f, "round {} {}", register, a),
            Arithmetic::Sine { register, a } => write!(f, "sin {} {}", register, a),
            Arithmetic::SquareRoot { register, a } => write!(f, "sqrt {} {}", register, a),
            Arithmetic::Subtract { register, a, b } => write!(f, "sub {} {} {}", register, a, b),
            Arithmetic::Tangent { register, a } => write!(f, "tan {} {}", register, a),
            Arithmetic::Truncate { register, a } => write!(f, "trunc {} {}", register, a),
        }
    }
}
