use crate::parser::tokenizer::*;

use std::collections::HashMap;

pub mod parser;

fn main() {
    println!("Hello, world!");

    let symbol = PythonCoreTokenizer::is_keyword("False", 11, 16);
    match symbol {
        Some(x) => println!("Found: {}", x),
        _ => println!("Empty!")

    };
}
