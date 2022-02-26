use n2t_lib::vm::{parse, Segment, VMInstruction};

#[test]
fn push() {
    let code = r#"
    push constant 2
    push this 4
    push constant 13"#;

    assert_eq!(
        parse(code),
        Ok(vec![
            VMInstruction::PushConst(2),
            VMInstruction::Push(Segment::This, 4),
            VMInstruction::PushConst(13),
        ])
    );
}

#[test]
fn pop() {
    let code = r#"
    pop that 1
    pop local 3
    pop argument 2"#;

    assert_eq!(
        parse(code),
        Ok(vec![
            VMInstruction::Pop(Segment::That, 1),
            VMInstruction::Pop(Segment::Local, 3),
            VMInstruction::Pop(Segment::Argument, 2),
        ])
    );
}

#[test]
fn alu() {
    let code = r#"
    add
    sub
    and
    or
    eq
    gt
    lt
    neg
    not"#;

    assert_eq!(
        parse(code),
        Ok(vec![
            VMInstruction::Add,
            VMInstruction::Sub,
            VMInstruction::And,
            VMInstruction::Or,
            VMInstruction::Eq,
            VMInstruction::Gt,
            VMInstruction::Lt,
            VMInstruction::Neg,
            VMInstruction::Not,
        ])
    );
}

#[test]
fn goto() {
    let code = r#"
    label loop
    goto loop
    if-goto loop"#;

    assert_eq!(
        parse(code),
        Ok(vec![
            VMInstruction::Label("loop".to_string()),
            VMInstruction::Goto(0),
            VMInstruction::IfGoto(0),
        ])
    );
}

#[test]
fn function() {
    let code = r#"
    function main 3
    return
    call main 1"#;

    assert_eq!(
        parse(code),
        Ok(vec![
            VMInstruction::Function("main".to_string(), 3),
            VMInstruction::Return,
            VMInstruction::Call(0, 3),
        ])
    );
}

#[test]
fn fibonacci() {
    let code = r#"
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
    return"#;

    assert_eq!(
        parse(code),
        Ok(vec![
            VMInstruction::Function("Main.fibonacci".to_string(), 0),
            VMInstruction::Push(Segment::Argument, 0),
            VMInstruction::PushConst(2),
            VMInstruction::Lt,
            VMInstruction::IfGoto(6),
            VMInstruction::Goto(9),
            VMInstruction::Label("IF_TRUE".to_string()),
            VMInstruction::Push(Segment::Argument, 0),
            VMInstruction::Return,
            VMInstruction::Label("IF_FALSE".to_string()),
            VMInstruction::Push(Segment::Argument, 0),
            VMInstruction::PushConst(2),
            VMInstruction::Sub,
            VMInstruction::Call(0, 0),
            VMInstruction::Push(Segment::Argument, 0),
            VMInstruction::PushConst(1),
            VMInstruction::Sub,
            VMInstruction::Call(0, 0),
            VMInstruction::Add,
            VMInstruction::Return
        ])
    );
}

#[test]
fn simple_function() {
    let code = r#"
    // This file is part of www.nand2tetris.org
    // and the book "The Elements of Computing Systems"
    // by Nisan and Schocken, MIT Press.
    // File name: projects/08/FunctionCalls/SimpleFunction/SimpleFunction.vm
    
    // Performs a simple calculation and returns the result.
    function SimpleFunction.test 2
    push local 0
    push local 1
    add
    not
    push argument 0
    add
    push argument 1
    sub
    return
    "#;

    assert_eq!(
        parse(code),
        Ok(vec![
            VMInstruction::Function("SimpleFunction.test".to_string(), 2),
            VMInstruction::Push(Segment::Local, 0),
            VMInstruction::Push(Segment::Local, 1),
            VMInstruction::Add,
            VMInstruction::Not,
            VMInstruction::Push(Segment::Argument, 0),
            VMInstruction::Add,
            VMInstruction::Push(Segment::Argument, 1),
            VMInstruction::Sub,
            VMInstruction::Return
        ])
    );
}
