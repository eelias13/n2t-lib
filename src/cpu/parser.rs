use super::{Comp, Dest, Instruction};
use logos::{Lexer, Logos};
use std::collections::HashMap;
use tokenizer::{Error, Tokenizer, TypeEq};

pub fn asm2ml(asm: Vec<Instruction>) -> Vec<u16> {
    let mut ml = Vec::new();
    for instrc in asm {
        match instrc {
            Instruction::AInstruc(val) => ml.push((val & 0 << 15) as u16),
            Instruction::CInstruc(comp, dest, jump) => {
                ml.push(0b111 << 13 | (comp as u16) << 6 | (dest as u16) << 3 | jump as u16)
            }
        }
    }
    ml
}

pub fn ml2asm(ml: Vec<u16>) -> Result<Vec<Instruction>, String> {
    let mut asm = Vec::new();
    for i in ml {
        if i & 0b1000000000000000 == 0b1000000000000000 {
            let comp = ((6 >> (i & 0b0001111111000000)) as u8).try_into()?;
            let dest = ((3 >> (i & 0b0000000000111000)) as u8).try_into()?;
            let jump = ((0 >> (i & 0b0000000000000111)) as u8).try_into()?;
            asm.push(Instruction::CInstruc(comp, dest, jump))
        } else {
            asm.push(Instruction::AInstruc(i as i16));
        }
    }
    Ok(asm)
}

pub fn parse(code: &str) -> Result<Vec<Instruction>, Error> {
    let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);
    let mut asm = Vec::new();

    let mut labals = HashMap::new();
    let mut names = Vec::new();

    labals.insert(String::from("R0"), 0);
    labals.insert(String::from("R1"), 1);
    labals.insert(String::from("R2"), 2);
    labals.insert(String::from("R3"), 3);
    labals.insert(String::from("R4"), 4);
    labals.insert(String::from("R5"), 5);
    labals.insert(String::from("R6"), 6);
    labals.insert(String::from("R7"), 7);
    labals.insert(String::from("R8"), 8);
    labals.insert(String::from("R9"), 9);
    labals.insert(String::from("R10"), 10);
    labals.insert(String::from("R11"), 11);
    labals.insert(String::from("R12"), 12);
    labals.insert(String::from("R13"), 13);
    labals.insert(String::from("R14"), 14);
    labals.insert(String::from("R15"), 15);
    labals.insert(String::from("R16"), 16);

    while let Some(token) = tokenizer.next() {
        match token {
            Token::Labal(labal) => {
                if let Some(val) = labals.insert(labal.clone(), asm.len()) {
                    return Err(tokenizer.error(&format!(
                        "the label {} has already been set to {}",
                        labal, val
                    )));
                }
            }
            Token::Name(name) => {
                names.push((name, asm.len()));
                asm.push(Instruction::AInstruc(0));
            }
            Token::Number(val) => asm.push(Instruction::AInstruc(val as i16)),
            _ => asm.push(c_instruc(&mut tokenizer)?),
        }
    }

    // resolve labels
    let mut var_count = 16;
    for (name, line) in names {
        let val = if let Some(&num) = labals.get(&name) {
            num
        } else {
            labals.insert(name, var_count);
            var_count += 1;
            var_count - 1
        };

        asm[line] = Instruction::AInstruc(val as i16);
    }

    Ok(asm)
}

fn c_instruc(tokenizer: &mut Tokenizer<Token>) -> Result<Instruction, Error> {
    let token = tokenizer.current().unwrap();

    let dest;
    let comp;
    let jump;

    if let Some(t) = tokenizer.peek() {
        if t == Token::Eq {
            dest = get_dest(token.clone());
            tokenizer.next();
        } else {
            dest = Dest::Null;
        }
    } else {
        return Err(tokenizer.error("unexpected end of file"));
    }

    if let Some(token) = tokenizer.next() {
        comp = match get_comp(token) {
            Ok(val) => val,
            Err(msg) => return Err(tokenizer.error(&msg)),
        }
    } else {
        return Err(tokenizer.error("unexpected end of file"));
    }

    if let Some(token) = tokenizer.peek() {
        if token == Token::Semic {
            tokenizer.next();
            if let Some(token) = tokenizer.next() {
                jump = get_jump(token);
            } else {
                return Err(tokenizer.error("expected jump after semicolon"));
            }
        } else {
            jump = super::Jump::Null;
        }
    } else {
        jump = super::Jump::Null;
    }

    Ok(Instruction::CInstruc(comp, dest, jump))
}

fn get_dest(token: Token) -> Dest {
    match token {
        Token::A => Dest::A,
        Token::D => Dest::D,
        Token::M => Dest::M,
        Token::AD => Dest::AD,
        Token::AM => Dest::AM,
        Token::MD => Dest::MD,
        Token::AMD => Dest::AMD,
        _ => Dest::Null,
    }
}

