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
