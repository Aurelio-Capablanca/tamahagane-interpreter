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

        fn slice(&self, start_point: usize, end_point: usize) -> &'a str {
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
            self.slice(start, self.pos)
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
    }
}
