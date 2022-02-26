use super::{Segment, VMInstruction};
use crate::cpu::{CPUInstruction, Comp, Dest};
use logos::{Lexer, Logos};
use std::collections::HashMap;
use tokenizer::{Error, Tokenizer, TypeEq};

pub fn vm2asm(instrucs: Vec<VMInstruction>) -> Vec<CPUInstruction> {
    let mut asm = Vec::new();

    for instruc in instrucs {
        match instruc {
            VMInstruction::Push(seg, value) => (),
            VMInstruction::Pop(seg, value) => (),
            VMInstruction::PushConst(value) => (),
            VMInstruction::Add => (),
            VMInstruction::Sub => (),
            VMInstruction::Neg => (),
            VMInstruction::Eq => (),
            VMInstruction::Gt => (),
            VMInstruction::Lt => (),
            VMInstruction::And => (),
            VMInstruction::Or => (),
            VMInstruction::Not => (),
            VMInstruction::Label(name) => (),
            VMInstruction::Goto(addr) => asm.push(CPUInstruction::CInstruc(
                Comp::Zero,
                Dest::Null,
                crate::cpu::Jump::JMP,
            )),
            VMInstruction::IfGoto(addr) => (),
            VMInstruction::Function(name, n_var) => (),
            VMInstruction::Call(addr, n_arg) => (),
            VMInstruction::Return => (),
        }
    }

    asm
}

pub fn parse(code: &str) -> Result<Vec<VMInstruction>, Error> {
    let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);
    let mut result = Vec::new();

    let mut labels = HashMap::new();
    let mut functions = HashMap::new();

    let mut set_label = Vec::new();

    while let Some(token) = tokenizer.next() {
        match token {
            Token::Add => result.push(VMInstruction::Add),
            Token::Sub => result.push(VMInstruction::Sub),
            Token::And => result.push(VMInstruction::And),
            Token::Or => result.push(VMInstruction::Or),
            Token::Eq => result.push(VMInstruction::Eq),
            Token::Gt => result.push(VMInstruction::Gt),
            Token::Lt => result.push(VMInstruction::Lt),
            Token::Not => result.push(VMInstruction::Not),
            Token::Neg => result.push(VMInstruction::Neg),

            Token::Push => push(&mut tokenizer, &mut result)?,
            Token::Pop => pop(&mut tokenizer, &mut result)?,

            Token::Label => label(&mut tokenizer, &mut result, &mut labels)?,
            Token::Goto => goto(
                &mut tokenizer,
                &mut result,
                &mut labels,
                &mut set_label,
                false,
            )?,
            Token::IfGoto => goto(
                &mut tokenizer,
                &mut result,
                &mut labels,
                &mut set_label,
                true,
            )?,

            Token::Function => function(&mut tokenizer, &mut result, &mut functions)?,
            Token::Call => call(&mut tokenizer, &mut result, &mut functions)?,
            Token::Return => result.push(VMInstruction::Return),

            _ => {
                tokenizer.expect_multi(vec![
                    Token::Push,
                    Token::Pop,
                    Token::Add,
                    Token::Sub,
                    Token::And,
                    Token::Or,
                    Token::Eq,
                    Token::Gt,
                    Token::Lt,
                    Token::Not,
                    Token::Neg,
                    Token::Label,
                    Token::Goto,
                    Token::IfGoto,
                    Token::Function,
                    Token::Call,
                    Token::Return,
                ])?;
                unreachable!();
            }
        }
    }

    for (name, i) in set_label {
        if let Some(&addr) = labels.get(&name) {
            let addr = addr as usize;
            if result[i as usize] == VMInstruction::Goto(0) {
                result[i as usize] = VMInstruction::Goto(addr);
            } else if result[i as usize] == VMInstruction::IfGoto(0) {
                result[i as usize] = VMInstruction::IfGoto(addr);
            } else {
                unreachable!();
            }
        } else {
            return Err(tokenizer.error(&format!("can not finde label {}", name)));
        }
    }

    return Ok(result);
}

fn function(
    tokenizer: &mut Tokenizer<Token>,
    result: &mut Vec<VMInstruction>,
    functions: &mut HashMap<String, (usize, usize)>,
) -> Result<(), Error> {
    tokenizer.next();
    if let Token::Name(name) = tokenizer.expect(Token::Name(String::new()))? {
        let num = get_num(tokenizer)? as usize;
        functions.insert(name.clone(), (num, result.len()));
        result.push(VMInstruction::Function(name, num));
        return Ok(());
    } else {
        unreachable!();
    }
}

fn call(
    tokenizer: &mut Tokenizer<Token>,
    result: &mut Vec<VMInstruction>,
    functions: &mut HashMap<String, (usize, usize)>,
) -> Result<(), Error> {
    tokenizer.next();
    if let Token::Name(name) = tokenizer.expect(Token::Name(String::new()))? {
        let _num = get_num(tokenizer)?; // ????????????????????
        if let Some(&(argc, adder)) = functions.get(&name) {
            result.push(VMInstruction::Call(adder, argc));
            return Ok(());
        } else {
            return Err(tokenizer.error(&format!("can not finde labal {}", name)));
        }
    } else {
        unreachable!();
    }
}

