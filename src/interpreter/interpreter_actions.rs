use crate::{
    functions::maths::arithmetics::{power, sum},
    model::expression::{
        Expression, Value,
        operators::{BOperator, UOperator},
    },
};

pub fn general_actions(expression: &Expression) -> Value {
    match expression {
        Expression::Values(val) => val.clone(),
        //Call Binary nature operations (arithmetic related)
        Expression::Binary { op, left, right } => {            
            println!("Reach Binary operator");
            match op {
                BOperator::Plus => {
                    let a = general_actions(left);
                    let b = general_actions(right);
                    let res: Value = sum(&a, &b);
                    res
                }
                BOperator::Divide => Value::Nones,
                BOperator::Substract => Value::Nones,
                BOperator::Power => {
                    println!("action made! {:?}^{:?}", left, right);
                    let a = general_actions(left);
                    let b = general_actions(right);
                    let res = power(&a, &b);
                    res
                }
                _ => Value::Nones,
            }
        }
        Expression::Unary { op, expr } => {
            println!("Reach Unary operator");
            //Call Unary actions  (Not, Exponent)
            Value::Nones
        }
        Expression::Lambda { params, body } => {
            println!("Reach Lambda Check");
            let next = body.as_ref();
            general_actions(next);
            Value::Nones
        }
        Expression::Alloc { name, value } => {
            general_actions(value);
            Value::Nones
        }
        Expression::Variable {
            val,
            operand,
            semicolon,
        } => {
            println!("reach for Variable!");
            general_actions(val);
            Value::Nones
        }
        Expression::Function {
            params,
            body,
            domain,
        } => {
            println!("not supported yet [Function]");
            Value::Nones
        }
        Expression::Block(members) => {
            println!("Reach Block Check");
            for member in members {
                general_actions(member);
            }
            Value::Nones
        }
        _ => {
            println!("other arms not applied for this layer! {:?}", expression);
            Value::Nones
        }
    }
}
