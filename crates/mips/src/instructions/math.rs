use crate::types::{Register, RegisterOrNumber};

/// Instructions for mathematical operations.
pub enum Arithmetic {
    /// Register = abs(a)
    ///
    /// abs r? a(r?|num)
    AbsoluteValue {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = acos(a)
    ///
    /// acos r? a(r?|num)
    ArcCosine {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = a + b
    ///
    /// add r? a(r?|num) b(r?|num)
    Add {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = asin(a)
    ///
    /// asin r? a(r?|num)
    ArcSine {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = atan(a)
    ///
    /// atan r? a(r?|num)
    ArcTangent {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = atan2(a, b) (angle in radians whose tangent is a/b)
    ///
    /// atan2 r? a(r?|num) b(r?|num)
    ArcTangent2 {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = ceil(a)
    ///
    /// ceil r? a(r?|num)
    Ceiling {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = cos(a)
    ///
    /// cos r? a(r?|num)
    Cosine {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = a / b
    ///
    /// div r? a(r?|num) b(r?|num)
    Divide {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = exp(a)
    ///
    /// exp r? a(r?|num)
    Exponent {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = floor(a)
    ///
    /// floor r? a(r?|num)
    Floor {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = linear interpolation from a to b by ratio c (clamped 0-1)
    ///
    /// lerp r? a(r?|num) b(r?|num) c(r?|num)
    LinearInterpolation {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
        c: RegisterOrNumber,
    },
    /// Register = log(a) (natural logarithm, base e)
    ///
    /// log r? a(r?|num)
    Logarithm {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = max(a, b)
    ///
    /// max r? a(r?|num) b(r?|num)
    Maximum {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = min(a, b)
    ///
    /// min r? a(r?|num) b(r?|num)
    Minimum {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = a mod b
    ///
    /// mod r? a(r?|num) b(r?|num)
    Mod {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = a * b
    ///
    /// mul r? a(r?|num) b(r?|num)
    Multiply {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = a ^ b (power)
    ///
    /// pow r? a(r?|num) b(r?|num)
    Power {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = random value where 0 <= x < 1
    ///
    /// rand r?
    Random { register: Register },
    /// Register = round(a)
    ///
    /// round r? a(r?|num)
    Round {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = sin(a)
    ///
    /// sin r? a(r?|num)
    Sine {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = sqrt(a)
    ///
    /// sqrt r? a(r?|num)
    SquareRoot {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = a - b
    ///
    /// sub r? a(r?|num) b(r?|num)
    Subtract {
        register: Register,
        a: RegisterOrNumber,
        b: RegisterOrNumber,
    },
    /// Register = tan(a)
    ///
    /// tan r? a(r?|num)
    Tangent {
        register: Register,
        a: RegisterOrNumber,
    },
    /// Register = trunc(a) (round towards zero)
    ///
    /// trunc r? a(r?|num)
    Truncate {
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
            Arithmetic::ArcTangent2 { register, a, b } => {
                write!(f, "atan2 {} {} {}", register, a, b)
            }
            Arithmetic::Ceiling { register, a } => write!(f, "ceil {} {}", register, a),
            Arithmetic::Cosine { register, a } => write!(f, "cos {} {}", register, a),
            Arithmetic::Divide { register, a, b } => write!(f, "div {} {} {}", register, a, b),
            Arithmetic::Exponent { register, a } => write!(f, "exp {} {}", register, a),
            Arithmetic::Floor { register, a } => write!(f, "floor {} {}", register, a),
            Arithmetic::LinearInterpolation { register, a, b, c } => {
                write!(f, "lerp {} {} {} {}", register, a, b, c)
            }
            Arithmetic::Logarithm { register, a } => write!(f, "log {} {}", register, a),
            Arithmetic::Maximum { register, a, b } => write!(f, "max {} {} {}", register, a, b),
            Arithmetic::Minimum { register, a, b } => write!(f, "min {} {} {}", register, a, b),
            Arithmetic::Mod { register, a, b } => write!(f, "mod {} {} {}", register, a, b),
            Arithmetic::Multiply { register, a, b } => write!(f, "mul {} {} {}", register, a, b),
            Arithmetic::Power { register, a, b } => write!(f, "pow {} {} {}", register, a, b),
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
