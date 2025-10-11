mod scanner;
mod keyword;
mod token;
use scanner::Scanner;

fn main() {
    
    let query = String::from("CREATE TABLE Users;");

    let scanner = Scanner::new(query);

    scanner.traverse_source_code();

}
