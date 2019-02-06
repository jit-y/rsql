use std::fmt;

pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    CREATE,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token: &str = match *self {
            TokenType::CREATE => "CREATE",
        };

        write!(f, "{}", token)
    }
}

pub fn keyword(key: &str) -> Result<TokenType, failure::Error> {
    match key.to_lowercase().as_str() {
        "create" => Ok(TokenType::CREATE),
        _ => Err(failure::err_msg(key.to_string())),
    }
}
