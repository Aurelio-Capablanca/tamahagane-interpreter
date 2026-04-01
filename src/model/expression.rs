use crate::model::{domains::domain_definition::Domain, operators::BOperator};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64, i64),
    Boolean(bool),
    Hex(String, i64),
    Symbol(String),
    Vector(Vec<Value>),
    Matrix(Vec<Vec<Value>>),    
    NoValue
}

#[derive(Debug, Clone)]
pub enum Expression {
    Values(Value),
    Binary {
        op: BOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: BOperator,
        expr: Box<Expression>,
    },
    Function{
        params: Vec<String>,
        body: Box<Expression>,
        domain: Domain
    },
    Lambda {
        params: Vec<String>,
        body: Box<Expression>
    }
}

impl Expression {
    pub fn as_numbers(&self) -> f64 {
        match self {
            Expression::Values(Value::Number(n, _)) => *n,
            Expression::Values(Value::Boolean(b)) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    pub fn get_number_or_hex_base(&self) -> &i64 {
        match self {
            Expression::Values(Value::Number(_, b)) => {
                println!("You Reached a Number!!!");
                b
            }
            Expression::Values(Value::Hex(_, b)) => {
                println!("You Reached a String!!!");
                b
            }
            _ => {
                println!("You Reached a Unknown!!!");
                &10_i64
            }
        }
    }

    pub fn to_numeric(&self) -> Expression {
        match self {
            Expression::Values(Value::Boolean(b)) => {
                Expression::Values(Value::Number(if *b { 1.0 } else { 0.0 }, 10))
            }
            _ => self.clone(),
        }
    }

    pub fn to_hex_string(&self) -> Expression {
        match self {
            Expression::Values(Value::Number(n, b)) => {
                Expression::Values(Value::Hex(n.to_string(), *b))
            }
            _ => self.clone(),
        }
    }

    pub fn _as_boolean(&self) -> bool {
        match self {
            Expression::Values(Value::Number(n, _)) => *n != 0.0,
            Expression::Values(Value::Boolean(b)) => *b,
            _ => false,
        }
    }

    pub fn to_boolean(&self) -> Expression {
        match self {
            Expression::Values(Value::Number(n, _)) => Expression::Values(Value::Boolean(*n != 0.0)),
            _ => self.clone(),
        }
    }

    pub fn get_hex(&self) -> &str {
        match self {
            Expression::Values(Value::Hex(s, _)) => s.as_str(),
            _ => "",
        }
    }
}
