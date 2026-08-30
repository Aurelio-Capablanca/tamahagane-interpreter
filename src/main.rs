use crate::ast::{lexer::{lex_analisys::Lexer, token::Token}, parser::Parser};

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
mod functions;

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
    /*let mut lines = String::new();
    loop {
        println!("hearing ....");        
        std::io::stdin().read_line(&mut lines).unwrap_or(0_usize);
        println!("passed in ! {:?}",lines);
        //
        lines = String::new();        
        println!("Do you want to continue? [Y/n]");
        std::io::stdin().read_line(&mut lines).unwrap_or(0_usize);
        if !lines.trim().eq_ignore_ascii_case("y") {
            break;
        }
        lines = String::new();
    }*/
    let string_start = "(&) => {78 + 7}";/*"(&) => {
        alloc reiter = 604; 
        alloc calc = 67 + 4;
        }";*///"alloc f = 0;";//"(&) => {5 + 5}";
    let mut lexer = Lexer::new(string_start);
    let tokens : Vec<Token> = lexer.tokenize();
    for token in &tokens {
        println!(
            "{:?} '{}' at {}:{}",
            token.type_token, token.lexeme, token.line, token.column
        );
    }
    let mut parser : Parser = Parser::new(tokens);
    let result = parser.parse();
    match  result {
        Ok(out) => {
            println!("Exp : {:?}",out)
        }
        Err(err) => {
            eprintln!("outcome err : {}",err)
        }
    }
}
