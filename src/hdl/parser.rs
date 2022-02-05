use hardware_sim::{ChipDef, ComponentMap};
use logos::{Lexer, Logos};
use tokenizer::{Error, Tokenizer, TypeEq};

// quick fix
fn expect(got: Option<Token>, expected: Token) -> Result<Token, Error> {
    if let Some(got) = got.clone() {
        if got.type_eq(&expected) {
            return Ok(got);
        }
    }
    return Err(Error::new(
        None,
        None,
        format!("expected {:?} but got {:?}", expected, got),
    ));
}

pub fn parse(code: &str) -> Result<Vec<ChipDef<ComponentMap>>, Error> {
    let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore(None)]);

    let mut chips = Vec::new();

    let mut name;
    let mut inputs;
    let mut outputs;
    let mut parts;

    loop {
        tokenizer.expect_next(Token::Chip)?;
        name = get_identifier(tokenizer.next())?;

        tokenizer.expect_next(Token::OpenC)?;

        tokenizer.expect_next(Token::In)?;
        inputs = get_names(&mut tokenizer)?;
        tokenizer.expect_next(Token::Semicolon)?;

        tokenizer.expect_next(Token::Out)?;
        outputs = get_names(&mut tokenizer)?;
        tokenizer.expect_next(Token::Semicolon)?;

        tokenizer.expect_next(Token::Parts)?;
        tokenizer.expect_next(Token::Colon)?;
        parts = get_parts(&mut tokenizer)?;
        tokenizer.expect_next(Token::CloseC)?;

        chips.push(ChipDef::new_string(name, inputs, outputs, parts));
        if tokenizer.peek().is_none() {
            break;
        }
    }
    Ok(chips)
}

fn get_parts(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<ComponentMap>, Error> {
    let mut parts = Vec::new();

    parts.push(get_component(tokenizer)?);

    while let Some(token) = tokenizer.peek() {
        if !token.type_eq(&Token::Identifier(String::new())) {
            break;
        }
        parts.push(get_component(tokenizer)?);
    }

    Ok(parts)
}

fn get_component(tokenizer: &mut Tokenizer<Token>) -> Result<ComponentMap, Error> {
    let chip_name = get_identifier(tokenizer.next())?;
    tokenizer.expect_next(Token::OpenP)?;

    let mut var_map = get_eq(tokenizer)?;

    let mut token = tokenizer.next();
    while let Some(t) = token.clone() {
        if !t.type_eq(&Token::Comma) {
            break;
        }
        get_eq(tokenizer)?
            .iter()
            .for_each(|temp| var_map.push(temp.to_owned()));
        token = tokenizer.next();
    }

    expect(token, Token::CloseP)?;
    tokenizer.expect_next(Token::Semicolon)?;

    Ok(ComponentMap::new_string(var_map, chip_name))
}

fn get_eq(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<(String, String)>, Error> {
    let first = get_name(tokenizer)?;
    tokenizer.expect_next(Token::Equals)?;
    let second = get_name(tokenizer)?;

    if first.len() != second.len() {
        todo!();
    }

    let mut var_map = Vec::new();
    for i in 0..first.len() {
        var_map.push((first[i].to_owned(), second[i].to_owned()));
    }
    Ok(var_map)
}

fn get_names(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<String>, Error> {
    let mut names = get_name(tokenizer)?;
    while let Some(token) = tokenizer.peek() {
        if !token.type_eq(&Token::Comma) {
            break;
        }
        tokenizer.expect_next(Token::Comma)?;
        for name in get_name(tokenizer)? {
            names.push(name);
        }
    }

    Ok(names)
}

fn get_name(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<String>, Error> {
    let identifier = get_identifier(tokenizer.next())?;
    if let Some(token) = tokenizer.peek() {
        if !token.type_eq(&Token::OpenB) {
            return Ok(vec![identifier]);
        }
        tokenizer.expect_next(Token::OpenB)?;
        let start = get_num(tokenizer.next())?;
        tokenizer.expect_next(Token::DoubleDot)?;
        let end = get_num(tokenizer.next())? + 1;
        tokenizer.expect_next(Token::CloseB)?;
        let mut result = Vec::new();
        for i in start..end {
            result.push(format!("{}{}", identifier, i));
        }
        return Ok(result);
    }
    Ok(vec![identifier])
}

fn get_num(token: Option<Token>) -> Result<usize, Error> {
    if let Token::Number(num) = expect(token, Token::Number(0))? {
        return Ok(num);
    } else {
        unreachable!();
    }
}

fn get_identifier(token: Option<Token>) -> Result<String, Error> {
    let token = expect(token, Token::Identifier(String::new()))?;
    if let Token::Identifier(name) = token {
        return Ok(name);
    } else {
        unreachable!();
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
enum Token {
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

impl TypeEq for Token {
    fn type_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Number(_), Token::Number(_)) => true,
            (Token::Identifier(_), Token::Identifier(_)) => true,
            (Token::Ignore(_), Token::Ignore(_)) => true,

            _ => self == other,
        }
    }
}

fn ignore(lex: &mut Lexer<Token>) -> Option<Option<String>> {
    let slice = lex.slice();
    match slice {
        " " => Some(None),
        "\n" => Some(Some("newline".to_string())),
        "\t" => Some(None),
        _ => Some(Some(slice.to_string())),
    }
}
