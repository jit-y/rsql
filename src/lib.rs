pub mod token;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(
            super::token::keyword("create").unwrap(),
            super::token::Token::CREATE
        );
    }
}
