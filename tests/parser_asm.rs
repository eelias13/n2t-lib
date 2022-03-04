use n2t_lib::cpu::{asm2ml, ml2asm, parse, str2ml, CPUInstruction, Comp, Dest, Jump};

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

#[test]
fn test_edge_cases() {
    let code = r"
       @SP
       @THIS
       @THAT
       @ARG
       @LCL
       @a_.$AZ234
       A=A-1
       A=D-1
       (a_.$AZ234)
    ";

    let hack = r"
    0000000000000000
    0000000000000011
    0000000000000100
    0000000000000010
    0000000000000001
    0000000000001000
    1110110010100000
    1110001110100000
    ";

    let asm = parse(code).unwrap();
    assert_eq!(
        asm.clone(),
        vec![
            CPUInstruction::AInstruc(0),
            CPUInstruction::AInstruc(3),
            CPUInstruction::AInstruc(4),
            CPUInstruction::AInstruc(2),
            CPUInstruction::AInstruc(1),
            CPUInstruction::AInstruc(8),
            CPUInstruction::CInstruc(Comp::DMinusOne, Dest::A, Jump::Null),
            CPUInstruction::CInstruc(Comp::AMinusOne, Dest::A, Jump::Null)
        ]
    );

    let ml = str2ml(hack).unwrap();
    assert_eq!(ml.clone(), asm2ml(asm.clone()));
    assert_eq!(ml2asm(ml).unwrap(), asm);
}
