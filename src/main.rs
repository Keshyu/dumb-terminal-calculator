mod parser;
mod lexer;
mod expression;
mod token;
mod token_type;

use parser::Parser;
use lexer::Lexer;

fn main() {
    let source = "23.345 + .454".to_string();

    let lexer = Lexer::new(source);

    match lexer.lex() {
        Ok(tokens) => {
            println!("{:?}\n", tokens);

            let parser = Parser::new(tokens);
            
            match parser.parse() {
                Ok(expression) => {
                    println!("{:?}", expression);
                }
                Err(message) => {
                    println!("ERROR");
                    println!("{}\n", message);
                }
            }
        }
        Err(message) => {
            println!("ERROR");
            println!("{}\n", message);
        }
    }
}
