use crate::ast::lexer::token::TokenType::{Eqs, Semicolon};
use crate::model::domains::domain_definition::{self, Domain};
use crate::model::expression::operators::*;
use crate::{
    ast::lexer::token::{Token, TokenType},
    model::expression::{Expression, Value},
};

#[derive(Debug)]
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

    fn _peek(&self, offset: usize) -> Option<&Token> {
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
            println!("postition in : {:?}", token);
            if token.type_token == expected {
                self.position += 1;
                return Ok(token.clone());
            }
        }
        Err(format!("Expected {:?}", expected))
    }

    fn elements(&mut self) -> Result<Expression, String> {
        println!("reach : elements");
        if let Some(token) = self.tokens.get(self.position) {
            println!("current Token at 'elements' : {:?}", token);
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
                    self.advance();
                    let current_token = self.current();
                    let getter = current_token.unwrap();
                    let name_alloc = getter.lexeme.clone();
                    self.advance();
                    self.consume_elements(Eqs).unwrap();
                    let val: Expression = self.make_expression().unwrap();
                    while self.current().unwrap().type_token != Semicolon {
                        self.advance();
                    }
                    if self.current().unwrap().type_token == Semicolon {
                        self.consume_elements(Semicolon).unwrap();
                    }
                    Ok(Expression::Alloc {
                        name: name_alloc,
                        value: Box::new(Expression::Variable {
                            val: Box::new(val),
                            operand: BOperator::Equals,
                            semicolon: Semicolon,
                        }),
                    })
                }
                TokenType::LParen => {
                    self.advance();
                    let expression_paren = self.make_expression().unwrap();
                    self.consume_elements(TokenType::RParen).unwrap();
                    Ok(expression_paren)
                }
                TokenType::LCBracket => {
                    self.advance();
                    let mut expression_curl_b: Vec<Expression> =
                        Vec::from([self.make_expression().unwrap()]);
                    while self.current().map(|t| t.type_token).unwrap() != TokenType::RCBracket {
                        expression_curl_b.push(self.make_expression().unwrap());
                    }
                    self.consume_elements(TokenType::RCBracket).unwrap();
                    Ok(Expression::Block(expression_curl_b))
                }
                TokenType::Lambda | TokenType::LambdaAssign => {
                    self.advance();
                    let expression_lambda = self.make_expression().unwrap();
                    let funct = match expression_lambda {
                        Expression::Function { params, body, domain } => {
                            Expression::Function {
                                    params,
                                    body,
                                    domain,
                            }
                        }
                        _=> {
                            Expression::Function {
                                    params: Vec::new(),
                                    body: Box::new(expression_lambda),
                                    domain: if let Some(get) = domain_definition::DOMAIN_DIC.get(0) {
                                        get.clone()
                                    } else {
                                        Domain::empty()
                                    },
                            }
                        }
                    };                     
                    Ok(funct)
                }
                _ => Err(format!(
                    "Unexpected Token at 'elements': {:?}",
                    token.type_token
                )),
            }
        } else {
            Err("Unexpected End of block! ".to_string())
        }
    }

    fn unary(&mut self) -> Result<Expression, String> {
        println!("reach : unary");
        if let Some(token) = self.current() {
            println!("current Token at 'unary': {:?}", token);
            match token.type_token {
                TokenType::Minus => {
                    self.advance();
                    let oper = self.unary().unwrap();
                    return Ok(Expression::Unary {
                        op: UOperator::Negative,
                        expr: Box::new(oper),
                    });
                }
                TokenType::Not => {
                    self.advance();
                    let oper = self.unary().unwrap();
                    return Ok(Expression::Unary {
                        op: UOperator::Not,
                        expr: Box::new(oper),
                    });
                }
                _ => {}
            }
        }
        self.elements()
    }

    fn power(&mut self) -> Result<Expression, String> {
        println!("reach : power");
        let mut left = self.unary().unwrap();
        if let Some(token) = self.current() {
            println!("current Token at 'power': {:?}", token);
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
        println!("reach : multiplicative");
        let mut left = self.power().unwrap();
        while let Some(token) = self.current() {
            println!("current Token at 'multiplicative': {:?}", token);
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
        println!("reach : additive");
        let mut left = self.multiplicative().unwrap();
        while let Some(token) = self.current() {
            println!("current Token at 'additive': {:?}", token);
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

    fn comparisons(&mut self) -> Result<Expression, String> {
        println!("reach : comparisons");
        let mut left = self.additive().unwrap();
        while let Some(token) = self.current() {
            println!("current Token at 'comparisons': {:?}", token);
            let operator = match token.type_token {
                TokenType::EqEqs => BOperator::EqualsEquals,
                TokenType::Lesser => BOperator::Less,
                TokenType::LesserEq => BOperator::LessEqual,
                TokenType::Greater => BOperator::Greater,
                TokenType::GreaterEq => BOperator::GreaterEqual,
                _ => break,
            };
            self.advance();
            let right = self.additive().unwrap();
            left = Expression::Binary {
                op: operator,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn and_expression(&mut self) -> Result<Expression, String> {
        println!("reach : and_expression");
        let mut left = self.comparisons().unwrap();
        while let Some(token) = self.current() {
            println!("current Token at 'and_expression': {:?}", token);
            if token.type_token == TokenType::And {
                self.advance();
                let right = self.comparisons().unwrap();
                left = Expression::Binary {
                    op: BOperator::And,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn or_expresion(&mut self) -> Result<Expression, String> {
        println!("reach : or_expresion");
        let mut left = self.and_expression().unwrap();
        while let Some(token) = self.current() {
            println!("current Token at 'or_expresion': {:?}", token);
            if token.type_token == TokenType::Or {
                self.advance();
                let right = self.and_expression().unwrap();
                left = Expression::Binary {
                    op: BOperator::Or,
                    left: Box::new(left),
                    right: Box::new(right),
                }
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn make_expression(&mut self) -> Result<Expression, String> {
        self.or_expresion()
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.make_expression()
    }
}
