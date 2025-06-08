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
    Indent,
    Dedent,
    Newl,
    And,
    Or,
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

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();
    let mut stack = vec![0];
    let mut line_start = true;

    while let Some(&ch) = iter.peek() {
        if line_start {
            // This is prolly gonna 100% fuck me over
            let mut indent = 0;
            while let Some(&c) = iter.peek() {
                if c == ' ' {
                    indent += 1;
                    iter.next();
                } else if c == '\t' {
                    indent += 4;
                    iter.next();
                } else {
                    break;
                }
            }
            if indent > *stack.last().unwrap() {
                stack.push(indent);
                tokens.push(Token::Indent);
            } else {
                while indent < *stack.last().unwrap() {
                    stack.pop();
                    tokens.push(Token::Dedent);
                }
            }
            line_start = false;
        }
        match ch {
            c if c.is_whitespace() => { iter.next(); }
            '+' => { tokens.push(Token::Plus); iter.next(); }
            '-' => { 
                iter.next();
                if let Some('>') = iter.peek() {
                    iter.next();
                    tokens.push(Token::RArrow);
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
            '\n' => { tokens.push(Token::Newl); iter.next(); line_start = true; }
            '<' => {
                iter.next();
                if let Some('-') = iter.peek() {
                    iter.next();
                    tokens.push(Token::LArrow);
                } else {
                    tokens.push(Token::LessThan)
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
            '&' => { tokens.push(Token::And); iter.next(); }
            '|' => { tokens.push(Token::Or); iter.next(); }
            '!' => {
                iter.next();
                if let Some('=') = iter.peek() {
                    iter.next();
                    tokens.push(Token::NotEqual);
                } else {
                    tokens.push(Token::Not);
                }
            }
            '%' => { tokens.push(Token::Modulo); iter.next(); }
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
                    if c == '\\' {
                        iter.next();
                        if let Some(&esc) = iter.peek() {
                            let esc_char = match esc {
                                'n' => '\n',
                                '\'' => '\'',
                                '"' => '"',
                                '\\' => '\\',
                                other => other,
                            };
                            s.push(esc_char);
                            iter.next();
                        }
                    } else {
                        s.push(c);
                        iter.next();
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
                    _ => { tokens.push(Token::Identifier(ident)); }
                }
            }
            _ => { return Err(format!("Unexpected character {}", ch)); }
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}