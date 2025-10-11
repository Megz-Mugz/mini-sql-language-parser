use std::fmt;

#[derive(Debug)] // <-- add this line!
pub enum TokenType {
    Create,
    Insert,
    Update,
    Delete,
    Table, 
    Identifier
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub word: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, word: &'a str) -> Token<'a> {
        Token { token_type, word }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({:?}, '{}')", self.token_type, self.word)
    }
}