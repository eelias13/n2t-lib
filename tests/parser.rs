use n2t_vm::{parse, Instruction, Segment};

#[test]
fn push() {
    let code = r"
    push constant 2
    push this 4
    push constant -13
    ";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::PushConst(2),
            Instruction::Push(Segment::This, 4),
            Instruction::PushConst(-13),
        ])
    );
}

#[test]
fn pop() {
    let code = r"
    pop that 1
    pop local 3
    pop argument 2
    ";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::Pop(Segment::That, 1),
            Instruction::Pop(Segment::Local, 3),
            Instruction::Pop(Segment::Argument, 2),
        ])
    );
}

#[test]
fn alu() {
    let code = r"
    add
    sub
    and
    or
    eq
    gt
    lt
    neg
    not
    ";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::Add,
            Instruction::Sub,
            Instruction::And,
            Instruction::Or,
            Instruction::Eq,
            Instruction::Gt,
            Instruction::Lt,
            Instruction::Neg,
            Instruction::Not,
        ])
    );
}

#[test]
fn goto() {
    let code = r"
    lable loop
    goto loop
    if-goto loop
    ";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::Lable("loop".to_string()),
            Instruction::Goto(0),
            Instruction::IfGoto(0),
        ])
    );
}
