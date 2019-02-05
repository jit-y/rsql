use std::str::Chars;

struct Lexer<'l> {
    input: Chars<'l>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'l> Lexer<'l> {
    fn new(input: &'l str) -> Self {
        Lexer {
            input: input.chars(),
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    fn read_char(&mut self) {
        match self.input.nth(self.read_position) {
            Some(ch) => self.ch = ch,
            None => self.ch = '0',
        }

        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    // fn token(&mut self) -> super::token::Token {
    //     let token: super::token::Token;

    //     self.skip_whitespace();

    //     match self.ch {
    //         '=' => {}
    //         _ => {}
    //     };

    //     token
    // }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}
#[test]
fn test_read_char() {
    let mut l = Lexer::new("abcde");
    l.read_position = 4;
    l.read_char();

    assert_eq!(l.ch, 'e')
}
