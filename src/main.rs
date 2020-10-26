mod reducer;
mod parser;
mod lexer;
mod expression;
mod fraction;
mod token;
mod token_type;
mod sign;

use reducer::reduce;
use fraction::Fraction;
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
                    if answer.denumerator() != 1 {
                        println!("= {}", answer);
                    }
                    println!("= {}", decimilize(&answer));
                }
                Err(message) => {
                    println!("\nERROR");
                    println!("{}\n", message);
                }
            }
        }
        Err(message) => {
            println!("\nERROR");
            println!("{}\n", message);
        }
    }
}

fn calculate(expression: String) -> Result<Fraction, String> {
    let tokens = Lexer::new(expression).lex()?;
    let expression_tree = Parser::new(tokens).parse()?;
    let result = reduce(expression_tree)?;

    Ok(result)
}

fn decimilize(fraction: &Fraction) -> String {
    use sign::Sign;
    use std::collections::HashMap;

    let mut decimal = String::new();

    if fraction.sign == Sign::Negative {
        decimal.push('-');
    }

    if fraction.denumerator() == 1 {
        decimal += &*format!("{}", fraction.numerator());
    }
    else {
        decimal += &*format!("{}", fraction.numerator() / fraction.denumerator());
        decimal.push('.');
        
        let mut remainder = fraction.numerator() % fraction.denumerator();
        let mut rem_positions = HashMap::new();

        while remainder != 0 {
            if rem_positions.contains_key(&remainder) {
                decimal.insert(rem_positions[&remainder], '(');
                decimal.push(')');
                break
            }
            else {
                rem_positions.insert(remainder, decimal.len());
                remainder *= 10;
                decimal += &*format!("{}", remainder / fraction.denumerator());
                remainder %= fraction.denumerator();
            }
        }
    }

    decimal
}

fn get_input() -> Result<String, String> {
    let mut input = String::new();

    stdin().read_line(&mut input).map_err(|_| { "Incorrect input".to_owned() })?;

    if let Some('\n')= input.chars().next_back() {
        input.pop();
    }

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    Ok(input)
}