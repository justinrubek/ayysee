#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}

#[derive(Debug)]
pub enum Statement {
    Alias(String, String),
    Constant(String),
}

#[derive(Debug)]
pub enum Expr {
    Value(Value),
    Identifier(Identifier),
}

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
}

#[derive(Debug)]
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
