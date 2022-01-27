use crate::lexer::{Error, Token, Tokenizer};
use crate::{Instruction, Segment};
use std::collections::HashMap;

pub fn parse(code: &str) -> Result<Vec<Instruction>, Error> {
    let mut lexer ;//= Token::lexer(code);
    let mut result = Vec::new();

    let mut lables = HashMap::new();
    let mut functions = HashMap::new();

    while let Some(token) = lexer.next() {
        match token {
            Token::Add => result.push(Instruction::Add),
            Token::Sub => result.push(Instruction::Sub),
            Token::And => result.push(Instruction::And),
            Token::Or => result.push(Instruction::Or),
            Token::Eq => result.push(Instruction::Eq),
            Token::Gt => result.push(Instruction::Gt),
            Token::Lt => result.push(Instruction::Lt),
            Token::Not => result.push(Instruction::Not),
            Token::Neg => result.push(Instruction::Neg),

            Token::Push => pop(&mut lexer, &mut result)?,
            Token::Pop => pop(&mut lexer, &mut result)?,

            Token::Lable => lable(&mut lexer, &mut result, &mut lables)?,
            Token::Goto => goto(&mut lexer, &mut result, &mut lables, false)?,
            Token::IfGoto => goto(&mut lexer, &mut result, &mut lables, true)?,

            Token::Function => function(&mut lexer, &mut result, &mut functions)?,
            Token::Call => call(&mut lexer, &mut result, &mut functions)?,
            Token::Return => result.push(Instruction::Return),

            _ => {
                Error::expect_multi(
                    Some(token),
                    vec![
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
                        Token::Lable,
                        Token::Goto,
                        Token::IfGoto,
                        Token::Function,
                        Token::Call,
                        Token::Return,
                    ],
                    &lexer,
                )?;
                unreachable!();
            }
        }
    }
    Ok(result)
}

fn function(
    lexer: &mut Lexer<Token>,
    result: &mut Vec<Instruction>,
    functions: &mut HashMap<String, (usize, usize)>,
) -> Result<(), Error> {
    Ok(())
}

fn call(
    lexer: &mut Lexer<Token>,
    result: &mut Vec<Instruction>,
    functions: &mut HashMap<String, (usize, usize)>,
) -> Result<(), Error> {
    Ok(())
}

fn lable(
    lexer: &mut Lexer<Token>,
    result: &mut Vec<Instruction>,
    lables: &mut HashMap<String, usize>,
) -> Result<(), Error> {
    Ok(())
}

fn goto(
    lexer: &mut Lexer<Token>,
    result: &mut Vec<Instruction>,
    lables: &mut HashMap<String, usize>,
    is_if: bool,
) -> Result<(), Error> {
    Ok(())
}

fn pop(lexer: &mut Lexer<Token>, result: &mut Vec<Instruction>) -> Result<(), Error> {
    let seg = get_seg(lexer.next(), lexer)?;
    let addr = Error::expect(lexer.next(), Token::Number(0), lexer)?;
    if let Token::Number(addr) = addr {
        result.push(Instruction::Pop(seg, addr));
        Ok(())
    } else {
        unreachable!();
    }
}

fn push(lexer: &mut Lexer<Token>, result: &mut Vec<Instruction>) -> Result<(), Error> {
    let token = lexer.next();
    if token == Some(Token::Constant) {
        let token = lexer.next();
        let value = if token == Some(Token::MinusSign) {
            get_num(lexer.next(), lexer)? as isize * -1
        } else {
            get_num(token, lexer)? as isize
        };

        result.push(Instruction::PushConst(value));
        Ok(())
    } else {
        let seg = get_seg(token, lexer)?;
        let addr = get_num(lexer.next(), lexer)?;
        result.push(Instruction::Push(seg, addr));
        Ok(())
    }
}

fn get_num(token: Option<Token>, lexer: &mut Lexer<Token>) -> Result<usize, Error> {
    if let Token::Number(num) = Error::expect(lexer.next(), Token::Number(0), lexer)? {
        Ok(num)
    } else {
        unreachable!();
    }
}

fn get_seg(token: Option<Token>, lexer: &Lexer<Token>) -> Result<Segment, Error> {
    let token = Error::expect_multi(
        token,
        vec![
            Token::This,
            Token::That,
            Token::Local,
            Token::Argument,
            Token::Static,
            Token::Pointer,
            Token::Temp,
        ],
        lexer,
    )?;

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
