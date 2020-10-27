use crate::expression::Expression;
use crate::fraction::Fraction;
use crate::token::Token;
use std::result;

pub type Result<T> = result::Result<T, String>;

pub fn reduce(tree: Expression) -> Result<Fraction> {
    reduce_expression(Box::new(tree))
}

fn reduce_expression(expression: Box<Expression>) -> Result<Fraction> {
    use Expression::*;

    match *expression {
        Sum { left, right } => {
            let left_result = reduce_expression(left)?;
            let right_result = reduce_expression(right)?;

            let result = left_result + right_result;

            Ok(result)
        }
        Product { left, right } => {
            let left_result = reduce_expression(left)?;
            let right_result = reduce_expression(right)?;
            let result = left_result * right_result;

            Ok(result)
        }
        Division { left, right } => {
            let left_result = reduce_expression(left)?;
            let right_result = reduce_expression(right)?;

            let result = left_result / right_result;

            Ok(result)
        }
        Negation(expression) => {
            let expression_result = reduce_expression(expression)?;

            Ok(expression_result.negate())
        }
        Number(fraction) => {
            Ok(fraction)
        }
    }
}