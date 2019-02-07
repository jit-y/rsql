use std::fmt;

pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    CREATE,
    IDENT,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token: &str = match *self {
            TokenType::CREATE => "CREATE",
            TokenType::IDENT => "IDENT",
        };

        write!(f, "{}", token)
    }
}

pub fn keyword(key: &str) -> TokenType {
    match key.to_lowercase().as_str() {
        "create" => TokenType::CREATE,
        _ => TokenType::IDENT,
    }
}