fn get_jump(token: Token) -> super::Jump {
    match token {
        Token::JGT => super::Jump::JGT,
        Token::JEQ => super::Jump::JEQ,
        Token::JGE => super::Jump::JGE,
        Token::JLT => super::Jump::JLT,
        Token::JNE => super::Jump::JNE,
        Token::JLE => super::Jump::JLE,
        Token::JMP => super::Jump::JMP,
        _ => super::Jump::Null,
    }
}

fn get_comp(token: Token) -> Result<Comp, String> {
    match token {
        Token::Zero => Ok(Comp::Zero),
        Token::One => Ok(Comp::One),
        Token::MinusOne => Ok(Comp::MinusOne),
        Token::D => Ok(Comp::D),
        Token::A => Ok(Comp::A),
        Token::NotD => Ok(Comp::NotD),
        Token::NotA => Ok(Comp::NotA),
        Token::MinusD => Ok(Comp::MinusD),
        Token::MinusA => Ok(Comp::MinusA),
        Token::DPulsOne => Ok(Comp::DPulsOne),
        Token::APulsOne => Ok(Comp::APulsOne),
        Token::DMinusOne => Ok(Comp::DMinusOne),
        Token::AMinusOne => Ok(Comp::AMinusOne),
        Token::DPulsA => Ok(Comp::DPulsA),
        Token::DMinusA => Ok(Comp::DMinusA),
        Token::AMinusD => Ok(Comp::AMinusD),
        Token::DAndA => Ok(Comp::DAndA),
        Token::DOrA => Ok(Comp::DOrA),
        Token::M => Ok(Comp::M),
        Token::NotM => Ok(Comp::NotM),
        Token::MPlusOne => Ok(Comp::MPlusOne),
        Token::MMinusOne => Ok(Comp::MMinusOne),
        Token::DPulsM => Ok(Comp::DPulsM),
        Token::DMinusM => Ok(Comp::DMinusM),
        Token::MMinusD => Ok(Comp::MMinusD),
        Token::DAndM => Ok(Comp::DAndM),
        Token::DOrM => Ok(Comp::DOrM),
        _ => Err("expected Comp".to_string()),
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
enum Token {
    #[token("0")]
    Zero,
    #[token("1")]
    One,
    #[token("-1")]
    MinusOne,
    #[token("D", priority = 5)]
    D,
    #[token("A", priority = 5)]
    A,
    #[token("!D")]
    NotD,
    #[token("!A")]
    NotA,
    #[token("-D")]
    MinusD,
    #[token("-A")]
    MinusA,
    #[token("D+1")]
    DPulsOne,
    #[token("A+1")]
    APulsOne,
    #[token("A-1")]
    DMinusOne,
    #[token("D-1")]
    AMinusOne,
    #[token("D+A")]
    #[token("A+D")]
    DPulsA,
    #[token("D-A")]
    DMinusA,
    #[token("A-D")]
    AMinusD,
    #[token("D&A")]
    #[token("A&D")]
    DAndA,
    #[token("D|A")]
    #[token("A|D")]
    DOrA,
    #[token("M", priority = 5)]
    M,
    #[token("!M")]
    NotM,
    #[token("M+1")]
    MPlusOne,
    #[token("M-1")]
    MMinusOne,
    #[token("D+M")]
    #[token("M+D")]
    DPulsM,
    #[token("D-M")]
    DMinusM,
    #[token("M-D")]
    MMinusD,
    #[token("D&M")]
    #[token("M&D")]
    DAndM,
    #[token("D|M")]
    #[token("M|D")]
    DOrM,
    #[token("AD")]
    #[token("DA")]
    AD,
    #[token("AM")]
    #[token("MA")]
    AM,
    #[token("DM")]
    #[token("MD")]
    MD,
    #[token("AMD")]
    #[token("ADM")]
    #[token("DAM")]
    #[token("DMA")]
    #[token("MAD")]
    #[token("MDA")]
    AMD,

    #[token("JGT")]
    JGT,
    #[token("JEQ")]
    JEQ,
    #[token("JGE")]
    JGE,
    #[token("JLT")]
    JLT,
    #[token("JNE")]
    JNE,
    #[token("JLE")]
    JLE,
    #[token("JMP")]
    JMP,

    #[token("=")]
    Eq,
    #[token(";")]
    Semic,

    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    Ignore((usize, Option<String>)),

    #[regex(r"@[a-zA-Z][a-zA-Z|0-9|\.|_]+", name)]
    Name(String),
    #[regex(r"@[0-9]+", number)]
    Number(usize),
    #[regex(r"\([a-zA-Z][a-zA-Z|0-9|\.|_]+\)", labal)]
    Labal(String),

    #[error]
    Unknown,
}

impl TypeEq for Token {
    fn type_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Ignore(_), Token::Ignore(_)) => true,
            (Token::Name(_), Token::Name(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::Labal(_), Token::Labal(_)) => true,
            _ => self == other,
        }
    }
}

fn name(lexer: &mut Lexer<Token>) -> Option<String> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 1];
    Some(slice.to_string())
}

fn labal(lexer: &mut Lexer<Token>) -> Option<String> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 2];
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
        _ => Some((0, Some(slice.to_string()))),
    }
}
