
use core::num;

use crate::token_category::TokenCategory;

#[derive(Debug)]
pub struct Token {
    category: TokenCategory,
    lexeme: String,
}


impl Token {
    
    // Constructor for Token
    fn new(category: TokenCategory, lexeme: String) -> Self {
        Token { category, lexeme }
    }

}


pub struct Tokenizer{
    cursor: usize,
    source: String,
    len_of_source: usize,
}
impl Tokenizer {

    pub fn new(source: String) -> Self {
        let len_of_source = source.len();
        Tokenizer { cursor: 0, source, len_of_source}
    }

    fn move_cursor(&mut self) {
        self.cursor += 1;
    }

    
    // Actually tokenizes the query
pub fn get_next_token(&mut self) -> Result<Token, String> {
    if self.cursor >= self.len_of_source {
        return Err("End of source".to_string());
    }

    let current_letter = self.source.as_bytes()[self.cursor] as char;

    let token = match current_letter {
        '+' => {
            self.move_cursor();
            Token::new(TokenCategory::AdditionOperator, current_letter.to_string())
        }, 
        '-' => {
            self.move_cursor();
            Token::new(TokenCategory::SubtractionOperator, current_letter.to_string())
        }, 
        '*' => {
            self.move_cursor();
            Token::new(TokenCategory::MultiplicationOperator, current_letter.to_string())
        }, 
        '/' => {
            self.move_cursor();
            Token::new(TokenCategory::DivisionOperator, current_letter.to_string())
        },
        '0'..='9' => {
            self.scan_integer()
        },  
        _ => {
            return Err(format!("Unexpected character '{}'", current_letter));
        }, 
    };

    Ok(token)
}


    fn scan_integer(&mut self) -> Token {
        let mut number = String::new();

        while self.cursor < self.len_of_source {
            let ch = self.source.as_bytes()[self.cursor] as char;

            if !ch.is_numeric() {
                break;
            } 

            number.push(ch);
            self.move_cursor();
        }

        Token::new(TokenCategory::Integer, number)
    }

}

