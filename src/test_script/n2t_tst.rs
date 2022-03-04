use super::{N2tCmp, OutType};
use logos::{Lexer, Logos};
use tokenizer::{Error, Tokenizer, TypeEq};

const DOUBLE_QUOTES: &str = "#!#";

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Set(String, OutType),
    Tock,
    Tick,
    TickTock,
    Output,
    Eval,
    Echo(String),
    BeginRepeat(Option<usize>),
    EndRepeat,
    EndInstruction,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OutList {
    name: String,
    triplet: (usize, usize, usize),
    out_type: OutType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct N2tTst {
    load: String,
    out_file: Option<String>,
    compare_to: Option<String>,
    out_list: Option<Vec<OutList>>,
    instruction: Vec<Instruction>,

    out: Option<N2tCmp>,
}

impl OutList {
    pub fn new(name: &str, triplet: (usize, usize, usize), out_type: OutType) -> Self {
        Self {
            name: name.to_string(),
            triplet,
            out_type,
        }
    }
}

impl N2tTst {
    pub fn new(
        load: &str,
        out_file: Option<&str>,
        compare_to: Option<&str>,
        out_list: Option<Vec<OutList>>,
        instruction: Vec<Instruction>,
    ) -> Self {
        let out_file = if let Some(val) = out_file {
            Some(val.to_string())
        } else {
            None
        };

        let compare_to = if let Some(val) = compare_to {
            Some(val.to_string())
        } else {
            None
        };

        Self {
            load: load.to_string(),
            out_file,
            compare_to,
            out_list,
            instruction,
            out: None,
        }
    }

    pub fn from_code(code: &str) -> Result<Self, Error> {
        // let code = &code.replace("\"", DOUBLE_QUOTES);

        let load;
        let out_file;
        let compare_to;
        let out_list;

        let mut instruction = Vec::new();
        let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);

        tokenizer.next();
        // get head
        expect_str(&mut tokenizer, "load")?;
        load = get_str(&mut tokenizer)?;
        if tokenizer.next_is(Token::Comma) {
            tokenizer.expect(Token::Comma)?;

            expect_str(&mut tokenizer, "output-file")?;
            out_file = Some(get_str(&mut tokenizer)?);
            tokenizer.expect(Token::Comma)?;

            expect_str(&mut tokenizer, "compare-to")?;
            compare_to = Some(get_str(&mut tokenizer)?);
            tokenizer.expect(Token::Comma)?;

            expect_str(&mut tokenizer, "output-list")?;
            out_list = Some(get_out_list(&mut tokenizer)?);
        } else {
            tokenizer.expect(Token::Semicolon)?;

            out_file = None;
            compare_to = None;
            out_list = None;
        }

        while tokenizer.current() != None {
            if tokenizer.next_is(Token::CurlyClose) {
                tokenizer.expect(Token::CurlyClose)?;
                instruction.push(Instruction::EndRepeat);
                continue;
            }

            let value = get_str(&mut tokenizer)?;
            let mut is_repeat = false;
            match value.as_str() {
                "tock" => instruction.push(Instruction::Tock),
                "tick" => instruction.push(Instruction::Tick),
                "ticktock" => instruction.push(Instruction::TickTock),
                "output" => instruction.push(Instruction::Output),
                "eval" => instruction.push(Instruction::Eval),
                "echo" => instruction.push(echo(&mut tokenizer)?),
                "repeat" => {
                    instruction.push(repeat(&mut tokenizer)?);
                    is_repeat = true;
                }
                "set" => instruction.push(set(&mut tokenizer, &out_list)?),
                _ => return Err(tokenizer.error(&format!("expected string set but got {}", value))),
            }

            if !is_repeat {
                if tokenizer.next_is(Token::Comma) {
                    tokenizer.expect(Token::Comma)?;
                } else {
                    tokenizer.expect(Token::Semicolon)?;
                    instruction.push(Instruction::EndInstruction);
                }
            }
        }

        Ok(Self {
            load,
            out_file,
            compare_to,
            out_list,
            instruction,

            out: None,
        })
    }

    // pub fn validate(&self, cmp: CMP) -> Result<(), usize> {
    //     Ok(())
    // }
}

fn expect_str(tokenizer: &mut Tokenizer<Token>, expect: &str) -> Result<(), Error> {
    let token = tokenizer.expect(Token::String(String::new()))?;
    if token == Token::String(expect.to_string()) {
        Ok(())
    } else {
        Err(tokenizer.error(&format!(
            "expected string {} but got token {:?}",
            expect, token
        )))
    }
}

fn get_str(tokenizer: &mut Tokenizer<Token>) -> Result<String, Error> {
    if let Token::String(value) = tokenizer.expect(Token::String(String::new()))? {
        return Ok(value);
    }
    unreachable!();
}

