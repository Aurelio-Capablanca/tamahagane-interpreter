use std::{collections::HashMap, sync::LazyLock};

const HEX_TO_DECIMAL: LazyLock<HashMap<String, i32>> = LazyLock::new(|| {
    HashMap::from([
        ("0".to_string(), 0),
        ("1".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
        ("A".to_string(), 10),
        ("B".to_string(), 11),
        ("C".to_string(), 12),
        ("D".to_string(), 13),
        ("E".to_string(), 14),
        ("F".to_string(), 15),
    ])
});

fn validate_goal_not_bigger_than_base(goal: &str, base: &i32) -> bool {
    for slice in goal.chars() {
        if slice == '.' {
            continue;
        }
        match slice.to_digit(10) {
            Some(digit) => {
                let val = digit as i32;
                if &val >= base {
                    return false;
                }
            }
            None => {
                false;
            }
        }
    }
    true
}

fn parse_int_to_base(int_part: &str, base: &i32) -> Option<i32> {
    let mut val: i32 = 0;
    for slice in int_part.chars() {
        let digit = slice.to_digit(10)? as i32;
        if &digit >= base {
            return None;
        }
        println!("val: {} * base: {} + digit: {}", val, base, digit);
        val = val * base + digit;
        println!("val: {}", val);
    }
    Some(val)
}

fn parse_dec_to_base(dec_part: &str, base: &i32) -> Option<f32> {
    if dec_part == "0" {
        return Some(0.0);
    }
    let mut val = 0.0;
    let mut base_frac = *base as f32;
    for slice in dec_part.chars() {
        let digit = slice.to_digit(10)? as i32;
        if &digit >= base {
            return None;
        }
        println!("val: digit: {} / base_frac: {}", digit, base_frac);
        val = (digit as f32) / base_frac;
        println!("val: {}", val);
        println!("base_frac : {} * base: {}", base_frac, base);
        base_frac *= *base as f32;
        println!("base_frac: {}", base_frac);
    }
    Some(val)
}

pub fn convert_any_base(num: &f64, base_num: &i32, base_conv: &i32, decimal_limit: &i32) -> f64 {
    if base_num == &10_i32 && base_conv == &16_i32 {
        println!("Decimal to Hex!");
        return 0.0;
    }
    let mid_point = num.to_string();
    if !validate_goal_not_bigger_than_base(mid_point.as_str(), base_num) {
        eprintln!("mismatch in base conversion");
        return 0.0;
    }
    let split: Vec<&str> = mid_point.split('.').collect::<Vec<&str>>();    
    let integer_slice: String = split.first().unwrap_or(&"0").to_string();
    let decimal_slice: String = match split.get(1) {
        Some(val) => val.to_string(),
        None => "0".to_string(),
    };
    let mut convert_int =
        parse_int_to_base(integer_slice.as_str(), base_num).unwrap_or_else(|| 0_i32);
    let mut integer_part: Vec<String> = Vec::new();
    while convert_int != 0 {
        println!("{} / {}", convert_int, base_conv);
        println!("{} % {}", convert_int, base_conv);
        let remainder = convert_int % base_conv;        
        integer_part.push(remainder.to_string());
        convert_int = convert_int / base_conv;
        println!("Result = {:?}; reminder = {:?}", convert_int, remainder);
    }
    integer_part.reverse();
    println!("{:?}",integer_part);
    let mut decimal_part: Vec<i32> = Vec::new();
    let mut convert_dec = 
        parse_dec_to_base(decimal_slice.as_str(), base_num).unwrap_or_else(|| 0_f32);
    for _ in 0..*decimal_limit {
        let frac = convert_dec * (*base_conv as f32);
        let number = frac.floor() as i32;
        decimal_part.push(number);
        convert_dec = frac - frac.floor();
        if convert_dec == 0.0 {
            break;
        }
    }
    
    0.0
}
