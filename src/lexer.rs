use crate::token::Token;
use crate::token_type::TokenType::*;
use crate::fraction::Fraction;
use crate::sign::Sign;
use std::vec::IntoIter;
use std::result;

pub struct Lexer {
    source_string: String,
    source: IntoIter<char>,
    lookahead: Option<char>,
    index: u32,
}

pub type Result<T> = result::Result<T, String>;

impl Lexer {
    pub fn new(source_string: String) -> Self {
        let mut source = {
            let char_vector: Vec<char> = source_string.chars().collect();
            char_vector.into_iter()
        };

        Self {
            source_string: source_string,
            lookahead: source.next(),
            source: source,
            index: 0,
        }
    }

    pub fn lex(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            match (&mut self).lex_token() {
                Ok(Some(token)) => {
                    tokens.push(token);
                },
                Err(message) => {
                    return Err(message)
                },
                _ => { },
            }
        }

        tokens.push(Token::EndOfFile);

        Ok(tokens)
    }

    fn lex_token(&mut self) -> Result<Option<Token>> {
        if self.check_if(|c| { c.is_digit(10) }) {
            let number = self.lex_number();
            return Ok(Some(Token::Number(number)))
        }
        else if self.check('.') {
            let decimal = self.lex_decimal();
            return Ok(Some(Token::Number(decimal)))
        }
        else if self.eat('+') {
            return Ok(Some(Token::Symbol(PLUS)))
        }
        else if self.eat('-') {
            return Ok(Some(Token::Symbol(MINUS)))
        }
        else if self.eat('*') {
            return Ok(Some(Token::Symbol(MULTIPLY)))
        }
        else if self.eat('/') {
            return Ok(Some(Token::Symbol(DIVIDE)))
        }
        else if self.eat('(') {
            return Ok(Some(Token::Symbol(LPAREN)))
        }
        else if self.eat(')') {
            return Ok(Some(Token::Symbol(RPAREN)))
        }
        else if self.check_if(|c| { c.is_whitespace() }) {
            self.advance();
            return Ok(None);
        }
        else {
            Err(
                self.create_error_message(self.lookahead.unwrap_or('\0'))
            )
        }
    }

    fn lex_number(&mut self) -> Fraction {
        let integer = {
            let integer_str = self.record_while(|c| { c.is_digit(10) });
            Fraction::new(integer_str.parse().unwrap(), 1, Sign::Positive)
        };

        if self.check('.') {
            let decimal = self.lex_decimal();
            integer + decimal
        }
        else {
            integer
        }
    }

    fn lex_decimal(&mut self) -> Fraction {
        self.eat('.');

        let decimal_str = self.record_while(|c| { c.is_digit(10) });
        let denumerator = (10u64).pow(decimal_str.len() as u32);
        
        Fraction::new(
            decimal_str.parse().unwrap(),
            denumerator,
            Sign::Positive,
        )
    }

    fn record_while(&mut self, predicate: impl Fn(char) -> bool) -> String {
        let mut recording = String::new();

        while self.check_if(&predicate) {
            if let Some(c) = self.lookahead {
                recording.push(c);
            }

            self.advance();
        }

        recording
    }

    fn eat(&mut self, character: char) -> bool {
        if self.check(character) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, character: char) -> bool {
        self.check_if(|lookahead| { character == lookahead })
    }

    fn check_if(&self, predicate: impl Fn(char) -> bool) -> bool{
        self.lookahead.map_or(false, predicate)
    }

    fn advance(&mut self) {
        self.lookahead = self.source.next();
        self.index += 1;
    }

    fn is_at_end(&self) -> bool {
        self.lookahead.is_none()
    }

    fn create_error_message(&self, c: char) -> String {
        let arrow = {
            let mut string = String::new();

            for _ in 0..self.index {
                string.push('-');
            }

            string.push('^');

            string
        };

        let message = format!("Unexpected character: \'{}\'", c);

        format!(
            "| {source}\n| {arrow}\n|\n| {message}",
            source = self.source_string,
            arrow = arrow,
            message = message,
        )
    }
}
