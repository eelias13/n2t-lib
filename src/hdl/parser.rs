use hardware_sim::{ChipDef, ComponentMap};
use logos::{Lexer, Logos};
use std::fmt::Debug;
use std::iter::Peekable;
use std::slice::Iter;
// use tokenizer::{Error, Tokenizer, TypeEq};

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    line: Option<usize>,
    index: Option<usize>,
    len: Option<usize>,
    msg: String,
}

pub trait TokenTrait: Clone {
    type TokenType: PartialEq + Debug;
    fn line(&self) -> usize;
    fn index(&self) -> usize;
    fn len(&self) -> usize;
    fn token_type(&self) -> Self::TokenType;
}

impl Error {
    pub fn expect<T: TokenTrait>(
        got: Option<&T>,
        expected: T::TokenType,
    ) -> Result<T::TokenType, Self> {
        if let Some(token) = got {
            let token = token.clone();
            if token.token_type() == expected {
                Ok(token.token_type())
            } else {
                Err(Self {
                    line: Some(token.line()),
                    index: Some(token.index()),
                    len: Some(token.len()),
                    msg: format!(
                        "unexpected token expected <{:?}> but got <{:?}>",
                        expected,
                        token.token_type()
                    ),
                })
            }
        } else {
            Err(Self {
                line: None,
                index: None,
                len: None,
                msg: format!("unexpected end of file expected token <{:?}> ", expected),
            })
        }
    }

    pub fn msg_token<T: TokenTrait>(msg: &str, token: T) -> Self {
        Self {
            line: Some(token.line()),
            index: Some(token.index()),
            len: Some(token.len()),
            msg: msg.to_string(),
        }
    }

    pub fn msg_len<T: TokenTrait>(msg: &str, token: T, len: usize) -> Self {
        Self {
            line: Some(token.line()),
            index: Some(token.index()),
            len: Some(len),
            msg: msg.to_string(),
        }
    }

    pub fn msg(msg: &str) -> Self {
        Self {
            line: None,
            index: None,
            len: None,
            msg: msg.to_string(),
        }
    }
}

pub fn parse(code: &str) -> Result<Vec<ChipDef<ComponentMap>>, Error> {
    let tokens = tokenize(code);
    let mut t_iter = tokens.iter().peekable();

    let mut chips = Vec::new();

    let mut name;
    let mut inputs;
    let mut outputs;
    let mut parts;

    loop {
        Error::expect(t_iter.next(), TokenType::Chip)?;
        name = get_identifier(t_iter.next())?;

        Error::expect(t_iter.next(), TokenType::OpenC)?;

        Error::expect(t_iter.next(), TokenType::In)?;
        inputs = get_names(&mut t_iter)?;
        Error::expect(t_iter.next(), TokenType::Semicolon)?;

        Error::expect(t_iter.next(), TokenType::Out)?;
        outputs = get_names(&mut t_iter)?;
        Error::expect(t_iter.next(), TokenType::Semicolon)?;

        Error::expect(t_iter.next(), TokenType::Parts)?;
        Error::expect(t_iter.next(), TokenType::Colon)?;
        parts = get_parts(&mut t_iter)?;
        Error::expect(t_iter.next(), TokenType::CloseC)?;

        chips.push(ChipDef::new_string(name, inputs, outputs, parts));
        if t_iter.peek().is_none() {
            break;
        }
    }
    Ok(chips)
}

// --------------------------------- components ---------------------------------

