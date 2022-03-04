use std::fs::read_to_string;

fn test_hdl(path: &str) {
    let hdl = read_to_string(format!("{}.hdl", path)).unwrap();
    let tst = read_to_string(format!("{}.tst", path)).unwrap();
    let cmp = read_to_string(format!("{}.cmp", path)).unwrap();

    assert_eq!(
        n2t_lib::hdl::test(hdl.as_str(), tst.as_str(), cmp.as_str()),
        Ok(())
    );
}

fn test_asm_ml(asm: &str, hack: &str) {
    let ml = n2t_lib::cpu::str2ml(hack).unwrap();
    let asm2ml = n2t_lib::cpu::asm2ml(n2t_lib::cpu::parse(asm).unwrap());
    let ml2asm = n2t_lib::cpu::ml2asm(ml.clone()).unwrap();
    assert_eq!(ml2asm, n2t_lib::cpu::parse(asm).unwrap());
    assert_eq!(asm2ml, ml);
}

#[test]
fn demo() {
    test_hdl("tests/projects/demo/Xor");
}

#[test]
fn project_01() {
    test_hdl("tests/projects/01/And");
    test_hdl("tests/projects/01/And16");
    test_hdl("tests/projects/01/DMux");
    test_hdl("tests/projects/01/DMux4Way");
    test_hdl("tests/projects/01/DMux8Way");
    test_hdl("tests/projects/01/Mux");
    test_hdl("tests/projects/01/Mux16");
    test_hdl("tests/projects/01/Mux4Way16");
    test_hdl("tests/projects/01/Mux8Way16");
    test_hdl("tests/projects/01/Not");
    test_hdl("tests/projects/01/Not16");
    test_hdl("tests/projects/01/Or");
    test_hdl("tests/projects/01/Or16");
    test_hdl("tests/projects/01/Or8Way");
    test_hdl("tests/projects/01/Xor");
}

#[test]
fn project_02() {
    test_hdl("tests/projects/02/ALU");
    test_hdl("tests/projects/02/Add16");
    test_hdl("tests/projects/02/Add8");
    test_hdl("tests/projects/02/FullAdder");
    test_hdl("tests/projects/02/HalfAdder");
    test_hdl("tests/projects/02/Inc16");
    test_hdl("tests/projects/02/Inc8");
}

#[test]
fn project_03_a() {
    test_hdl("tests/projects/03/a/Bit");
    test_hdl("tests/projects/03/a/PC");
    test_hdl("tests/projects/03/a/RAM64");
    test_hdl("tests/projects/03/a/RAM8");
    test_hdl("tests/projects/03/a/Register");
}

#[test]
fn project_03_b() {
    test_hdl("tests/projects/03/b/RAM16K");
    test_hdl("tests/projects/03/b/RAM4K");
    test_hdl("tests/projects/03/b/RAM512");
}

mod project_06 {
    use std::fs::read_to_string;

    use n2t_lib::cpu::{asm2ml, ml2asm, parse, str2ml, CPUInstruction, Comp, Dest, Jump};

    #[test]
    fn add() {
        let asm = read_to_string("tests/projects/06/add/Add.asm").unwrap();
        let hack = read_to_string("tests/projects/06/add/Add.hack").unwrap();

        let asm = parse(&asm).unwrap();
        let hack = str2ml(&hack).unwrap();

        assert_eq!(
            asm,
            vec![
                CPUInstruction::AInstruc(2),
                CPUInstruction::CInstruc(Comp::A, Dest::D, Jump::Null),
                CPUInstruction::AInstruc(3),
                CPUInstruction::CInstruc(Comp::DPulsA, Dest::D, Jump::Null),
                CPUInstruction::AInstruc(0),
                CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null),
            ]
        );

        assert_eq!(hack, vec![2, 60432, 3, 57488, 0, 58120]);
        assert_eq!(hack, asm2ml(asm.clone()));
        assert_eq!(Ok(asm), ml2asm(hack));
    }

    #[test]
    fn max() {
        let asm = read_to_string("tests/projects/06/max/Max.asm").unwrap();
        let asml = read_to_string("tests/projects/06/max/MaxL.asm").unwrap();
        let hack = read_to_string("tests/projects/06/max/Max.hack").unwrap();

        assert_eq!(
            parse(&asml),
            Ok(vec![
                CPUInstruction::AInstruc(0),
                CPUInstruction::CInstruc(Comp::M, Dest::D, Jump::Null),
                CPUInstruction::AInstruc(1),
                CPUInstruction::CInstruc(Comp::DMinusM, Dest::D, Jump::Null),
                CPUInstruction::AInstruc(10),
                CPUInstruction::CInstruc(Comp::D, Dest::Null, Jump::JGT),
                CPUInstruction::AInstruc(1),
                CPUInstruction::CInstruc(Comp::M, Dest::D, Jump::Null),
                CPUInstruction::AInstruc(12),
                CPUInstruction::CInstruc(Comp::Zero, Dest::Null, Jump::JMP),
                CPUInstruction::AInstruc(0),
                CPUInstruction::CInstruc(Comp::M, Dest::D, Jump::Null),
                CPUInstruction::AInstruc(2),
                CPUInstruction::CInstruc(Comp::D, Dest::M, Jump::Null),
                CPUInstruction::AInstruc(14),
                CPUInstruction::CInstruc(Comp::Zero, Dest::Null, Jump::JMP),
            ])
        );
        assert_eq!(parse(&asm), parse(&asml));
        super::test_asm_ml(&asm, &hack);
    }

    #[test]
    fn pong() {
        let asm = read_to_string("tests/projects/06/pong/Pong.asm").unwrap();
        let asml = read_to_string("tests/projects/06/pong/PongL.asm").unwrap();
        let hack = read_to_string("tests/projects/06/pong/Pong.hack").unwrap();

        assert_eq!(parse(&asm), parse(&asml));
        super::test_asm_ml(&asm, &hack);
    }

    #[test]
    fn rect() {
        let asm = read_to_string("tests/projects/06/rect/Rect.asm").unwrap();
        let asml = read_to_string("tests/projects/06/rect/RectL.asm").unwrap();
        let hack = read_to_string("tests/projects/06/rect/Rect.hack").unwrap();

        assert_eq!(parse(&asm), parse(&asml));
        super::test_asm_ml(&asm, &hack);
    }
}
