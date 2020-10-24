use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Expression {
    Infix {
        left: Box<Expression>, op: TokenType, right: Box<Expression>
    },
    Number(Token),
}