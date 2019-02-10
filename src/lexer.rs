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
            b'(' => new_token(TokenType::LParen),
            b')' => new_token(TokenType::RParan),
            b';' => new_token(TokenType::SemiColon),
            b'.' => new_token(TokenType::Comma),
            b'`' => new_token(TokenType::BackQuote),
            _ => {
                let literal = self.read_identifier()?.to_lowercase();
                let token_type: TokenType = token::keyword(&literal);

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

    struct Test {
        expected_type: TokenType,
        expected_literal: String,
    }

    #[test]
    fn test_token() {
        let mut l = Lexer::new(
            r#"
CREATE TABLE database_name.table_name(`foo` INT);
"#,
        );

        let tests = vec![
            create_test(TokenType::Create, "create"),
            create_test(TokenType::Table, "table"),
            create_test(TokenType::Ident, "database_name"),
            create_test(TokenType::Comma, "."),
            create_test(TokenType::Ident, "table_name"),
            create_test(TokenType::LParen, "("),
            create_test(TokenType::BackQuote, "`"),
            create_test(TokenType::Ident, "foo"),
            create_test(TokenType::BackQuote, "`"),
            create_test(TokenType::Int, "int"),
            create_test(TokenType::RParan, ")"),
            create_test(TokenType::SemiColon, ";"),
        ];

        for test in tests.iter() {
            let tok = l.token().expect("error");
            assert_eq!(tok.literal, test.expected_literal);
            assert_eq!(tok.token_type, test.expected_type);
        }
    }

    fn create_test(token_type: TokenType, literal: &str) -> Test {
        Test {
            expected_type: token_type,
            expected_literal: literal.to_string(),
        }
    }
}
