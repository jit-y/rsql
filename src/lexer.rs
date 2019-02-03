struct Lexer {
    input: String,
    position: u64,
    read_position: u64,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }
}
