// Doing a Reverse Polish Notation implementation to get back up to speed:
// Ex: 2 5 + will return 7
extern crate regex;

use std::{env};
use std::io::{self, Write};
use std::vec::Vec;
use regex::Regex;
use std::ops::Add;

#[derive(Debug, Clone, PartialEq)]
enum Symbol {
    Nil,
    Number(String),
    String(String),
    Operator(String),
}

impl Symbol {
    fn convert(input: &str) -> Symbol {
    //let num_regex = Regex::new(r"\d+").unwrap();
        if input.is_empty() {
            return Symbol::Nil;
        }

        let first = input.chars().nth(0).unwrap();

        // TODO: If it could be a number, check the whole string... as it might not be.
        match first {
            '0'..='9' => return Symbol::Number(input.to_string()),
            '+' | '-' => return Symbol::Operator(input.to_string()),
            _ => return Symbol::String(input.to_string()),
        }
    }
}

impl Add for Symbol {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return self + other;
    }
}

fn parse(input: &mut String) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = Vec::new();

    for s in input.split_whitespace() {
        symbols.push(Symbol::convert(s));
    }

    return symbols;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stack: Vec<Symbol> = Vec::new();

    loop {
        let mut input = String::new();

        print!("> ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        println!("Parsing:\n{}", input);

        let symbols = parse(&mut input);

        println!("Symbols:\n[{:?}]", symbols);

        for sym in symbols {
            stack.push(sym);
        }

        println!("Stack:\n[{:?}]", stack);

        if stack.last().unwrap() == &Symbol::Operator("+".to_string()) {
            stack.pop();

            let op1 = stack.pop().unwrap();
            let op2 = stack.pop().unwrap();
            let res = op1 + op2;

            stack.push(res);
        }

        if input.trim() == "exit" {
            println!("Exiting Interpreter");
            break;
        }
    }
}
