use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i64),
    Set(HashSet<i64>),
    Plus,
    Minus,
    Multiply,
    Power,
    LeftMustache,
    RightMustache,
    LeftParentheses,
    RightParentheses
}

impl Token {
    pub fn operator_precedence(self) -> i64 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Power => 2,
            _ => 0,
        }
    }
}