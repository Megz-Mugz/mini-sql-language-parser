/*
The purpose of this file is to read the source code
and send each created token over to the parser

The scanner will enforce the rules at the word level
*/

use crate::{keyword, token};
use crate::token::{Token, TokenType};

pub struct Scanner {
    source_code: String,
}

impl Scanner {
    // constructor for the scanner
    pub fn new(source_code: String) -> Scanner {
        Scanner { source_code }
    }

    pub fn traverse_source_code(self) {
        let mut current_word = String::new();

        for source_code_char in self.source_code.chars() {
            if source_code_char.is_whitespace() || source_code_char == ';' {
                // use pattern matching for keyword detection
                match current_word.as_str() {
                    keyword::CREATE => self.create_token(TokenType::Create, &current_word),
                    keyword::INSERT => self.create_token(TokenType::Insert, &current_word),
                    keyword::UPDATE => self.create_token(TokenType::Update, &current_word),
                    keyword::DELETE => self.create_token(TokenType::Delete, &current_word),
                    keyword::TABLE => self.create_token(TokenType::Table, &current_word),
                    _ => self.create_token(TokenType::Identifier, &current_word),
                }

                current_word.clear();
            } else {
                current_word.push(source_code_char.to_ascii_lowercase());
            }
        }
    }

    fn create_token(&self, token_type: TokenType, word: &str) {
        println!("TokenType: {:?} Word: {}", token_type, word);
    }
}