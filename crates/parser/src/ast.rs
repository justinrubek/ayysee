#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Assignment {
        identifier: Identifier,
        expression: Box<Expr>,
    },
    Definition {
        identifier: Identifier,
        expression: Box<Expr>,
    },
    Alias {
        /// The identifier to alias to
        identifier: Identifier,
        /// The new alias to the identifier
        alias: Identifier,
    },
    /// Defines a constant value for use in expressions
    Constant(Identifier, Value),
    Function {
        identifier: Identifier,
        parameters: Vec<Identifier>,
        body: Block,
    },
    FunctionCall {
        identifier: Identifier,
        arguments: Vec<Box<Expr>>,
    },
    Block(Block),
    Loop {
        body: Block,
    },
    IfStatement(IfStatement),
    DeviceStatement(DeviceStatement),
    Yield,
    Break,
    Sleep {
        duration: Box<Expr>,
    },
}

impl Statement {
    pub fn new_assignment(identifier: Identifier, expression: Box<Expr>) -> Self {
        Self::Assignment {
            identifier,
            expression,
        }
    }

    pub fn new_definition(identifier: Identifier, expression: Box<Expr>) -> Self {
        Self::Definition {
            identifier,
            expression,
        }
    }

    pub fn new_alias(identifier: Identifier, alias: Identifier) -> Self {
        Self::Alias { identifier, alias }
    }

    pub fn new_constant(identifier: Identifier, value: Value) -> Self {
        Self::Constant(identifier, value)
    }

    pub fn new_function(identifier: Identifier, parameters: Vec<Identifier>, body: Block) -> Self {
        Self::Function {
            identifier,
            parameters,
            body,
        }
    }

    pub fn new_function_call(identifier: Identifier, arguments: Vec<Box<Expr>>) -> Self {
        Self::FunctionCall {
            identifier,
            arguments,
        }
    }

    pub fn new_block(block: Block) -> Self {
        Self::Block(block)
    }

    pub fn new_loop(body: Block) -> Self {
        Self::Loop { body }
    }

    pub fn new_if(if_statement: IfStatement) -> Self {
        Self::IfStatement(if_statement)
    }

    pub fn new_device(statement: DeviceStatement) -> Self {
        Self::DeviceStatement(statement)
    }

    pub fn new_yield() -> Self {
        Self::Yield
    }

    pub fn new_break() -> Self {
        Self::Break
    }

    pub fn new_sleep(duration: Box<Expr>) -> Self {
        Self::Sleep { duration }
    }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Constant(Value),
    Identifier(Identifier),
    BinaryOp(Box<Expr>, BinaryOpcode, Box<Expr>),
    UnaryOp(UnaryOpcode, Box<Expr>),
    /// Built-in function call (abs, sqrt, min, max, etc.)
    Call(Identifier, Vec<Box<Expr>>),
    /// Ternary expression: condition ? true_val : false_val
    /// Maps directly to the MIPS `select` instruction.
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Conj,
    Disj,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Lower,
    LowerEquals,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOpcode {
    Not,
    BitNot,
}

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Identifier(String);

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<Identifier> for String {
    fn from(id: Identifier) -> Self {
        id.0
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<String> for Identifier {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub enum Block {
    Statements(Vec<Statement>),
}

impl Block {
    pub fn new_statements(statements: Option<Vec<Statement>>) -> Self {
        match statements {
            Some(statements) => Self::Statements(statements),
            None => Self::Statements(vec![]),
        }
    }
}

#[derive(Clone, Debug)]
pub enum IfStatement {
    If {
        condition: Box<Expr>,
        body: Block,
    },
    IfElse {
        condition: Box<Expr>,
        body: Block,
        else_body: Block,
    },
}

impl IfStatement {
    pub fn new_if(condition: Box<Expr>, body: Block) -> Self {
        Self::If { condition, body }
    }

    pub fn new_if_else(condition: Box<Expr>, body: Block, else_body: Block) -> Self {
        Self::IfElse {
            condition,
            body,
            else_body,
        }
    }
}

/// A statement that interacts with a device
#[derive(Clone, Debug)]
pub enum DeviceStatement {
    Read {
        device: Box<Expr>,
        device_variable: Identifier,
        local: Identifier,
    },
    Write {
        value: Box<Expr>,
        device: Box<Expr>,
        device_variable: Identifier,
    },
    /// Batch read from all network devices matching a type hash.
    /// `batch read HASH.Variable mode into local;`
    BatchRead {
        hash: Box<Expr>,
        device_variable: Identifier,
        mode: Identifier,
        local: Identifier,
    },
    /// Batch write to all network devices matching a type hash.
    /// `batch write expr into HASH.Variable;`
    BatchWrite {
        value: Box<Expr>,
        hash: Box<Expr>,
        device_variable: Identifier,
    },
    /// Slot read: `slot read Device[N].SlotVar into local;`
    SlotRead {
        device: Box<Expr>,
        slot: Box<Expr>,
        slot_variable: Identifier,
        local: Identifier,
    },
    /// Slot write: `slot write expr into Device[N].SlotVar;`
    SlotWrite {
        value: Box<Expr>,
        device: Box<Expr>,
        slot: Box<Expr>,
        slot_variable: Identifier,
    },
}

impl DeviceStatement {
    pub fn new_read(device: Box<Expr>, device_variable: Identifier, local: Identifier) -> Self {
        Self::Read {
            device,
            device_variable,
            local,
        }
    }

    pub fn new_write(value: Box<Expr>, device: Box<Expr>, device_variable: Identifier) -> Self {
        Self::Write {
            value,
            device,
            device_variable,
        }
    }

    pub fn new_batch_read(
        hash: Box<Expr>,
        device_variable: Identifier,
        mode: Identifier,
        local: Identifier,
    ) -> Self {
        Self::BatchRead {
            hash,
            device_variable,
            mode,
            local,
        }
    }

    pub fn new_batch_write(value: Box<Expr>, hash: Box<Expr>, device_variable: Identifier) -> Self {
        Self::BatchWrite {
            value,
            hash,
            device_variable,
        }
    }

    pub fn new_slot_read(
        device: Box<Expr>,
        slot: Box<Expr>,
        slot_variable: Identifier,
        local: Identifier,
    ) -> Self {
        Self::SlotRead {
            device,
            slot,
            slot_variable,
            local,
        }
    }

    pub fn new_slot_write(
        value: Box<Expr>,
        device: Box<Expr>,
        slot: Box<Expr>,
        slot_variable: Identifier,
    ) -> Self {
        Self::SlotWrite {
            value,
            device,
            slot,
            slot_variable,
        }
    }
}
