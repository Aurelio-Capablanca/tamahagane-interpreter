use crate::model::expression::Value;

pub fn sum(a: &Value, b: &Value) -> Value {
    match a {
        Value::Integer(val, _bs) => {
            let s = match b.get_integer() {
                Some(s) => s,
                None => {
                    println!("{:?}", b);
                    panic!("Impossible to sum at NULL value back!")
                }
            };
            let res = val + s;
            println!("{} + {} = {}", val, s, res);
            Value::Integer(res, Some(10))
        }
        _ => {
            panic!("No Known type submitted to Eval!")
        }
    }
}

pub fn power(a: &Value, b: &Value) -> Value {
    match a {
        Value::Integer(val, _bs) => {
            let mid = match b.get_integer() {
                Some(val) => val,
                None => {
                    println!("{:?}", b);
                    panic!("Impossible to get value from NULL !")
                }
            };
            let res = val.pow(mid as u32);
            println!("{} ^ {} = {}", val, mid, res);
            Value::Integer(res, Some(10))
        }
        _ => panic!("No Known Type submitted to Eval!"),
    }
}
