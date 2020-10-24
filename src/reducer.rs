use crate::expression::Expression;
use crate::token::Token;
use crate::token_type::TokenType::*;
use std::result;

pub type Result<T> = result::Result<T, String>;

pub fn reduce(tree: Expression) -> Result<f64> {
    reduce_expression(Box::new(tree))
}

fn reduce_expression(expression: Box<Expression>) -> Result<f64> {
    use Expression::*;

    match *expression {
        Binary { left, operator, right } => {
            let left_result = reduce_expression(left)?;
            let right_result = reduce_expression(right)?;

            match operator {
                PLUS => Ok(left_result + right_result),
                MINUS => Ok(left_result - right_result),
                MULTIPLY => Ok(left_result * right_result),
                DIVIDE => {
                    if right_result == 0f64 {
                        Err(format!("Can't divide by 0"))
                    }
                    else {
                        Ok(left_result / right_result)
                    }
                }
                _ => {
                    panic!("Internal: Can't treat {:?} as an operator", operator);
                }
            }
        }
        Number(token) => {
            if let Token::Number(number) = token {
                Ok(number)
            }
            else {
                panic!("Internal: Token {:?} is not a number", token)
            }
        }
    }

}