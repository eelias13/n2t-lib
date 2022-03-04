use n2t_lib::cpu::{parse, CPUInstruction, Comp, Dest, Jump};

#[test]
fn jmp() {
    let code = r"
        0;JMP
        A=D
        M=M+D;JLT
    ";

    assert_eq!(
        parse(code),
        Ok(vec![
            CPUInstruction::CInstruc(Comp::Zero, Dest::Null, Jump::JMP),
            CPUInstruction::CInstruc(Comp::D, Dest::A, Jump::Null),
            CPUInstruction::CInstruc(Comp::DPulsM, Dest::M, Jump::JLT)
        ])
    );
}
