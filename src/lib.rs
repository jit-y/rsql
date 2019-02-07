pub mod lexer;
pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::token::keyword("create"),
            super::token::TokenType::CREATE
        );
    }
}
