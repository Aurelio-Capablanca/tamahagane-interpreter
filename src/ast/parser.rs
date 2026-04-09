use crate::model::expression::operators::*;
use crate::{
    ast::lexer::token::{Token, TokenType},
    model::expression::{Expression, Value},
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.position + offset)
    }

    fn advance(&mut self) -> Option<&Token> {
        let position = self.position;
        let token = self.tokens.get(position);
        self.position += 1;
        token
    }

    fn consume_elements(&mut self, expected: TokenType) -> Result<Token, String> {
        if let Some(token) = self.tokens.get(self.position) {
            if token.type_token == expected {
                self.position += 1;
                return Ok(token.clone());
            }
        }
        Err(format!("Expected {:?}", expected))
    }

    fn elements(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.tokens.get(self.position) {
            match token.type_token {
                TokenType::Number => {
                    let content = token.lexeme.clone();
                    self.advance();
                    let number: f64 = match content.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            panic!("Error parsing to Integer ! ")
                        }
                    };
                    Ok(Expression::Values(Value::Number(number, Some(10))))
                }
                TokenType::True => {
                    self.advance();
                    Ok(Expression::Values(Value::Boolean(true)))
                }
                TokenType::False => {
                    self.advance();
                    Ok(Expression::Values(Value::Boolean(false)))
                }
                TokenType::Alloc => {
                    let alloc_name = token.lexeme.clone();
                    self.advance();
                    Ok(Expression::Variable(alloc_name))
                }
                TokenType::LParen => {
                    self.advance();
                    let expression_paren = self.make_expression().unwrap();
                    self.consume_elements(TokenType::RParen).unwrap();
                    Ok(expression_paren)
                }
                TokenType::LCBracket => {
                    self.advance();
                    let expression_curl_b = self.make_expression().unwrap();
                    self.consume_elements(TokenType::RCBracket).unwrap();
                    Ok(expression_curl_b)
                }
                _ => Err(format!("Unexpected Token : {:?}", token.type_token)),
            }
        } else {
            Err("Unexpected End of block! ".to_string())
        }
    }

    fn unary(&mut self) -> Result<Expression, String> {
        if let Some(token) = self.current() {
            match token.type_token {
                TokenType::Minus => {
                    self.advance();
                    let oper = self.unary().unwrap();
                    Ok(Expression::Unary {
                        op: UOperator::Negative,
                        expr: Box::new(oper),
                    })
                }
                TokenType::Not => {
                    self.advance();
                    let oper = self.unary().unwrap();
                    Ok(Expression::Unary {
                        op: UOperator::Not,
                        expr: Box::new(oper),
                    })
                }
                _ => Err("Unary Token not find !".to_string()),
            }
        } else {
            Err("Unary Error token".to_string())
        }
    }

    fn power(&mut self) -> Result<Expression, String> {
        let mut left = self.unary().unwrap();
        if let Some(token) = self.current() {
            if token.type_token == TokenType::Caret {
                self.advance();
                let power_next = self.power().unwrap();
                left = Expression::Binary {
                    op: BOperator::Power,
                    left: Box::new(left),
                    right: Box::new(power_next),
                };
            }
        }
        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<Expression, String> {
        let mut left = self.power().unwrap();
        while let Some(token) = self.current() {
            let operator = match token.type_token {
                TokenType::Star => BOperator::Multiply,
                TokenType::Slash => BOperator::Divide,
                _ => break,
            };
            self.advance();
            let right = self.power().unwrap();
            left = Expression::Binary {
                op: operator,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Ok(left)
    }

    fn additive(&mut self) -> Result<Expression, String> {
        let mut left = self.multiplicative().unwrap();
        while let Some(token) = self.current() {
            let operator = match token.type_token {
                TokenType::Plus => BOperator::Plus,
                TokenType::Minus => BOperator::Substract,
                _ => break,
            };
            self.advance();
            let right = self.multiplicative().unwrap();
            left = Expression::Binary {
                op: operator,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
        Ok(left)
    }

    fn make_expression(&mut self) -> Result<Expression, String> {
        Ok(Expression::Values(Value::NoValue))
    }
}
