#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    NUMBER,
    PLUS, MINUS, MULTIPLY, DIVIDE,
    LPAREN, RPAREN,
    EOF,
}