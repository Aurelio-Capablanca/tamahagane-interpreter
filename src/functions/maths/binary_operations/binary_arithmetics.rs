/*
knowledge article :

0 means LOW or OFF
1 means HIGH or ON

Rules:

0 + 0 = 0 (result) | 0 (carry)
0 + 1 = 0 (result) | 0 (carry)
1 + 0 = 1 (result) | 0 (carry)
1 + 1 = 10 | 0 (result)| 1 (carry) (we carry one to the next operation)


over boolean operation:
1+1 = 1

*/


fn apply_sum_rule(a: char, b: char) -> &'static str {
    if a.eq(&'0') && b.eq(&'0') {
        return "0";
    }
    if a.eq(&'0') && b.eq(&'1') {
        return "0";
    }
    if a.eq(&'1') && b.eq(&'0') {
        return "1";
    }
    if a.eq(&'1') && b.eq(&'1') {
        return "10";
    }
    "_"
}

pub fn sum_binary_b_nums(input_a: &str, input_b: &str) -> (String /* S */, i32 /* Cout */) {
    let mut final_result : String = String::new();
    let carry : &str = "";
    
    if input_a.len() > input_b.len() {
        return sum_binary_b_nums(input_b, input_a);
    }
    let bit_o : Vec<char> = input_a.chars().rev().collect::<Vec<char>>();
    let mut bit_t : Vec<char> = input_b.chars().rev().collect::<Vec<char>>();
    
    let mut carry = '0';
    for (i, val) in bit_t.iter().enumerate() {
        
        let one = if i < bit_o.len() {
            match bit_o.get(i) {
                Some(val) => val,
                None => {
                    eprintln!("[mode failure!] -> following bit couln't be found in maths::binary_operations::sum_binary_b_nums (returning None!)");
                    return ("0".to_string(), 0);
                }
            }
        } else {
            val
        };
        let current_eval = apply_sum_rule(*one, *val);
        if current_eval.len() > 2_usize {
            let interm = current_eval.as_bytes()[1];
            carry = interm as char;
            bit_t[i + 1_usize ] = apply_sum_rule(carry, bit_t[i + 1_usize]);
        }
        
        final_result.push(*one);
    }
    
    (final_result, 0)
}
