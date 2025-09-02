pub mod tokens;

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }
}
