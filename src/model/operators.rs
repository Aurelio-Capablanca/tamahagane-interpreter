#[derive(Debug, PartialEq, Clone)]
pub enum BOperator {
    // Arithmetic
    Plus,
    Minus,
    Multiply,
    Divide,

    // Comparison
    Equals,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Logical
    And, // &&
    Or,  // ||
        
    //Values
    Number(f64),
    Boolean(bool),
    Base(i64),
    Hex(String),

    // Punctuation
    LParenthesis,
    RParenthesis,

    //conversions from decimals to binaries and vice versa
    Convert// ' at 2 mode
}

pub enum UOperator{
    Not, // !
    Power, //
    Radix,
    Factorial,    
}