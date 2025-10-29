
use crate::token_category::TokenCategory;

pub struct Token {
    category: TokenCategory,
    lexeme: String,
}


impl Token {
    
    // Constructor for Token
    fn new(category: TokenCategory, lexeme: String) -> Self {
        Token { category, lexeme }
    }

    // Actually tokenizes the input query and outputs a vector
    // of tokens <Category, Lexeme>
    
    pub fn tokenize_query(query: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let current_word = String::new();
        
        for letter in query.chars() {
            if (letter.is_whitespace()) {continue;}

            println!("Current letter: {}", letter);
        }

        tokens
    }

}

