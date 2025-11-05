use my_own_parser_Kharchenko::*;
use std::io::{self, Write};

fn main() {
    
    println!("Enter an arythmetic expression:");
    io::stdout().flush().unwrap();

    let mut expr = String::new();
    io::stdin().read_line(&mut expr).unwrap();

    match parse_expression(expr.trim()) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error parsing expression: {:?}", e),
    }

    
}
