use crate::parser::tokenizer::*;

pub mod parser;

fn main() {
    println!("Hello, world!");

    let lexer : PythonCoreTokenizer = PythonCoreTokenizer::new( String::from("Hello, World!"), 4 );
    let symbol = lexer.is_keyword("False", 11, 16);
    match symbol {
        Some( _ ) => println!("Found:"),
        _ => println!("Empty!")

    };
}
