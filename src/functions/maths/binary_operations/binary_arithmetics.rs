/*
knowledge article :

0 means LOW or OFF
1 means HIGH or ON

Rules:

Semi Sum circuit:

0 + 0 = 0 (result) | 0 (carry)
0 + 1 = 0 (result) | 0 (carry)
1 + 0 = 1 (result) | 0 (carry)
1 + 1 = 10 | 0 (result)| 1 (carry) (we carry one to the next operation)


Full Sum Circuit :

a   b   c     s   c

0 + 0 + 0  =  0   0   0



over boolean operation:
1+1 = 1

*/

fn semi_sum_circuit(a: char, b: char) -> (char /*res */, char /* carry */) {
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

fn full_sum_circuit(a: char, b: char, c: char) -> (char /* res */, char /* carry */) {
    if a.eq(&'0') && b.eq(&'0') && c.eq(&'0') {
        return ('0', '0');
    }
    if a.eq(&'0') && b.eq(&'0') && c.eq(&'1') {
        return ('1', '0');
    }

    if a.eq(&'0') && b.eq(&'1') && c.eq(&'0') {
        return ('1', '0');
    }
    if a.eq(&'0') && b.eq(&'1') && c.eq(&'1') {
        return ('0', '1');
    }

    if a.eq(&'1') && b.eq(&'0') && c.eq(&'0') {
        return ('1', '0');
    }
    if a.eq(&'1') && b.eq(&'0') && c.eq(&'1') {
        return ('0', '1');
    }

    if a.eq(&'1') && b.eq(&'1') && c.eq(&'0') {
        return ('0', '1');
    }
    if a.eq(&'1') && b.eq(&'1') && c.eq(&'1') {
        return ('1', '1');
    }

    ('_', '_')
}

pub fn sum_binary_b_nums(input_a: &str, input_b: &str) -> (String /* S */, i32 /* Cout */) {
    let mut final_result: String = String::new();

    if input_a.len() > input_b.len() {
        return sum_binary_b_nums(input_b, input_a);
    }

    println!(" {}\n+ {}", input_b, input_a);

    let bit_o: Vec<char> = input_a.chars().rev().collect::<Vec<char>>();
    let bit_t: Vec<char> = input_b.chars().rev().collect::<Vec<char>>();

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
            let full = semi_sum_circuit(val, carry);
            println!("{} + {} = {:?} |  carry = {}", val, carry, full.0, full.1);            
            final_result.push(full.0);
            final_result.push(full.1);
            continue;
        };        
        let full = full_sum_circuit(*one, val, carry);
        println!("{} + {} = {:?} |  carry = {}", val, one, full.0, full.1);        
        println!("______________");
        carry = full.1;
        final_result.push(full.0);
    }

    final_result = final_result.chars().rev().collect();
    (final_result, 0)
}
