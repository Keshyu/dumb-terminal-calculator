use crate::fraction::Fraction;
use crate::token_type::TokenType::{self, *};

#[derive(Debug, Clone)]
pub enum Token {
    Number(Fraction),
    Symbol(TokenType),
    EndOfFile,
}

impl Token {
    pub fn r#type(&self) -> TokenType{
        use Token::*;

        match self {
            Number(_) => NUMBER,
            Symbol(token_type) => token_type.clone(),
            EndOfFile => EOF,
        }
    }
}