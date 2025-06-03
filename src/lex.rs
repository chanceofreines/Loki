#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    EOF,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable()

    while let Some(ch) = iter.peek() {
        match ch {
            ch if ch.is_whitespace() => iter.next(),
            '+' => { tokens.push(Token::Plus); iter.next(); }
            '-' => { tokens.push(Token::Minus); iter.next(); }
            '*' => { tokens.push(Token::Multiply); iter.next(); }
            '/' => { tokens.push(Token::Divide); iter.next(); }
            '(' => { tokens.push(Token::LParen); iter.next(); }
            ')' => { tokens.push(Token::RParen); iter.next(); }
        }
    }
}