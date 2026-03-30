#[derive(Debug, PartialEq, Clone)]
pub enum Token {
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
    Not, // !
    
    //Values
    Number(f64),
    Boolean(bool),
    Base(i64),
    Hex(String),
    Binary(String),
    Octal(String),
    Segisdecimal(String),

    // Punctuation
    LParenthesis,
    RParenthesis,

    //conversions from decimals to binaries and vice versa
    Convert
}