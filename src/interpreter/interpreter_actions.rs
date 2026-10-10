use crate::model::expression::Expression;


pub fn general_actions (expression : Expression)  {
    match expression {
        Expression::Binary{ op, left, right } => {
            
        }
        Expression::Lambda { params, body } => {
            
        }
        Expression::Function { params, body, domain } => {
            
        }
        Expression::Block(members) => {
            
        }
        _ => {
            println!("other arms not applied for this layer!")
        }
    }
}