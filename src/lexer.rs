use logos::{Lexer, Logos};

pub struct Tokenizer {
    lexer: Lexer<'static, Token>,
    current: Option<Token>,
    next: Option<Token>,
}

impl Tokenizer {
    pub fn new(code: &str) {
        let mut lexer = Token::lexer(code.clone());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    index: Option<usize>,
    len: Option<usize>,
    msg: String,
}

impl Error {
    fn new(line: Option<usize>, index: Option<usize>, len: Option<usize>, msg: String) -> Self {
        Self { index, len, msg }
    }

    fn lexer(msg: String, lexer: &Lexer<Token>) -> Self {
        Self {
            msg,
            index: Some(lexer.span().start),
            len: Some(lexer.span().len()),
        }
    }

    pub fn expect(got: Option<Token>, token: Token, lexer: &Lexer<Token>) -> Result<Token, Self> {
        if let Some(got) = got {
            if token == got {
                Ok(got)
            } else {
                Err(Self::lexer(
                    format!("expect token {:?} but got {:?}", token, got),
                    lexer,
                ))
            }
        } else {
            Err(Self::lexer(
                format!("expect token {:?} but got {}", token, "end of file"),
                lexer,
            ))
        }
    }

    pub fn expect_multi(
        got: Option<Token>,
        tokens: Vec<Token>,
        lexer: &Lexer<Token>,
    ) -> Result<Token, Self> {
        if let Some(got) = got {
            if tokens.contains(&got) {
                Ok(got)
            } else {
                Err(Self::lexer(
                    format!("expect tokens {:?} but got {:?}", tokens, got),
                    lexer,
                ))
            }
        } else {
            Err(Self::lexer(
                format!("expect tokens {:?} but got {}", tokens, "end of file"),
                lexer,
            ))
        }
    }
}

#[derive(Logos, Debug, Clone)]
pub enum Token {
    #[token("push")]
    Push,
    #[token("pop")]
    Pop,

    #[token("add")]
    Add,
    #[token("sub")]
    Sub,
    #[token("neg")]
    Neg,
    #[token("eq")]
    Eq,
    #[token("gt")]
    Gt,
    #[token("lt")]
    Lt,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,

    #[token("lable")]
    Lable,
    #[token("goto")]
    Goto,
    #[token("if-goto")]
    IfGoto,

    #[token("function")]
    Function,
    #[token("call")]
    Call,
    #[token("return")]
    Return,

    #[token("this")]
    This,
    #[token("that")]
    That,
    #[token("local")]
    Local,
    #[token("argument")]
    Argument,
    #[token("static")]
    Static,
    #[token("pointer")]
    Pointer,
    #[token("temp")]
    Temp,
    #[token("constant")]
    Constant,

    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    Ignore((usize, Option<String>)),

    #[regex(r"[a-zA-Z]+", |lexer| lexer.slice().parse())]
    Name(String),
    #[regex(r"[0-9]+", |lexer| lexer.slice().parse())]
    Number(usize),
    #[token("-")]
    MinusSign,

    #[error]
    Unknown,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Number(_), Token::Number(_)) => true,
        }
    }
}
fn ignore(lexer: &mut Lexer<Token>) -> Option<(usize, Option<String>)> {
    let slice = lexer.slice();
    match slice {
        " " => Some((0, None)),
        "\n" => Some((0, Some("newline".to_string()))),
        "\t" => Some((0, None)),
        _ => Some((0, Some(slice.to_string()))),
    }
}
