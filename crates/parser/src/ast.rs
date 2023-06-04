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
    Constant(String),
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

    pub fn new_constant(value: String) -> Self {
        Self::Constant(value)
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
}

#[derive(Clone, Debug)]
pub enum Expr {
    Constant(Value),
    Identifier(Identifier),
    BinaryOp(Box<Expr>, BinaryOpcode, Box<Expr>),
    UnaryOp(UnaryOpcode, Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOpcode {
    Add,
    Sub,
    Mul,
    Div,
    Conj,
    Disj,
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
}

#[derive(Clone, Debug)]
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

impl ToString for Identifier {
    fn to_string(&self) -> String {
        self.0.clone()
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
        // Self::Statements(statements)
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
        /// The device to read from
        device: Identifier,
        /// The attribute to read from the device
        device_variable: Identifier,
        /// The local variable to store the read value
        local: Identifier,
    },
    Write {
        /// The value to write to the device
        value: Box<Expr>,
        /// The device to write to
        device: Identifier,
        /// The attribute to write to the device
        device_variable: Identifier,
    },
}

impl DeviceStatement {
    pub fn new_read(device: Identifier, device_variable: Identifier, local: Identifier) -> Self {
        Self::Read {
            device,
            device_variable,
            local,
        }
    }

    pub fn new_write(value: Box<Expr>, device: Identifier, device_variable: Identifier) -> Self {
        Self::Write {
            value,
            device,
            device_variable,
        }
    }
}
