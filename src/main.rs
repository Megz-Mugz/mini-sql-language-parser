mod scanner;
mod token_category;

use crate::scanner::Token;

fn main() {
    
    let query = String::from("CREATE TABLE Users;");


    let mut tokens = Token::tokenize_query(&query);

    
    
}
