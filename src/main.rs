use crate::parser::tokenizer::*;

pub mod parser;

fn main() {
    println!("Hello, world!");

    let symbol = PythonCoreTokenizer::is_keyword("False", 11, 16);
    match symbol {
        Some( _ ) => println!("Found:"),
        _ => println!("Empty!")

    };
}
