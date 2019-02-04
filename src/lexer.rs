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
}

#[test]
fn test_read_char() {
    let mut l = Lexer::new("abcde");
    l.read_position = 4;
    l.read_char();

    assert_eq!(l.ch, 'e')
}
