use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Token {
    CREATE,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token: &str = match *self {
            Token::CREATE => "CREATE",
        };

        write!(f, "{}", token)
    }
}

pub fn keyword(key: &str) -> Result<Token, &str> {
    match key {
        "create" => Ok(Token::CREATE),
        _ => Err("not implemented"),
    }
}
