/*

-- Define models 
-- redefine AST working out
-- improve console for cleaner input
-- improve module definitions
-- extend API internal behavior (stable)
 */
mod ast;
mod interpreter;
mod model;

/*
 modes:
 (1, arithmetics)
 (2, algebra) {
  (2.1 factorization)
  (2.2 algebraic operations)
  (2.3 functions)  
 }
 (3, conversion base) {
    special_char = ' (base of a number)
 }
 (4, calculus)
 (5, physics)
 (6, chemistry)
 (7, boolean expressions)
 */
 
 
fn main() {
    
    //call to init console
    let mut lines = String::new();
    loop {
        println!("hearing ....");        
        std::io::stdin().read_line(&mut lines).unwrap_or(0_usize);
        println!("passed in ! {:?}",lines);
        lines = String::new();        
        println!("Do you want to continue? [Y/n]");
        std::io::stdin().read_line(&mut lines).unwrap_or(0_usize);
        if !lines.trim().eq_ignore_ascii_case("y") {
            break;
        }
        lines = String::new();
    }
}
