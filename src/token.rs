use std::fmt;

pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Create,
    Table,

    Ident,

    LParen,
    RParan,
    SemiColon,
    Comma,
    BackQuote,

    Int,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token: &str = match *self {
            TokenType::Create => "CREATE",
            TokenType::Table => "TABLE",
            TokenType::Ident => "IDENT",

            TokenType::LParen => "(",
            TokenType::RParan => ")",
            TokenType::SemiColon => ";",
            TokenType::Comma => ".",
            TokenType::BackQuote => "`",

            TokenType::Int => "INT",
        };

        write!(f, "{}", token)
    }
}

pub fn keyword(key: &str) -> TokenType {
    match key {
        "create" => TokenType::Create,
        "table" => TokenType::Table,

        "int" => TokenType::Int,
        _ => TokenType::Ident,
    }
}
