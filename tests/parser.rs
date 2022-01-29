use n2t_lib::{parse, Instruction, Segment};

#[test]
fn push() {
    let code = r"
    push constant 2
    push this 4
    push constant -13";

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
    pop argument 2";

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
    not";

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
    label loop
    goto loop
    if-goto loop";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::Label("loop".to_string()),
            Instruction::Goto(0),
            Instruction::IfGoto(0),
        ])
    );
}

#[test]
fn function() {
    let code = r"
    function main 3
    return
    call main 1";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::Function("main".to_string(), 3),
            Instruction::Return,
            Instruction::Call(0, 3),
        ])
    );
}

#[test]
fn fibonacci() {
    let code = r"
    function Main.fibonacci 0
    push argument 0
    push constant 2
    lt
    if-goto IF_TRUE
    goto IF_FALSE
    label IF_TRUE  
    push argument 0        
    return
    label IF_FALSE        
    push argument 0
    push constant 2
    sub
    call Main.fibonacci 1 
    push argument 0
    push constant 1
    sub
    call Main.fibonacci 1  
    add                    
    return";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::Function("Main.fibonacci".to_string(), 0),
            Instruction::Push(Segment::Argument, 0),
            Instruction::PushConst(2),
            Instruction::Lt,
            Instruction::IfGoto(6),
            Instruction::Goto(9),
            Instruction::Label("IF_TRUE".to_string()),
            Instruction::Push(Segment::Argument, 0),
            Instruction::Return,
            Instruction::Label("IF_FALSE".to_string()),
            Instruction::Push(Segment::Argument, 0),
            Instruction::PushConst(2),
            Instruction::Sub,
            Instruction::Call(0, 0),
            Instruction::Push(Segment::Argument, 0),
            Instruction::PushConst(1),
            Instruction::Sub,
            Instruction::Call(0, 0),
            Instruction::Add,
            Instruction::Return
        ])
    );
}

#[test]
fn simple_function() {
    let code = r"
    function SimpleFunction.test 2
    push local 0
    push local 1
    add
    not
    push argument 0
    add
    push argument 1
    sub
    return";

    assert_eq!(
        parse(code),
        Ok(vec![
            Instruction::Function("SimpleFunction.test".to_string(), 2),
            Instruction::Push(Segment::Local, 0),
            Instruction::Push(Segment::Local, 1),
            Instruction::Add,
            Instruction::Not,
            Instruction::Push(Segment::Argument, 0),
            Instruction::Add,
            Instruction::Push(Segment::Argument, 1),
            Instruction::Sub,
            Instruction::Return
        ])
    );
}