fn get_out_list(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<OutList>, Error> {
    let mut out_list = Vec::new();
    while !tokenizer.next_is(Token::Semicolon) {
        out_list.push(get_out(tokenizer)?);
    }
    Ok(out_list)
}

fn get_out(tokenizer: &mut Tokenizer<Token>) -> Result<OutList, Error> {
    let temp = get_str(tokenizer)?;
    let mut chars = temp.chars();

    let mut name = String::new();

    while let Some(c) = chars.next() {
        if c == '%' {
            break;
        }

        name.push(c);
    }

    let out_type = match chars.next() {
        Some('B') => OutType::Binary(Vec::new()),
        Some('D') => OutType::Decimal(0),
        Some('S') => OutType::Clock((0, false)),

        Some(c) => {
            return Err(tokenizer.error(&format!("unexpectde symbol {} expected D, B or S", c)))
        }
        None => return Err(tokenizer.error("wrong format")),
    };

    let temp: String = chars.collect();
    let mut triplet = Vec::<usize>::new();
    for num in temp.split('.') {
        if let Ok(num) = num.parse() {
            triplet.push(num)
        } else {
            return Err(tokenizer.error(&format!("unexpectde symbol {} expected a number", num)));
        }
    }
    if triplet.len() != 3 {
        return Err(tokenizer.error(&format!(
            "must be a triplet expectde len 3 but got {}",
            triplet.len()
        )));
    }
    let triplet = (triplet[0], triplet[1], triplet[2]);

    Ok(OutList {
        name,
        triplet,
        out_type,
    })
}

fn set(
    tokenizer: &mut Tokenizer<Token>,
    out_list: &Option<Vec<OutList>>,
) -> Result<Instruction, Error> {
    let var = get_str(tokenizer)?;
    let mut out_type = if let Some(out_list) = out_list {
        let mut found = None;
        for out in out_list {
            if out.name == var {
                found = Some(out.out_type.clone());
                break;
            }
        }
        if let Some(found) = found {
            found
        } else {
            return Err(tokenizer.error(&format!(
                "var {} was not initalized in the output-list",
                var
            )));
        }
    } else {
        return Err(tokenizer.error("no output-list supplied"));
    };

    if let OutType::Binary(_) = out_type {
        if tokenizer.next_is(Token::Number(0)) {
            if let Token::Number(num) = tokenizer.expect(Token::Number(0))? {
                if num == 0 {
                    out_type = OutType::Binary(vec![false]);
                } else if num == 1 {
                    out_type = OutType::Binary(vec![true]);
                } else {
                    return Err(tokenizer.error(&format!("{} is not a binary number", num)));
                }
            } else {
                unreachable!();
            }
        } else {
            let b_num = get_str(tokenizer)?;
            if &b_num[0..2] != "B%" {
                return Err(tokenizer.error(&format!(
                    "binary number {} dose not to start with B%",
                    b_num
                )));
            }

            let mut result = Vec::new();
            for c in b_num.chars().skip(2) {
                if c == '0' {
                    result.push(false);
                } else if c == '1' {
                    result.push(true);
                } else {
                    return Err(tokenizer.error(&format!("{} is not a binary number", b_num)));
                }
            }

            out_type = OutType::Binary(result);
        }
    } else if let OutType::Decimal(_) = out_type {
        out_type = OutType::Decimal(0);
    } else {
        return Err(tokenizer.error("no output-list supplied"));
    }

    Ok(Instruction::Set(var, out_type))
}

fn repeat(tokenizer: &mut Tokenizer<Token>) -> Result<Instruction, Error> {
    let num = if tokenizer.next_is(Token::Number(0)) {
        if let Token::Number(num) = tokenizer.expect(Token::Number(0))? {
            Some(num)
        } else {
            unreachable!();
        }
    } else {
        None
    };
    tokenizer.expect(Token::CurlyOpen)?;
    Ok(Instruction::BeginRepeat(num))
}

fn echo(tokenizer: &mut Tokenizer<Token>) -> Result<Instruction, Error> {
    if let Token::StringLit(string) = tokenizer.expect(Token::StringLit(String::new()))? {
        let string = string.replace(DOUBLE_QUOTES, "");

        Ok(Instruction::Echo(string))
    } else {
        unreachable!();
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,

    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    #[regex(r"(/\*([^*]|\*[^/])*\*/)|(//[^\r\n]*(\r\n|\n)?)", ignore)]
    Ignore((usize, Option<String>)),

    // DOUBLE_QUOTES
    #[regex(r"#!#[a-zA-Z_$0-9%. ]*#!#", name, priority = 6)]
    StringLit(String),

    #[regex(r"[a-zA-Z_$0-9%.]*", name)] //  priority = 5
    String(String),
    #[regex(r"[0-9]+", number)]
    Number(usize),

    #[error]
    Unknown,
}

fn name(lexer: &mut Lexer<Token>) -> Option<String> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 1];
    Some(slice.to_string())
}

fn number(lexer: &mut Lexer<Token>) -> Option<usize> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 1];
    if let Ok(val) = slice.parse() {
        Some(val)
    } else {
        None
    }
}

fn ignore(lexer: &mut Lexer<Token>) -> Option<(usize, Option<String>)> {
    let slice = lexer.slice();
    match slice {
        " " => Some((0, None)),
        "\n" => Some((0, Some("newline".to_string()))),
        "\t" => Some((0, None)),
        _ => Some((slice.matches("\n").count(), Some(slice.to_string()))),
    }
}

impl TypeEq for Token {
    fn type_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Ignore(_), Token::Ignore(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::StringLit(_), Token::StringLit(_)) => true,
            _ => self == other,
        }
    }
}
