use super::token;
use super::token::{Token, TokenType};

struct Lexer<'l> {
    input: &'l [u8],
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'l> Lexer<'l> {
    fn new(input: &'l str) -> Self {
        let mut lexer = Lexer {
            input: input.as_bytes(),
            position: 0,
            read_position: 0,
            ch: b'0',
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        match self.input.get(self.read_position) {
            Some(ch) => self.ch = *ch,
            None => self.ch = 0,
        }

        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn token(&mut self) -> Result<Token, failure::Error> {
        self.skip_whitespace();

        let token = match self.ch {
            b'(' => new_token(TokenType::LPAREN),
            b')' => new_token(TokenType::RPAREN),
            b';' => new_token(TokenType::SEMICOLON),
            b'.' => new_token(TokenType::COMMA),
            _ => {
                let literal = self.read_identifier()?;
                let token_type: TokenType = token::keyword(literal.as_str());

                let tok = Token {
                    literal: literal,
                    token_type: token_type,
                };

                return Ok(tok);
            }
        };

        self.read_char();

        Ok(token)
    }

    fn read_identifier(&mut self) -> Result<String, failure::Error> {
        let position = self.position;

        while is_letter(self.ch) {
            self.read_char()
        }

        match String::from_utf8(self.input[position..self.position].to_vec()) {
            Ok(ident) => Ok(ident),
            Err(err) => Err(failure::Error::from(err)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }
}

fn is_letter(ch: u8) -> bool {
    (b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_')
}

fn new_token(token_type: token::TokenType) -> Token {
    let literal = format!("{}", token_type);

    Token {
        token_type: token_type,
        literal: literal,
    }
}

#[test]
fn test_read_char() {
    let mut l = Lexer::new("abcde");
    l.read_position = 4;
    l.read_char();

    assert_eq!(l.ch, b'e')
}

#[cfg(test)]
mod tests {
    use super::token::TokenType;
    use super::Lexer;

    #[test]
    fn test_token() {
        let mut l = Lexer::new("create table database_name.table_name();");

        assert_eq!(l.token().unwrap().token_type, TokenType::CREATE);
        assert_eq!(l.token().unwrap().token_type, TokenType::TABLE);
        assert_eq!(l.token().unwrap().token_type, TokenType::IDENT);
        assert_eq!(l.token().unwrap().token_type, TokenType::COMMA);
        assert_eq!(l.token().unwrap().token_type, TokenType::IDENT);
        assert_eq!(l.token().unwrap().token_type, TokenType::LPAREN);
        assert_eq!(l.token().unwrap().token_type, TokenType::RPAREN);
        assert_eq!(l.token().unwrap().token_type, TokenType::SEMICOLON);
    }

}
