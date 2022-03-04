use n2t_lib::test_script::{Instruction, N2tCmp, N2tTst, OutList, OutType};

#[test]
fn cmp() {
    let code = r"
    | time | RAM[24576]  |   load   |
    | 0+   |     0       |  000000  |
    | 1    |     20      |  111111  |
    | 1+   |     540     |  101010  |

    | 2    |     19      |  101010  |
    | 2+   |  -32123     |  000000  |
    | 3    |     1       |  000000  |
    
    ";

    let cmp = N2tCmp::new(
        vec![
            vec![
                OutType::Clock((0, true)),
                OutType::Clock((1, false)),
                OutType::Clock((1, true)),
                OutType::Clock((2, false)),
                OutType::Clock((2, true)),
                OutType::Clock((3, false)),
            ],
            vec![
                OutType::Decimal(0),
                OutType::Decimal(20),
                OutType::Decimal(540),
                OutType::Decimal(19),
                OutType::Decimal(-32123),
                OutType::Decimal(1),
            ],
            vec![
                OutType::Binary(vec![false, false, false, false, false, false]),
                OutType::Binary(vec![true, true, true, true, true, true]),
                OutType::Binary(vec![true, false, true, false, true, false]),
                OutType::Binary(vec![true, false, true, false, true, false]),
                OutType::Binary(vec![false, false, false, false, false, false]),
                OutType::Binary(vec![false, false, false, false, false, false]),
            ],
        ],
        vec!["time", "RAM[24576]", "load"],
    );

    assert_eq!(
        Ok(cmp),
        N2tCmp::from_code(
            code,
            vec![
                OutType::Clock((0, false)),
                OutType::Decimal(0),
                OutType::Binary(vec![false; 6]),
            ],
        )
    );
}

#[test]
#[ignore]
fn tst() {
    let code = r"
    load tst.hdl,
    output-file tst.out,
    compare-to tst.cmp,
    output-list a%B3.1.3 b%B3.1.3 out%B3.1.3, time%S1.4.1, z%B1.16.1, num%D2.6.2;

    set a 0,
    set b 1,
    set z %B0000111100000000,
    set num -5,
    tick,
    eval,
    output;

    tock,
    set num 3,
    output;

    repeat 10 {
        ticktock;
    }
    ";

    let tst = N2tTst::new(
        "tst.hdl",
        Some("tst.out"),
        Some("tst.cmp"),
        Some(vec![OutList::new(
            "a",
            (0, 0, 0),
            OutType::Binary(vec![false]),
        )]),
        vec![
            Instruction::Set("a".to_string(), OutType::Binary(vec![false])),
            Instruction::Set("b".to_string(), OutType::Binary(vec![true])),
            Instruction::BeginRepeat(Some(10)),
            Instruction::TickTock,
            Instruction::EndRepeat,
        ],
    );

    assert_eq!(N2tTst::from_code(code), Ok(tst));
}

#[test]
#[ignore]
fn tst_fill() {
    let code = r#"
    load Fill.asm;
    echo "Make sure that 'No Animation' is selected. Then, select the keyboard, press any key for some time, and inspect the screen.";
    
    repeat {
      ticktock;
    }
    "#;

    let tst = N2tTst::new(  "Fill.asm",
        None,
        None,
        None,
        vec![
            Instruction::Echo("Make sure that 'No Animation' is selected. Then, select the keyboard, press any key for some time, and inspect the screen.".to_string()), 
            Instruction::BeginRepeat(None),Instruction::TickTock, Instruction::EndRepeat]);

    assert_eq!(N2tTst::from_code(code), Ok(tst));
}
