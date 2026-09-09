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

fn handle_carries(coll_b: &mut Vec<char>, initial: &str, p_index: usize) -> char {
    let c_or_res;
    if initial.len() == 2_usize {
        let b_pos = coll_b[p_index];
        c_or_res = initial.as_bytes()[1] as char;
        println!("carry sum : {} + {}", b_pos, c_or_res);
        let temp_next = apply_sum_rule(b_pos, c_or_res);
        coll_b[p_index + 1] = handle_carries(coll_b, temp_next, p_index + 1_usize);
        println!("after : {}", coll_b[p_index + 1]);
    } else {
        c_or_res = initial.as_bytes()[0] as char;
    }
    c_or_res
}

pub fn sum_binary_b_nums(input_a: &str, input_b: &str) -> (String /* S */, i32 /* Cout */) {
    let mut final_result: String = String::new();

    if input_a.len() > input_b.len() {
        return sum_binary_b_nums(input_b, input_a);
    }

    println!("   {}\n+ {}", input_a, input_b);

    let bit_o: Vec<char> = input_a.chars().rev().collect::<Vec<char>>();
    let mut bit_t: Vec<char> = input_b.chars().rev().collect::<Vec<char>>();

    let iteration = bit_t.len() - 1_usize;

    for i in 0..iteration {
        let val = bit_t[i];
        let one = if i < bit_o.len() - 1 {
            match bit_o.get(i) {
                Some(val) => val,
                None => {
                    eprintln!(
                        "[mode failure!] -> following bit couln't be found in maths::binary_operations::sum_binary_b_nums (returning None!)"
                    );
                    return ("0".to_string(), 0);
                }
            }
        } else {
            println!("take down num : {}", val);
            final_result.push(val);
            continue;
        };

        let current = apply_sum_rule(val, *one);
        println!("{} + {} = {}", val, one, current);
        let handler = handle_carries(&mut bit_t, current, i);
        println!(" Result {}", handler);
        final_result.push(handler);
    }
    final_result = final_result.chars().rev().collect();
    (final_result, 0)
}
