use super::VMInstruction;

use crate::cpu::{CPUInstruction, Comp, Dest, Jump};

pub fn vm2asm(instrucs: Vec<VMInstruction>) -> Vec<CPUInstruction> {
    let mut asm = Vec::new();

    for instruc in instrucs {
        match instruc {
            VMInstruction::Push(seg, value) => (),
            VMInstruction::Pop(seg, value) => (),
            VMInstruction::PushConst(value) => push_const(&mut asm, value),
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
            VMInstruction::Goto(addr) => {
                asm.push(CPUInstruction::CInstruc(Comp::Zero, Dest::Null, Jump::JMP))
            }
            VMInstruction::IfGoto(addr) => (),
            VMInstruction::Function(name, n_var) => (),
            VMInstruction::Call(addr, n_arg) => call(&mut asm, addr, n_arg),
            VMInstruction::Return => (),
        }
    }

    asm
}

fn push() {}
fn push_const(asm: &mut Vec<CPUInstruction>, value: i16) {
    asm.push(CPUInstruction::AInstruc(value)); // @val
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::A, Jump::Null)); // A=D
    asm.push(CPUInstruction::AInstruc(0)); // @SP
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::D, Jump::Null)); // M=D
    asm.push(CPUInstruction::AInstruc(0)); // @SP
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
    asm.push(CPUInstruction::AInstruc(0)); // @SP
    asm.push(CPUInstruction::CInstruc(Comp::M, Dest::A, Jump::Null)); // A=M
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); // M=D

    // push LCL
    asm.push(CPUInstruction::AInstruc(1)); //@LOC
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(0)); //@SP
    asm.push(CPUInstruction::CInstruc(
        Comp::APulsOne,
        Dest::AM,
        Jump::Null,
    )); // AM=A+1
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // push ARG
    asm.push(CPUInstruction::AInstruc(2)); //@ARG
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(0)); //@SP
    asm.push(CPUInstruction::CInstruc(
        Comp::APulsOne,
        Dest::AM,
        Jump::Null,
    )); // AM=A+1
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // push THIS
    asm.push(CPUInstruction::AInstruc(3)); //@THIS
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(0)); //@SP
    asm.push(CPUInstruction::CInstruc(
        Comp::APulsOne,
        Dest::AM,
        Jump::Null,
    )); // AM=A+1
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // push THAT
    asm.push(CPUInstruction::AInstruc(4)); //@THAT
    asm.push(CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null)); // D=A
    asm.push(CPUInstruction::AInstruc(0)); //@SP
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
    asm.push(CPUInstruction::AInstruc(0)); //@SP
    asm.push(CPUInstruction::CInstruc(Comp::AMinusD, Dest::D, Jump::Null)); //D=A-D
    asm.push(CPUInstruction::AInstruc(2)); //@ARG
    asm.push(CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null)); //M=D

    // goto addr
    asm.push(CPUInstruction::AInstruc(addr as i16)); // @addr
    asm.push(CPUInstruction::CInstruc(Comp::Zero, Dest::Null, Jump::JMP)); //0;JMP
                                                                           // (RET) at 34
}
