#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    INTEGER,
    PLUS, MINUS, MULTIPLY, DIVIDE,
    LPAREN, RPAREN,
    EOF,
}