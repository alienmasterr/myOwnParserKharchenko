use my_own_parser_Kharchenko::*;
use std::io::{self, Write};

fn main() {

    println!("Enter an arythmetic expression:");
    io::stdout().flush().unwrap();

    let mut expr = String::new();
    io::stdin().read_line(&mut expr).unwrap();

    match parseExpression(expr.trim()) {
        Ok(parsedExpr) => {
            println!("Result: {}", parsedExpr.result);
            println!("Operands: {:?}", parsedExpr.operands);
            println!("Operators: {:?}", parsedExpr.operators);
        }
        Err(e) => println!("Error parsing expression: {:?}", e),
    }

    
}
