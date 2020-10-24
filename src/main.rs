mod reducer;
mod parser;
mod lexer;
mod expression;
mod token;
mod token_type;

use reducer::reduce;
use parser::Parser;
use lexer::Lexer;
use std::io::stdin;

fn main() {
    println!("Enter an expression:");
    
    match get_input() {
        Ok(input_expression) => {
            let result = calculate(input_expression);
        
            match result {
                Ok(answer) => {
                    println!("= {:?}", answer);
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

fn calculate(expression: String) -> Result<f64, String> {
    let lexer = Lexer::new(expression);
    
    let tokens = lexer.lex()?;

    let parser = Parser::new(tokens);

    let expression_tree = parser.parse()?;

    let result = reduce(expression_tree)?;

    Ok(result)
}

fn get_input() -> Result<String, String> {
    let mut input = String::new();

    if let Err(_) = stdin().read_line(&mut input) {
        return Err("Incorrect input".to_owned())
    }

    if let Some('\n')= input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    Ok(input)
}