fn get_parts(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<ComponentMap>, Error> {
    let mut parts = Vec::new();

    parts.push(get_component(t_iter)?);

    while let Some(&token) = t_iter.peek() {
        if !token.eq_type(TokenType::Identifier(String::new())) {
            break;
        }
        parts.push(get_component(t_iter)?);
    }

    Ok(parts)
}

fn get_component(t_iter: &mut Peekable<Iter<Token>>) -> Result<ComponentMap, Error> {
    let chip_name = get_identifier(t_iter.next())?;
    Error::expect(t_iter.next(), TokenType::OpenP)?;

    let mut var_map = get_eq(t_iter)?;

    let mut token = t_iter.next();
    while let Some(t) = token {
        if !t.eq_type(TokenType::Comma) {
            break;
        }
        get_eq(t_iter)?
            .iter()
            .for_each(|temp| var_map.push(temp.to_owned()));
        token = t_iter.next();
    }
    Error::expect(token, TokenType::CloseP)?;
    Error::expect(t_iter.next(), TokenType::Semicolon)?;

    Ok(ComponentMap::new_string(var_map, chip_name))
}

fn get_eq(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<(String, String)>, Error> {
    let first = get_name(t_iter)?;
    Error::expect(t_iter.next(), TokenType::Equals)?;
    let second = get_name(t_iter)?;

    if first.len() != second.len() {
        todo!();
    }

    let mut var_map = Vec::new();
    for i in 0..first.len() {
        var_map.push((first[i].to_owned(), second[i].to_owned()));
    }
    Ok(var_map)
}

// --------------------------------- utils ---------------------------------

fn get_names(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, Error> {
    let mut names = get_name(t_iter)?;
    while let Some(&token) = t_iter.peek() {
        if !token.eq_type(TokenType::Comma) {
            break;
        }
        Error::expect(t_iter.next(), TokenType::Comma)?;
        for name in get_name(t_iter)? {
            names.push(name);
        }
    }

    Ok(names)
}

fn get_name(t_iter: &mut Peekable<Iter<Token>>) -> Result<Vec<String>, Error> {
    let identifier = get_identifier(t_iter.next())?;
    if let Some(&token) = t_iter.peek() {
        if !token.eq_type(TokenType::OpenB) {
            return Ok(vec![identifier]);
        }
        Error::expect(t_iter.next(), TokenType::OpenB)?;
        let start = get_num(t_iter.next())?;
        Error::expect(t_iter.next(), TokenType::DoubleDot)?;
        let end = get_num(t_iter.next())? + 1;
        Error::expect(t_iter.next(), TokenType::CloseB)?;
        let mut result = Vec::new();
        for i in start..end {
            result.push(format!("{}{}", identifier, i));
        }
        return Ok(result);
    }
    Ok(vec![identifier])
}

fn get_num(token: Option<&Token>) -> Result<usize, Error> {
    if let TokenType::Number(num) = Error::expect(token, TokenType::Number(0))? {
        return Ok(num);
    } else {
        unreachable!();
    }
}

fn get_identifier(token: Option<&Token>) -> Result<String, Error> {
    let token = Error::expect(token, TokenType::Identifier(String::new()))?;
    if let TokenType::Identifier(name) = token {
        return Ok(name);
    } else {
        unreachable!();
    }
}

// ------------------------------- tokens ------------------------------------------------

fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut line = 0;
    let mut lex = TokenType::lexer(code);

    while let Some(token_type) = lex.next() {
        if let TokenType::Ignore(ignore) = token_type {
            if let Some(comment) = ignore {
                if comment == "newline".to_string() {
                    line += 1;
                }
            }
        } else {
            tokens.push(Token {
                index: lex.span().start,
                line,
                len: lex.span().len(),
                token_type,
            });
        }
    }

    tokens
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    index: usize,
    line: usize,
    len: usize,
    token_type: TokenType,
}

impl TokenTrait for Token {
    type TokenType = TokenType;
    fn line(&self) -> usize {
        self.line
    }
    fn index(&self) -> usize {
        self.index
    }
    fn len(&self) -> usize {
        self.len
    }
    fn token_type(&self) -> Self::TokenType {
        self.token_type.clone()
    }
}

impl Token {
    fn eq_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

#[derive(Logos, Debug, Clone)]
enum TokenType {
    #[token("CHIP")]
    Chip,
    #[token("IN")]
    In,
    #[token("OUT")]
    Out,
    #[token("PARTS")]
    Parts,

    #[token("{")]
    OpenC,
    #[token("}")]
    CloseC,
    #[token("(")]
    OpenP,
    #[token(")")]
    CloseP,
    #[token("[")]
    OpenB,
    #[token("]")]
    CloseB,

    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token("=")]
    Equals,
    #[token("..")]
    DoubleDot,
    #[token(":")]
    Colon,

    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    Ignore(Option<String>),

    #[regex(r"[a-zA-Z_$][a-zA-Z_$0-9]+", |lex| lex.slice().parse())]
    #[regex(r"[a-zA-Z]", |lex| lex.slice().parse())]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Number(usize),

    #[error]
    Unknown,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        match (self, other) {
            (TokenType::Number(_), TokenType::Number(_)) => true,
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (TokenType::Ignore(_), TokenType::Ignore(_)) => true,

            (TokenType::Chip, TokenType::Chip) => true,
            (TokenType::In, TokenType::In) => true,
            (TokenType::Out, TokenType::Out) => true,
            (TokenType::Parts, TokenType::Parts) => true,

            (TokenType::CloseB, TokenType::CloseB) => true,
            (TokenType::CloseC, TokenType::CloseC) => true,
            (TokenType::CloseP, TokenType::CloseP) => true,
            (TokenType::OpenB, TokenType::OpenB) => true,
            (TokenType::OpenC, TokenType::OpenC) => true,
            (TokenType::OpenP, TokenType::OpenP) => true,

            (TokenType::Colon, TokenType::Colon) => true,
            (TokenType::Semicolon, TokenType::Semicolon) => true,
            (TokenType::Comma, TokenType::Comma) => true,
            (TokenType::Equals, TokenType::Equals) => true,
            (TokenType::DoubleDot, TokenType::DoubleDot) => true,

            (TokenType::Unknown, TokenType::Unknown) => true,

            _ => false,
        }
    }
}

fn ignore(lex: &mut Lexer<TokenType>) -> Option<Option<String>> {
    let slice = lex.slice();
    match slice {
        " " => Some(None),
        "\n" => Some(Some("newline".to_string())),
        "\t" => Some(None),
        _ => Some(Some(slice.to_string())),
    }
}

#[cfg(test)]
mod test {
    use super::{ComponentMap, Token, TokenType};

    #[test]
    fn tokneize() {
        let code = " hello, world..";
        let tokens = super::tokenize(code);
        assert_eq!(
            tokens,
            vec![
                Token {
                    index: 1,
                    line: 0,
                    len: 5,
                    token_type: TokenType::Identifier("hello".to_string())
                },
                Token {
                    index: 6,
                    line: 0,
                    len: 1,
                    token_type: TokenType::Comma
                },
                Token {
                    index: 8,
                    line: 0,
                    len: 5,
                    token_type: TokenType::Identifier("world".to_string())
                },
                Token {
                    index: 13,
                    line: 0,
                    len: 2,
                    token_type: TokenType::DoubleDot,
                }
            ]
        )
    }

    #[test]
    fn get_name() {
        let code = " hello in[2..4]";
        let tokens = super::tokenize(code);
        let mut t_iter = tokens.iter().peekable();

        let name = super::get_names(&mut t_iter).unwrap();
        assert_eq!(name, vec!["hello"]);

        let name = super::get_names(&mut t_iter).unwrap();
        assert_eq!(name, vec!["in2", "in3", "in4"]);
    }

    #[test]
    fn get_component() {
        let code = "Nand(a=a, b=b, out=nand);";
        let tokens = super::tokenize(code);
        let mut t_iter = tokens.iter().peekable();

        let component = super::get_component(&mut t_iter).unwrap();
        assert_eq!(
            component,
            ComponentMap::new(vec![("a", "a"), ("b", "b"), ("out", "nand")], "Nand")
        );
    }
}
