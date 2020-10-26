use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Sum {
        left: Box<Expression>, right: Box<Expression>,
    },
    Product {
        left: Box<Expression>, right: Box<Expression>,
    },
    Division {
        left: Box<Expression>, right: Box<Expression>,
    },
    Negation(Box<Expression>),
    Integer(Token),
}