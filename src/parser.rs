mod lex;

#[derive(Debug, Clone)]
pub enum Grammar {
    Number(i64),
    Float(f64),
    String(String),
    Paren,
    Identifier(String),
}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: Grammar,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: Grammar::Paren,
        }
    }
}

pub fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_expr(&tokens, 0).and_then(|(n, i)| if i == tokens.len() {
        Ok(n)
    } else {
        Err(format!("Expected end of input {:?} at {}", tokens[i], i))
    })
}

pub fn parse_expr(tokens: &[Token], pos: usize) -> Result<(Grammar, usize), String> {
    // todo later im lazy
}