use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Expression>, operator: TokenType, right: Box<Expression>
    },
    Number(Token),
}