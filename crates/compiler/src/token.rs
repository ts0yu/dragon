use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    #[token("push")]
    Push,
    #[token("jump")]
    Jump,
    #[token("set")]
    Set,
    #[token("get")]
    Get,
    #[token("pop")]
    Pop,
    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("pc")]
    Pc,
    #[token("print")]
    Print,
    #[token("halt")]
    Halt,
    #[token("+")]
    AddSymb,
    #[token("-")]
    SubSymb,
    #[token("=")]
    SetSymb,
    #[token("$")]
    GetSymb,
    #[token("mul")]
    Mul,
    #[token("dup")]
    Dup,
    #[token("*")]
    MulSymb,
    #[token("sqrt")]
    Sqrt,
    #[regex(r"[//.*]+", logos::skip)] // Comments
    Comment,
    #[regex("[+-]?([0-9]*[.])?[0-9]+")]
    Literal,
    // Eof,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token<'a> {
    pub ttype: TokenType,
    pub slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(ttype: TokenType, slice: &'a str) -> Self {
        Self { ttype, slice }
    }

    pub fn lex(raw: &'a str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut lex = TokenType::lexer(raw);

        loop {
            let z = lex.next();
            if z == None {
                break;
            }
            tokens.push(Self::new(z.unwrap(), lex.slice()));
        }

        // tokens.push(Token::new(TokenType::Eof, ""));

        tokens
    }
}
