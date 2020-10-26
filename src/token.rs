use crate::token_type::TokenType::{self, *};

#[derive(Debug, Clone)]
pub enum Token {
    Integer(u64),
    Symbol(TokenType),
    EndOfFile,
}

impl Token {
    pub fn r#type(&self) -> TokenType{
        use Token::*;

        match self {
            Integer(_) => INTEGER,
            Symbol(token_type) => token_type.clone(),
            EndOfFile => EOF,
        }
    }
}