use chrono::{DateTime, FixedOffset};
/// Defines Excel Functions.
#[derive(Debug, Copy, Clone)]
pub enum Function {
    Abs,
    Sum,
    Product,
    Average,
    Or,
    And,
    Xor,
    Not,
    Negate,
    Days,
    Right,
    Left,
    Iff,
    IsBlank,
}

/// Defines Excel Operators.
#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Concat,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    Function(Function),
}

/// Defines error types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    Div0,
    Cast,
    Parse,
    Value,
    Argument,
    Reference,
}

/// Defines boolean types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Boolean {
    True,
    False,
}

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f32),
    Text(String),
    Boolean(Boolean),
    Iterator(Vec<Value>),
    Error(Error),
    Date(DateTime<FixedOffset>),
    Blank,
}

/// Defines each term in Expression Struct.
#[derive(Debug, Clone)]
pub enum Formula {
    Operation(Expression),
    Value(Value),
    Reference(String),
    RangeReference(String),
    Iterator(Vec<Formula>),
}

/// Struct that holds a parsed string. Formula enum and Expression Struct are defined recursively.
#[derive(Debug, Clone)]
pub struct Expression {
    pub op: Operator,
    pub values: Vec<Formula>,
}

/// Enum định nghĩa Reference
#[derive(Debug, Clone)]
pub enum Reference {
    CellReference(String),
    RangeReference(String),
}