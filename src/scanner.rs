/*

The purpose of this file is to read the source code
and send each created token over to the parser

The scanner will enforce the rules at the word level

*/





pub struct Scanner {
    source_code: String
}

impl Scanner {

    // constructor for the scanner
    pub fn new(source_code: String) -> Scanner{
        
        Scanner {
            source_code: source_code
        }
    }

    pub fn traverse_source_code(self) {


        let mut current_word = String::new();

        for source_code_char in self.source_code.chars() {
            
            if source_code_char.is_whitespace() || source_code_char == ';'{
                
                // use pattern matching for keyword detection
                match current_word.as_str() {
                    "create" => println!("{} - is CREATE!", current_word),
                    "insert" => println!("{} - is INSERT!", current_word),
                    "update" => println!("{} - is UPDATE!", current_word),
                    "delete" => println!("{} - is DELETE!", current_word),
                    "table"  => println!("{} - is TABLE!", current_word),
                    _ => println!("{} - probably an identifier", current_word),
                }

                current_word.clear();

            } else {
                current_word.push(source_code_char.to_ascii_lowercase());   
            }
        }
    }


}
