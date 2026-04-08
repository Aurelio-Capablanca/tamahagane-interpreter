pub mod token {

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

        //Operators
        Plus,        // +
        Minus,       // -
        Star,        // *
        Slash,       // /
        Caret,       // ^
        Eqs,         // =
        EqEqs,       // ==
        Lesser,      // <
        Greater,     //>
        LesserEq,    // <=
        GreaterEq,   //>=
        ReverseBool, // !
        ReverseEq,   // !=
        And,         //&& | and
        Or,          // || | or
        Not,         // ! | not
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
        //end for file
        EOF,
    }

    pub struct Token {
        pub type_token: TokenType,
        pub lexeme: String, // actual content from Lexer
        pub line: usize,
        pub column: usize,
    }
}

pub mod lex_analisys {
    use std::thread::current;

    use crate::ast::lexer::token::{Token, TokenType};

    pub struct Lexer<'a> {
        input: &'a [u8],
        pos: usize,
        line: usize,
        column: usize,
    }

    impl<'a> Lexer<'a> {
        pub fn new(new_input: &'a str) -> Self {
            Self {
                input: new_input.as_bytes(),
                pos: 0_usize,
                line: 1_usize,
                column: 0_usize,
            }
        }

        fn current(&self) -> Option<u8> {
            self.input.get(self.pos).copied()
        }

        fn peek(&self, offset: usize) -> Option<u8> {
            self.input.get(self.pos + offset).copied()
        }

        fn advance(&mut self) -> Option<u8> {
            let current = self.current().unwrap();
            self.pos += 1;

            if current == b'\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }

            Some(current)
        }

        fn advance_by(&mut self, n: usize) {
            self.pos += n.min(self.input.iter().len() - self.pos);
        }

        fn get_slice(&self, start_point: usize, end_point: usize) -> &'a str {
            unsafe { std::str::from_utf8_unchecked(&self.input[start_point..end_point]) }
        }

        pub fn consume_bites<F>(&mut self, pred: F) -> &'a str
        where
            F: Fn(u8) -> bool,
        {
            let start = self.pos;
            while let Some(byte) = self.current() {
                if pred(byte) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.get_slice(start, self.pos)
        }

        pub fn skip_non_valid(&mut self) {
            while let Some(byte) = self.current() {
                if (byte as char).is_whitespace()
                    || (byte as char).is_ascii_whitespace()
                    || byte.eq_ignore_ascii_case(&b'\\')
                {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // state keywords
        fn is_keyword(sub: &str) -> Option<TokenType> {
            match sub {
                "alloc" | "allc" => Some(TokenType::Alloc),
                "fn" => Some(TokenType::Fn),
                "lbmd" | "(&)" => Some(TokenType::Lambda),
                "true" | "True" => Some(TokenType::True),
                "false" | "False" => Some(TokenType::False),
                "and" => Some(TokenType::And),
                "not" => Some(TokenType::Not),
                "or" => Some(TokenType::Or),
                _ => None,
            }
        }

        //-------- peek at tokens --------
        pub fn peek_tokens(&mut self) -> Token {
            self.skip_non_valid();

            let line = self.line;
            let column = self.column;

            match self.current() {
                None => Token {
                    type_token: TokenType::EOF,
                    lexeme: String::new(),
                    line: line,
                    column: column,
                },
                Some(b'(') => {
                    self.advance();
                    if self.current() == Some(b'&') {
                        self.advance();
                        if self.current() == Some(b')') {
                            self.advance();
                            Token {
                                type_token: TokenType::Lambda,
                                lexeme: "(&)".to_string(),
                                line,
                                column,
                            }
                        } else {
                            Token {
                                type_token: TokenType::LParen,
                                lexeme: "(".to_string(),
                                line,
                                column,
                            }
                        }
                    } else {
                        Token {
                            type_token: TokenType::LParen,
                            lexeme: "(".to_string(),
                            line,
                            column,
                        }
                    }
                }
                Some(b')') => {
                    self.advance();
                    Token {
                        type_token: TokenType::RParen,
                        lexeme: ")".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'}') => {
                    self.advance();
                    Token {
                        type_token: TokenType::RCBracket,
                        lexeme: "}".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'{') => {
                    self.advance();
                    Token {
                        type_token: TokenType::LCBracket,
                        lexeme: "{".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'[') => {
                    self.advance();
                    Token {
                        type_token: TokenType::LBracket,
                        lexeme: "[".to_string(),
                        line,
                        column,
                    }
                }
                Some(b']') => {
                    self.advance();
                    Token {
                        type_token: TokenType::RBracket,
                        lexeme: "]".to_string(),
                        line,
                        column,
                    }
                }
                Some(b';') => {
                    self.advance();
                    Token {
                        type_token: TokenType::Semicolon,
                        lexeme: ";".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'+') => {
                    self.advance();
                    Token {
                        type_token: TokenType::Plus,
                        lexeme: "+".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'-') => {
                    self.advance();
                    Token {
                        type_token: TokenType::Minus,
                        lexeme: "-".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'*') => {
                    self.advance();
                    Token {
                        type_token: TokenType::Star,
                        lexeme: "*".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'^') => {
                    self.advance();
                    Token {
                        type_token: TokenType::Caret,
                        lexeme: "^".to_string(),
                        line,
                        column,
                    }
                }
                Some(b'=') => {
                    self.advance();
                    if self.current() == Some(b'=') {
                        Token {
                            type_token: TokenType::EqEqs,
                            lexeme: "==".to_string(),
                            line,
                            column,
                        }
                    } else if self.current() == Some(b'>') {
                        self.advance();
                        Token {
                            type_token: TokenType::LambdaAssign,
                            lexeme: "=>".to_string(),
                            line,
                            column,
                        }
                    } else {
                        Token {
                            type_token: TokenType::Eqs,
                            lexeme: "=".to_string(),
                            line,
                            column,
                        }
                    }
                }
                Some(b'<') => {
                    self.advance();
                    if self.current() == Some(b'=') {
                        Token {
                            type_token: TokenType::LesserEq,
                            lexeme: "<=".to_string(),
                            line,
                            column,
                        }
                    } else {
                        Token {
                            type_token: TokenType::Lesser,
                            lexeme: "<".to_string(),
                            line,
                            column,
                        }
                    }
                }
                Some(b'>') => {
                    self.advance();
                    if self.current() == Some(b'=') {
                        Token {
                            type_token: TokenType::GreaterEq,
                            lexeme: ">=".to_string(),
                            line,
                            column,
                        }
                    } else {
                        Token {
                            type_token: TokenType::Greater,
                            lexeme: ">".to_string(),
                            line,
                            column,
                        }
                    }
                }
                Some(b'!') => {
                    self.advance();
                    if self.current() == Some(b'=') {
                        Token {
                            type_token: TokenType::ReverseEq,
                            lexeme: "!=".to_string(),
                            line,
                            column,
                        }
                    } else {
                        Token {
                            type_token: TokenType::ReverseBool,
                            lexeme: "!".to_string(),
                            line,
                            column,
                        }
                    }
                }
                // Some(byte) => { // For Lambda Declaration
                //     Token { type_token: (), lexeme: (), line, column }
                // }
                Some(byte) if byte.is_ascii_alphabetic() || byte == b'_' => {
                    // keywords and identifiers
                    let lexeme =
                        self.consume_bites(|pred| pred.is_ascii_alphabetic() || pred == b'_');

                    let token_type = if let Some(keyword) = Self::is_keyword(lexeme) {
                        keyword
                    } else {
                        TokenType::AllocIdentifier
                    };

                    Token {
                        type_token: token_type,
                        lexeme: lexeme.to_string(),
                        line,
                        column,
                    }
                }
                Some(byte) if byte.is_ascii_digit() => {
                    let lexeme = self.consume_bites(|pred| pred.is_ascii_digit() || pred == b'.');
                    Token {
                        type_token: TokenType::Number,
                        lexeme: lexeme.to_string(),
                        line,
                        column,
                    }
                }
                Some(byte) => {
                    self.advance();
                    Token {
                        type_token: TokenType::NoFeature,
                        lexeme: format!("UNKNOWN : {}", byte as char),
                        line,
                        column,
                    }
                }
            }
        }
        // follow up!
        pub fn tokenize(&mut self) -> Vec<Token> {
            let mut tokens: Vec<Token> = Vec::new();

            loop {
                let token = self.peek_tokens();
                if token.type_token == TokenType::EOF {
                    tokens.push(token);
                    break;
                }
                tokens.push(token);
            }

            tokens
        }
    }
}
