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

fn handle_carries(
    coll_b: &mut Vec<char>,
    initial: &str,
    p_index: usize,
) -> (char /*carry */, char /*res */) /* carry , char  res )*/
{
    let mut res;
    let mut carry = '0';
    if initial.len() == 2_usize {
        if (p_index + 1_usize) >= coll_b.len() {
            return (initial.as_bytes()[0] as char, '0');
        }
        let b_pos = coll_b[p_index + 1_usize];
        carry = initial.as_bytes()[0] as char;
        res = initial.as_bytes()[1] as char;
        let temp_next = apply_sum_rule(carry, b_pos);
        println!(
            "\nb = {}\ncarry = {} \ncarry sum : {} + {} = {}",
            b_pos, carry, carry, b_pos, temp_next
        );
        
        let (pos_z, pos_o) = if temp_next.len() == 2_usize {
            // let eval = ;
            // if eval.0.eq(&'1') && carry.eq(&'1') && b_pos.eq(&'1'){
            //     res = eval.0;
            // }
            // eval
            (temp_next.as_bytes()[0] as char, temp_next.as_bytes()[1] as char)
        } else {
            (temp_next.as_bytes()[0] as char, '0')
        };
        
        //let middle = handle_carries(coll_b, temp_next, p_index + 1_usize);
        coll_b[p_index + 1] = pos_z;
        //return middle_men;
    } else {
        res = initial.as_bytes()[0] as char;
        //return ('0', res);
    }
    (carry, res)
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
            // let finale = apply_sum_rule(val, carry);
            // final_result.push_str(finale);
            final_result.push(val);
            final_result.push(carry);
            continue;
        };

        let current = apply_sum_rule(val, *one);
        println!("{} + {} = {}", val, one, current);
        let handler = handle_carries(&mut bit_t, current, i);
        println!(
            "result : {} | carry : {}\n______________",
            handler.1, handler.0
        );
        carry = handler.0;
        final_result.push(handler.1);
    }

    final_result = final_result.chars().rev().collect();
    (final_result, 0)
}
