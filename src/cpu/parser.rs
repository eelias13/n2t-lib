use super::{CPUInstruction, Comp, Dest};
use logos::{Lexer, Logos};
use std::collections::HashMap;
use tokenizer::{Error, Tokenizer, TypeEq};

pub fn asm2ml(asm: Vec<CPUInstruction>) -> Vec<u16> {
    let mut ml = Vec::new();
    for instrc in asm {
        match instrc {
            CPUInstruction::AInstruc(val) => ml.push((val & 0b0111111111111111) as u16),
            CPUInstruction::CInstruc(comp, dest, jump) => {
                ml.push(0b111 << 13 | (comp as u16) << 6 | (dest as u16) << 3 | jump as u16)
            }
        }
    }
    ml
}

pub fn ml2asm(ml: Vec<u16>) -> Result<Vec<CPUInstruction>, String> {
    let mut asm = Vec::new();
    for i in ml {
        if i & 0b1000000000000000 == 0b1000000000000000 {
            let jump = (((i & 0b0000000000000111) >> 0) as u8).try_into()?;
            let dest = (((i & 0b0000000000111000) >> 3) as u8).try_into()?;
            let comp = (((i & 0b0001111111000000) >> 6) as u8).try_into()?;
            asm.push(CPUInstruction::CInstruc(comp, dest, jump))
        } else {
            asm.push(CPUInstruction::AInstruc(i as i16));
        }
    }
    Ok(asm)
}

pub fn str2ml(hack: &str) -> Result<Vec<u16>, String> {
    let mut ml = Vec::new();
    let hack = hack.replace("\r\n", "\n");
    let hack = hack.replace("\t", "");
    let hack = hack.replace(" ", "");

    for (i, line) in hack.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut bool_vec = Vec::new();
        for c in line.chars() {
            if c == '1' {
                bool_vec.push(true);
            } else if c == '0' {
                bool_vec.push(false);
            } else {
                return Err(format!("unexpectet char {} in line {}", c, i));
            }
        }

        if bool_vec.len() != 16 {
            return Err(format!(
                "unexpectet len {} in line {} expectet 16 0 or 1's",
                bool_vec.len(),
                i
            ));
        }

        let mut num = 0;
        for (i, &val) in bool_vec.iter().enumerate() {
            if val {
                num += 1 << (15 - i);
            }
        }

        ml.push(num);
    }

    Ok(ml)
}

pub fn parse(code: &str) -> Result<Vec<CPUInstruction>, Error> {
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

    labals.insert(String::from("SP"), crate::SP);
    labals.insert(String::from("LCL"), crate::LCL);
    labals.insert(String::from("ARG"), crate::ARG);
    labals.insert(String::from("THIS"), crate::THIS);
    labals.insert(String::from("THAT"), crate::THAT);
    labals.insert(String::from("SCREEN"), crate::SCREEN);
    labals.insert(String::from("KBD"), crate::KBD);

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
                asm.push(CPUInstruction::AInstruc(0));
            }
            Token::Number(val) => asm.push(CPUInstruction::AInstruc(val as i16)),
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

        asm[line] = CPUInstruction::AInstruc(val as i16);
    }

    Ok(asm)
}

fn get_token(token: Option<Token>, tokenizer: &Tokenizer<Token>) -> Result<Token, Error> {
    if let Some(token) = token {
        Ok(token)
    } else {
        Err(tokenizer.error("unexpected end of file"))
    }
}

fn c_instruc(tokenizer: &mut Tokenizer<Token>) -> Result<CPUInstruction, Error> {
    let token = get_token(tokenizer.current(), tokenizer)?;

    let dest;
    let comp;
    let jump;

    let t = get_token(tokenizer.peek(), tokenizer)?;
    if t == Token::Eq {
        dest = get_dest(token.clone());
        tokenizer.expect_next(Token::Eq)?;
        comp = match get_comp(get_token(tokenizer.next(), tokenizer)?) {
            Ok(val) => val,
            Err(msg) => return Err(tokenizer.error(&msg)),
        }
    } else {
        dest = Dest::Null;
        comp = match get_comp(token) {
            Ok(val) => val,
            Err(msg) => return Err(tokenizer.error(&msg)),
        }
    }

    if let Some(token) = tokenizer.peek() {
        if token == Token::Semic {
            tokenizer.expect_next(Token::Semic)?;
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

    Ok(CPUInstruction::CInstruc(comp, dest, jump))
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
        _ => Err(format!("expected Comp but got {:?}", token)),
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
    #[token("\r\n", ignore)]
    #[token("\n", ignore)]
    #[regex(r"(/\*([^*]|\*[^/])*\*/)|(//[^\r\n]*(\r\n|\n)?)", ignore)]
    Ignore((usize, Option<String>)),

    #[regex(r"@[a-zA-Z][a-zA-Z|0-9|\.|_|$]+", name)]
    Name(String),
    #[regex(r"@[0-9]+", number)]
    Number(usize),
    #[regex(r"\([a-zA-Z][a-zA-Z|0-9|\.|_|$]+\)", label)]
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
    let slice = &slice[1..slice.len()];
    Some(slice.to_string())
}

fn label(lexer: &mut Lexer<Token>) -> Option<String> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 1];
    Some(slice.to_string())
}

fn number(lexer: &mut Lexer<Token>) -> Option<usize> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len()];
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
        "\n" => Some((1, Some("newline".to_string()))),
        "\r\n" => Some((1, Some("newline".to_string()))),
        "\t" => Some((0, None)),
        _ => Some((slice.matches("\n").count(), Some(slice.to_string()))),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokens() {
        let code = r"
    @abc
    @5
    (test)
    A=M+D
    ";
        let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);
        assert_eq!(tokenizer.next(), Some(Token::Name("abc".to_string())));
        assert_eq!(tokenizer.next(), Some(Token::Number(5)));
        assert_eq!(tokenizer.next(), Some(Token::Labal("test".to_string())));
        assert_eq!(tokenizer.next(), Some(Token::A));
        assert_eq!(tokenizer.next(), Some(Token::Eq));
        assert_eq!(tokenizer.next(), Some(Token::DPulsM));
    }
}
