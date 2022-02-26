mod jack_vm;
mod parser;

pub use jack_vm::JackVM;
pub use parser::parse;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
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
pub enum VMInstruction {
    Push(Segment, i16),
    Pop(Segment, i16),
    PushConst(i16),
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
    Label(String),
    Goto(usize),
    IfGoto(usize),
    Function(String, usize),
    Call(usize, usize),
    Return,
}
