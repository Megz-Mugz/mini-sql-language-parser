#![allow(warnings)]

mod scanner;
mod token_category;

use crate::scanner::{Token, Tokenizer};

fn main() {
    
    let source = String::from("1 + 1");
    
    let mut tokenizer = Tokenizer::new(source);

    println!("{:?}", tokenizer.get_next_token());
    println!("{:?}", tokenizer.get_next_token());
    println!("{:?}", tokenizer.get_next_token());

    
}
