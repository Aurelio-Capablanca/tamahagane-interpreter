use crate::model::expression::Value::{self, Integer};

pub fn sum(a: &Value, b: &Value) -> Value {
    match a {
        Value::Integer(val, bs) => {
            let s = match b.get_integer() {
                Some(s) => s,
               None => {
                   println!("{:?}",b);
                   panic!("Impossible to sum at NULL value back!") 
               }
            }; 
            println!("{} + {}",val,s);
            let res = val + s;
            Value::Integer(res, Some(10))            
        }
        _ => {
            panic!("No Known type submitted to Eval!")
        }
    }
}