fn label(
    tokenizer: &mut Tokenizer<Token>,
    result: &mut Vec<VMInstruction>,
    labels: &mut HashMap<String, u16>,
) -> Result<(), Error> {
    tokenizer.next();
    if let Some(Token::Name(name)) = tokenizer.current() {
        labels.insert(name.clone(), result.len() as u16);
        result.push(VMInstruction::Label(name));
        return Ok(());
    } else {
        tokenizer.expect(Token::Name(String::new()))?;
        unreachable!();
    }
}

fn goto(
    tokenizer: &mut Tokenizer<Token>,
    result: &mut Vec<VMInstruction>,
    labels: &mut HashMap<String, u16>,
    set_label: &mut Vec<(String, u16)>,
    is_if: bool,
) -> Result<(), Error> {
    tokenizer.next();
    if let Some(Token::Name(name)) = tokenizer.current() {
        if let Some(&addr) = labels.get(&name) {
            result.push(if is_if {
                VMInstruction::IfGoto(addr as usize)
            } else {
                VMInstruction::Goto(addr as usize)
            });
        } else {
            set_label.push((name, result.len() as u16));
            result.push(if is_if {
                VMInstruction::IfGoto(0)
            } else {
                VMInstruction::Goto(0)
            });
        }
        return Ok(());
    } else {
        tokenizer.expect(Token::Name(String::new()))?;
        unreachable!();
    }
}

fn pop(tokenizer: &mut Tokenizer<Token>, result: &mut Vec<VMInstruction>) -> Result<(), Error> {
    tokenizer.next();
    let seg = get_seg(tokenizer)?;
    let addr = get_num(tokenizer)?;
    result.push(VMInstruction::Pop(seg, addr));
    Ok(())
}

fn push(tokenizer: &mut Tokenizer<Token>, result: &mut Vec<VMInstruction>) -> Result<(), Error> {
    tokenizer.next();

    if tokenizer.is(Token::Constant) {
        tokenizer.next();
        let value = get_num(tokenizer)? as isize;
        result.push(VMInstruction::PushConst(value as i16));
        Ok(())
    } else {
        let seg = get_seg(tokenizer)?;
        let addr = get_num(tokenizer)?;
        result.push(VMInstruction::Push(seg, addr));
        Ok(())
    }
}

fn get_num(tokenizer: &mut Tokenizer<Token>) -> Result<i16, Error> {
    if let Some(Token::Number(num)) = tokenizer.current() {
        return Ok(num);
    } else {
        tokenizer.expect(Token::Number(0))?;
        unreachable!();
    }
}

fn get_seg(tokenizer: &mut Tokenizer<Token>) -> Result<Segment, Error> {
    let token = tokenizer.expect_multi(vec![
        Token::This,
        Token::That,
        Token::Local,
        Token::Argument,
        Token::Static,
        Token::Pointer,
        Token::Temp,
    ])?;

    Ok(match token {
        Token::This => Segment::This,
        Token::That => Segment::That,
        Token::Local => Segment::Local,
        Token::Argument => Segment::Argument,
        Token::Static => Segment::Static,
        Token::Pointer => Segment::Pointer,
        Token::Temp => Segment::Temp,
        _ => unreachable!(),
    })
}

#[derive(Logos, Debug, Clone)]
enum Token {
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

    #[token("label")]
    Label,
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
    #[regex(r"(/\*([^*]|\*[^/])*\*/)|(//[^\r\n]*(\r\n|\n)?)", ignore)] 
    Ignore((usize, Option<String>)),

    #[regex(r"[a-zA-Z][a-zA-Z|0-9|\.|_]+", |lexer| lexer.slice().parse())]
    Name(String),
    #[regex(r"[0-9]+", |lexer| lexer.slice().parse())]
    Number(i16),

    #[error]
    Unknown,
}

impl TypeEq for Token {
    fn type_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Push, Token::Push) => true,
            (Token::Pop, Token::Pop) => true,
            (Token::Add, Token::Add) => true,
            (Token::Sub, Token::Sub) => true,
            (Token::Neg, Token::Neg) => true,
            (Token::Eq, Token::Eq) => true,
            (Token::Gt, Token::Gt) => true,
            (Token::Lt, Token::Lt) => true,
            (Token::And, Token::And) => true,
            (Token::Or, Token::Or) => true,
            (Token::Not, Token::Not) => true,
            (Token::Label, Token::Label) => true,
            (Token::Goto, Token::Goto) => true,
            (Token::IfGoto, Token::IfGoto) => true,
            (Token::Function, Token::Function) => true,
            (Token::Call, Token::Call) => true,
            (Token::Return, Token::Return) => true,
            (Token::This, Token::This) => true,
            (Token::That, Token::That) => true,
            (Token::Local, Token::Local) => true,
            (Token::Argument, Token::Argument) => true,
            (Token::Static, Token::Static) => true,
            (Token::Pointer, Token::Pointer) => true,
            (Token::Temp, Token::Temp) => true,
            (Token::Constant, Token::Constant) => true,
            (Token::Ignore(_), Token::Ignore(_)) => true,
            (Token::Name(_), Token::Name(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            _ => false,
        }
    }
}

fn ignore(lexer: &mut Lexer<Token>) -> Option<(usize, Option<String>)> {
    let slice = lexer.slice();
    match slice {
        " " => Some((0, None)),
        "\n" => Some((1, Some("newline".to_string()))),
        "\t" => Some((0, None)),
        _ => Some((slice.matches("\n").count(), Some(slice.to_string()))),
    }
}
