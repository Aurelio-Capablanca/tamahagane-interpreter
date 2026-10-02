
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    //Values
    Number,
    True,
    False,
    String,

    //non-identified!
    NoFeature,
    AllocIdentifier,

    //keywords
    Lambda,
    Alloc,
    Fn,
    If,

    //Operators
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Caret,        // ^
    Eqs,          // =
    EqEqs,        // ==
    Lesser,       // <
    Greater,      //>
    LesserEq,     // <=
    GreaterEq,    //>=
    ReverseBool,  // !
    ReverseEq,    // !=
    And,          //&& | and
    Or,           // || | or
    Not,          // ! | not
    LambdaAssign, // =>

    // Delimitations
    LParen,    //(
    RParen,    //)
    LCBracket, //{
    RCBracket, //}
    LBracket,  //[
    RBracket,  //]
    Semicolon, //;

    //own usage at Tamahagane!
    Apostrohe, // ' -- For Base declaration
    
    //types
    
    //end for file
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_token: TokenType,
    pub lexeme: String, // actual content from Lexer
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn empty() -> Self {
        Self { type_token: TokenType::EOF, lexeme: "".to_string(), line: 0, column: 0 }
    }
}