use crate::expression::Expression;
use crate::token::Token;
use crate::token_type::TokenType::{self, *};
use std::result;
use std::vec::IntoIter;

pub struct Parser {
    tokens: IntoIter<Token>,
    current_token: Option<Token>,
}

pub type Result<T> = result::Result<T, String>;


impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut tokens_iter = tokens.into_iter();

        Self {
            current_token: tokens_iter.next(),
            tokens: tokens_iter,
        }
    }

    pub fn parse(mut self) -> Result<Expression> {
        /*
            expression : low ('-' | '+') low
            low : middle ('*' | '/') middle
            high : primary '^' primary              -- right precedence
            primary : ('-' | '+') NUMBER
        */

        let left = self.eat(NUMBER)?;
        
        self.eat(PLUS)?;

        let right = self.eat(NUMBER)?;

        Ok(Expression::Infix {
            left: Box::new(Expression::Number(left)),
            op: PLUS,
            right: Box::new(Expression::Number(right)),
        })
    }

    fn eat(&mut self, token_type: TokenType) -> Result<Token> {
        let result;
        
        if self.current_token.as_ref().map_or(false, |t| { t.r#type() == token_type }) {
            result = Ok(self.current_token.clone().unwrap());
        }
        else {
            result = Err(format!(
                "Expected {:?} but got {:?}",
                token_type,
                self.current_token
            ));
        }

        self.advance();

        result
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }
}