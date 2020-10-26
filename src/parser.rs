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
        // source : expression EOF
        let expression = self.parse_expression();

        self.eat(EOF)?;

        expression
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        // expression : term
        self.parse_term()
    }

    fn parse_term(&mut self) -> Result<Expression> {
        // term : factor (('+' | '-') factor)*

        let mut expression = self.parse_factor()?;

        while self.check_any(vec![PLUS, MINUS]) {
            let operator  = self.current_token.clone().unwrap().r#type();

            self.advance();

            let right_expression;

            if operator == MINUS {
                right_expression = Expression::Negation(
                    Box::new(self.parse_factor()?)
                );
            }
            else {
                right_expression = self.parse_factor()?;
            }

            
            expression = Expression::Sum {
                left: Box::new(expression),
                right: Box::new(right_expression),
            }
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<Expression> {
        // factor : unary (('*' | '/') unary)*

        let mut expression = self.parse_unary()?;

        while self.check_any(vec![MULTIPLY, DIVIDE]) {
            let operator = self.current_token.clone().unwrap().r#type();

            self.advance();

            let right_expression = self.parse_unary()?;

            if operator == MULTIPLY {
                expression = Expression::Product {
                    left: Box::new(expression),
                    right: Box::new(right_expression),
                }
            }
            else {
                expression = Expression::Division {
                    left: Box::new(expression),
                    right: Box::new(right_expression),
                }
            }
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression> {
        // unary : ('+' | '-')? primary

        if self.check_any(vec![PLUS, MINUS]) {
            let operator = self.current_token.clone().unwrap().r#type();

            self.advance();

            if operator == MINUS {
                return Ok(Expression::Negation(
                    Box::new(self.parse_primary()?)
                ))
            }
        }
        
        Ok(self.parse_primary()?)
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        // primary : INTEGER | '(' expression ')'

        if self.check(LPAREN) {
            self.eat(LPAREN)?;

            let expression = self.parse_expression()?;

            self.eat(RPAREN)?;

            Ok(expression)
        }
        else {
            let integer = self.eat(INTEGER)?;
    
            Ok(Expression::Integer(integer))
        }
    }

    fn eat(&mut self, token_type: TokenType) -> Result<Token> {
        let result;
        
        if self.check(token_type) {
            result = Ok(self.current_token.clone().unwrap());
        }
        else {
            result = Err(format!(
                "Expected {:?} but got {:?}",
                token_type,
                self.current_token.clone().unwrap().r#type(),
            ));
        }

        self.advance();

        result
    }

    fn check_any(&self, token_types: Vec<TokenType>) -> bool {
        token_types.into_iter().any(|token_type| {
            self.check(token_type)
        })
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.current_token.as_ref().map_or(
            false,
            |token| { token.r#type() == token_type }
        )
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next();
    }
}