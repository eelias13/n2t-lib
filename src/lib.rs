mod jack_vm;
mod parser;

pub use jack_vm::JackVM;
pub use parser::parse;

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    This,
    That,
    Local,
    Argument,
    Static,
    Pointer,
    Temp,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Push(Segment, usize),
    Pop(Segment, usize),
    PushConst(isize),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Lable(String),
    Goto(usize),
    IfGoto(usize),
    Function(String, usize),
    Call(usize, Vec<isize>),
    Return,
}
