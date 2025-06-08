#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Float(f64),
    String(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Break,
    Do,
    If,
    ElseIf,
    Goto,
    In,
    True,
    False,
    LessThan,
    GreaterThan,
    Equal,
    ComparativeEqual,
    LArrow,
    RArrow,
    NotEqual,
    Dot,
    Semicolon,
    LBrace,
    RBrace,
    LSquareBracket,
    RSquareBracket,
    Comma,
    Nil,
    For,
    Package,
    EOF,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&ch) = iter.peek() {
        match ch {
            c if c.is_whitespace() => { iter.next(); }
            '+' => { tokens.push(Token::Plus); iter.next(); }
            '-' => { 
                iter.next();
                if let Some('>') = iter.peek() {
                    iter.next();
                    tokens.push(Tokens::RArrow);
                } else {
                    tokens.push(Token::Minus)
                }
            }
            '*' => { tokens.push(Token::Multiply); iter.next(); }
            '/' => { tokens.push(Token::Divide); iter.next(); }
            '(' => { tokens.push(Token::LParen); iter.next(); }
            ')' => { tokens.push(Token::RParen); iter.next(); }
            '{' => { tokens.push(Token::LBrace); iter.next(); }
            '}' => { tokens.push(Token::RBrace); iter.next(); }
            '[' => { tokens.push(Token::LSquareBracket); iter.next(); }
            ']' => { tokens.push(Token::RSquareBracket); iter.next(); }
            ',' => { tokens.push(Token::Comma); iter.next(); }
            ';' => { tokens.push(Token::Semicolon); iter.next(); }
            '.' => { tokens.push(Token::Dot); iter.next(); }
            '<' => {
                iter.next();
                if let Some('-') = iter.peek() {
                    iter.next();
                    tokens.push(Tokens::LArrow);
                } else {
                    tokens.push(Tokens::LessThan)
                }
            }
            '>' => { tokens.push(Token::GreaterThan); iter.next(); }
            '=' => {
                iter.next();
                if let Some('=') = iter.peek() {
                    iter.next();
                    tokens.push(Token::ComparativeEqual);
                } else {
                    tokens.push(Token::Equal);
                }
            }
            '!' => {
                iter.next();
                if let Some('=') = iter.peek() {
                    iter.next();
                    tokens.push(Token::NotEqual);
                }
            }
            '0'..='9' => {
                let mut num = String::new();
                let mut is_float = false;
                while let Some(&c) = iter.peek() {
                    if c.is_ascii_digit() {
                        num.push(c);
                        iter.next();
                    } else if c == '.' && !is_float {
                        is_float = true;
                        num.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                if is_float {
                    tokens.push(Token::Float(num.parse().unwrap()));
                } else {
                    tokens.push(Token::Number(num.parse().unwrap()));
                }
            }
            '"' => {
                iter.next();
                let mut s = String::new();
                while let Some(&c) = iter.peek() {
                    if c == '"' {
                        break;
                    }
                    if c == '\\' {      // check for escape chars
                        let mut s = String::new();
                        iter.next()     // consume '\'
                        if let Some(&esc) = iter.peek() {
                            let esc_char = match esc {
                                'n' => '\n',
                                "'" => "'",
                                '"' => '"',
                                '\\' => '\\',
                                other => other,
                            };
                        s.push(esc_char);
                        iter.next();
                        }
                    } else {

                    }
                }
                iter.next();
                tokens.push(Token::String(s));
            }
            c if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while let Some(&c) = iter.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "break" => tokens.push(Token::Break),
                    "do" => tokens.push(Token::Do),
                    "if" => tokens.push(Token::If),
                    "elif" => tokens.push(Token::ElseIf),
                    "goto" => tokens.push(Token::Goto),
                    "in" => tokens.push(Token::In),
                    "true" => tokens.push(Token::True),
                    "false" => tokens.push(Token::False),
                    "nil" => tokens.push(Token::Nil),
                    "for" => tokens.push(Token::For),
                    "pkg" => tokens.push(Token::Package),
                    _ => {}
                }
            }
        }
        tokens.push(Token::EOF);
    }
}