use super::{Segment, VMInstruction};
use crate::cpu::{CPUInstruction, Comp, Dest, Jump};

pub fn vm2asm(instrucs: Vec<VMInstruction>) -> Vec<CPUInstruction> {
    let mut asm = Vec::new();

    for instruc in instrucs {
        match instruc {
            VMInstruction::Push(seg, value) => {
                let value = seg2addr(seg) as i16 + value;
                asm.push(CPUInstruction::AInstruc(value)); // @value
                asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
                push(&mut asm);
            }
            VMInstruction::Pop(seg, value) => {
                todo!();
            }
            VMInstruction::PushConst(value) => {
                asm.push(CPUInstruction::AInstruc(value)); // @value
                push(&mut asm);
            }
            VMInstruction::Add => {
                top2da(&mut asm);
                asm.push(CPUInstruction::CInstruc(Comp::DPulsA, Dest::D, Jump::Null)); //D=D+A
                asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
                asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
                asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null));
                // M=D
            }
            VMInstruction::Sub => {
                top2da(&mut asm);
                asm.push(CPUInstruction::CInstruc(Comp::AMinusD, Dest::D, Jump::Null)); //D=A-D
                asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
                asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
                asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null));
                // M=D
            }
            VMInstruction::Neg => {
                todo!();
            }
            VMInstruction::Eq => {
                top2da(&mut asm);
                todo!();
            }
            VMInstruction::Gt => {
                top2da(&mut asm);
                todo!();
            }
            VMInstruction::Lt => {
                top2da(&mut asm);
                todo!();
            }
            VMInstruction::And => {
                top2da(&mut asm);
                asm.push(CPUInstruction::CInstruc(Comp::DAndA, Dest::D, Jump::Null)); //D=D&A
                asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
                asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
                asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null));
                // M=D
            }
            VMInstruction::Or => {
                top2da(&mut asm);
                asm.push(CPUInstruction::CInstruc(Comp::DOrA, Dest::D, Jump::Null)); //D=D|A
                asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
                asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
                asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null));
                // M=D
            }
            VMInstruction::Not => {
                todo!();
            }
            VMInstruction::Label(name) => (),
            VMInstruction::Goto(addr) => {
                asm.push(CPUInstruction::AInstruc(addr as i16)); // @addr
                asm.push(CPUInstruction::CInstruc(Comp::Zero, Dest::Null, Jump::JMP));
                // 0;JMP
            }
            VMInstruction::IfGoto(addr) => {
                todo!();
            }
            VMInstruction::Function(name, n_var) => {
                todo!();
            }
            VMInstruction::Call(addr, n_arg) => call(&mut asm, addr, n_arg),
            VMInstruction::Return => {
                todo!();
            }
        }
    }

    asm
}
/// moves the top stack value in to the d regester and the value below to the a regester
/// Note : after using this you have to decrement the stack prt (SP)
fn top2da(asm: &mut Vec<CPUInstruction>) {
    // D=*SP
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); //@SP
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::D, Jump::Null)); // D=M

    // SP--
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
    asm.push(CPUInstruction::CInstruc(
        Comp::MMinusOne,
        Dest::AM,
        Jump::Null,
    )); // AM=M-1

    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
}

/// moves the top stack value in to the a regester
fn top2a(asm: &mut Vec<CPUInstruction>) {
    top2d(asm);
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::A, Jump::Null)); // A=D
}

/// moves the top stack value in to the d regester
fn top2d(asm: &mut Vec<CPUInstruction>) {
    // D=*SP
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); //@SP
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::D, Jump::Null)); // D=M

    // SP--
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
    asm.push(CPUInstruction::CInstruc(
        Comp::MMinusOne,
        Dest::M,
        Jump::Null,
    )); // M=M-1
}

fn seg2addr(seg: Segment) -> usize {
    match seg {
        Segment::Argument => crate::ARG,
        Segment::Local => crate::LCL,
        Segment::Pointer => crate::PTR,
        Segment::Static => crate::STATIC,
        Segment::Temp => crate::TEMP,
        Segment::That => crate::THAT,
        Segment::This => crate::THIS,
    }
}

fn push(asm: &mut Vec<CPUInstruction>) {
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::D, Jump::Null)); // M=D
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
    asm.push(CPUInstruction::CInstruc(
        Comp::MPlusOne,
        Dest::M,
        Jump::Null,
    )); // M=M+1
}

fn call(asm: &mut Vec<CPUInstruction>, addr: usize, n_arg: usize) {
    // push RET
    asm.push(CPUInstruction::AInstruc(34)); // @RET
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); // @SP
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); // M=D

    // push LCL
    asm.push(CPUInstruction::AInstruc(crate::LCL as i16)); //@LCL
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); //@SP
    asm.push(CPUInstruction::CInstruc(
        Comp::APulsOne,
        Dest::AM,
        Jump::Null,
    )); // AM=A+1
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // push ARG
    asm.push(CPUInstruction::AInstruc(crate::ARG as i16)); //@ARG
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); //@SP
    asm.push(CPUInstruction::CInstruc(
        Comp::APulsOne,
        Dest::AM,
        Jump::Null,
    )); // AM=A+1
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // push THIS
    asm.push(CPUInstruction::AInstruc(crate::THIS as i16)); //@THIS
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); //@SP
    asm.push(CPUInstruction::CInstruc(
        Comp::APulsOne,
        Dest::AM,
        Jump::Null,
    )); // AM=A+1
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // push THAT
    asm.push(CPUInstruction::AInstruc(crate::THAT as i16)); //@THAT
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); //@SP
    asm.push(CPUInstruction::CInstruc(
        Comp::APulsOne,
        Dest::AM,
        Jump::Null,
    )); // AM=A+1
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D
    asm.push(CPUInstruction::CInstruc(
        Comp::MPlusOne,
        Dest::M,
        Jump::Null,
    )); // M=M+1

    // ARG=SP-5-n
    asm.push(CPUInstruction::AInstruc(5 + (n_arg as i16))); //@5+n_arg
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); //D=A
    asm.push(CPUInstruction::AInstruc(crate::SP as i16)); //@SP
    asm.push(CPUInstruction::CInstruc(Comp::AMinusD, Dest::D, Jump::Null)); //D=A-D
    asm.push(CPUInstruction::AInstruc(crate::ARG as i16)); //@ARG
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // goto addr
    asm.push(CPUInstruction::AInstruc(addr as i16)); // @addr
    asm.push(CPUInstruction::CInstruc(Comp::Zero, Dest::Null, Jump::JMP)); //0;JMP

    // (RET) at 34
}
