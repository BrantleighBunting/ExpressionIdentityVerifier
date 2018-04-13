use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Set(HashSet<i64>),
    Plus,
    Multiply,
    LeftMustache,
    RightMustache,
    LeftParentheses,
    RightParentheses
}

impl Token {
    pub fn operator_precedence(self) -> i64 {
        match self {
            Token::Plus => 1,
            Token::Multiply => 2,
            _ => 0,
        }
    }
}