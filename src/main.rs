use arythemetic_expressions_parser_Kharchenko::*;
use std::io::{self, Write, BufRead};
use std::fs::File;

fn main() {

    println!("Welcome to Arythmetic Expression Parser CLI!");
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    if args[1] == "help" {
        print_help();
    } else if args[1] == "credits" {
        print_credits();
    } else if args[1] == "parse" {
        if args.len() < 3 {
            println!("provide a name of the file to parse");
            return;
        }
        parse_file(&args[2]);
    } else {
        println!("unknown enter: {}", args[1]);
        print_help();
    }
 
}

fn print_help() {
    println!("info:");
    println!("parse    - parse expressions from a file");
    println!("help     - show this help message");
    println!("credits  - show credits");
}

fn print_credits() {
    println!("Arythmetic Expression Parser by Alina Kharchenko for Rust course project");
    println!("CLI version");
}

fn parse_file(filename: &str) {
    let file = File::open(filename);
    if file.is_err() {
        println!("Could not open file: {}", filename);
        return;
    }

    let reader = io::BufReader::new(file.unwrap());
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        match parseExpression(line.trim()) {
            Ok(res) => println!("Line {}: {} = {}", i + 1, line, res.result),
            Err(e) => println!("Line {}: {} -> Error: {:?}", i + 1, line, e),
        }
    }
}