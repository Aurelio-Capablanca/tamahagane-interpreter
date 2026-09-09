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

fn apply_sum_rule(a: char, b: char) -> (char /*res */, char /* carry */) {
    if a.eq(&'0') && b.eq(&'0') {
        return ('0', '0');
    }
    if a.eq(&'0') && b.eq(&'1') {
        return ('1', '0');
    }
    if a.eq(&'1') && b.eq(&'0') {
        return ('1', '0');
    }
    if a.eq(&'1') && b.eq(&'1') {
        return ('0', '1');
    }
    ('_', '_')
}

fn handle_carries(
    coll_b: &mut Vec<char>,
    res: &char,
    carry: &char,
    p_index: usize,
) -> (char /*res */, char /*carry */) {
    if res.eq(&'0') && carry.eq(&'1') {
        if (p_index + 1_usize) >= coll_b.len() {
            return (*res, *carry);
        }
        let b_pos = coll_b[p_index + 1_usize];
        let temp_next = apply_sum_rule(*carry, b_pos);
        println!(
            "values :\nb_pos = {} \ncarry = {} \nnext_val = {} \nnext_carry = {}",
            b_pos, carry, temp_next.0, temp_next.1
        );
        println!("Sum : {} + {} = {:?}", *carry, b_pos, temp_next);
        let next_res = if b_pos.eq(&'1') && temp_next.1.eq(&'1') && carry.eq(&'1'){
            println!("all condition met!");
            temp_next.1
        } else {
            temp_next.0
        };
        coll_b[p_index + 1] = next_res;//temp_next.0; 
        println!("next coll_b value : {}", coll_b[p_index + 1]);
    }
    (*res, *carry)
}

pub fn sum_binary_b_nums(input_a: &str, input_b: &str) -> (String /* S */, i32 /* Cout */) {
    let mut final_result: String = String::new();

    if input_a.len() > input_b.len() {
        return sum_binary_b_nums(input_b, input_a);
    }

    println!(" {}\n+ {}", input_b, input_a);

    let bit_o: Vec<char> = input_a.chars().rev().collect::<Vec<char>>();
    let mut bit_t: Vec<char> = input_b.chars().rev().collect::<Vec<char>>();

    let mut carry = '0';
    for i in 0..bit_t.len() {
        println!("at : {} \nbit_t : {:?} \nbit_o : {:?}", i, bit_t, bit_o);
        let val = bit_t[i];
        let one = if i < bit_o.len() {
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
            // final_result.push(val);
            // final_result.push(carry);
            continue;
        };

        let current = apply_sum_rule(val, *one);
        println!("{} + {} = {:?} |  carry = {}", val, one, current.0, current.1);
        let handler = handle_carries(&mut bit_t, &current.0, &current.1, i);
        println!("______________");
        carry = handler.1;        
        final_result.push(handler.0);
    }

    final_result = final_result.chars().rev().collect();
    (final_result, 0)
}
