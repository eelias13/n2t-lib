use hardware_sim::{ChipDef, ComponentMap};
use n2t_lib::hdl::parse;

#[test]
fn and() {
    let code = r"
    CHIP And {
        IN a, b;
        OUT out;
     PARTS:
        Nand(a=a, b=b, out=nand);
        Nand(a=nand, b=nand, out=out);
    }";

    assert_eq!(
        parse(code),
        Ok(vec![ChipDef::new(
            "And",
            vec!["a", "b"],
            vec!["out"],
            vec![
                ComponentMap::new(vec![("a", "a"), ("b", "b"), ("out", "nand")], "Nand"),
                ComponentMap::new(vec![("a", "nand"), ("b", "nand"), ("out", "out")], "Nand"),
            ]
        )])
    );
}

#[test]
fn not() {
    let code = r"
    CHIP Not {
        IN a;
        OUT out;
     PARTS:
        Nand(a=a, b=a, out=out);
    }";

    assert_eq!(
        parse(code),
        Ok(vec![ChipDef::new(
            "Not",
            vec!["a"],
            vec!["out"],
            vec![ComponentMap::new(
                vec![("a", "a"), ("b", "a"), ("out", "out")],
                "Nand"
            )]
        )])
    );
}

#[test]
fn or() {
    let code = r"
    CHIP Or {
        IN a, b;
        OUT out;
     PARTS:
        Nand(a=a, b=a, out=not_a);
        Nand(a=b, b=b, out=not_b);
        Nand(a=not_a, b=not_b, out=out);
    }";

    assert_eq!(
        parse(code),
        Ok(vec![ChipDef::new(
            "Or",
            vec!["a", "b"],
            vec!["out"],
            vec![
                ComponentMap::new(vec![("a", "a"), ("b", "a"), ("out", "not_a")], "Nand"),
                ComponentMap::new(vec![("a", "b"), ("b", "b"), ("out", "not_b")], "Nand"),
                ComponentMap::new(vec![("a", "not_a"), ("b", "not_b"), ("out", "out")], "Nand"),
            ]
        )])
    );
}

#[test]
fn xor() {
    let code = r"
    CHIP Xor {
        IN a, b;
        OUT out;
     PARTS:
        Nand(a=a, b=b, out=ab_nand);
        Nand(a=a, b=ab_nand, out=a_nand);
        Nand(a=b, b=ab_nand, out=b_nand);
        Nand(a=a_nand, b=b_nand, out=out);
    }";

    assert_eq!(
        parse(code),
        Ok(vec![ChipDef::new(
            "Xor",
            vec!["a", "b"],
            vec!["out"],
            vec![
                ComponentMap::new(vec![("a", "a"), ("b", "b"), ("out", "ab_nand")], "Nand"),
                ComponentMap::new(
                    vec![("a", "a"), ("b", "ab_nand"), ("out", "a_nand")],
                    "Nand"
                ),
                ComponentMap::new(
                    vec![("a", "b"), ("b", "ab_nand"), ("out", "b_nand")],
                    "Nand"
                ),
                ComponentMap::new(
                    vec![("a", "a_nand"), ("b", "b_nand"), ("out", "out")],
                    "Nand"
                ),
            ]
        )])
    );
}

#[test]
fn rs_ff() {
    let code = r"
    CHIP RS_FF {
        IN R, S;
        OUT Q, Q_n;
     PARTS:
        Nand(a=S, b=Q_n, out=Q);
        Nand(a=R, b=Q, out=Q_n);
    }";

    assert_eq!(
        parse(code),
        Ok(vec![ChipDef::new(
            "RS_FF",
            vec!["R", "S"],
            vec!["Q", "Q_n"],
            vec![
                ComponentMap::new(vec![("a", "S"), ("b", "Q_n"), ("out", "Q")], "Nand"),
                ComponentMap::new(vec![("a", "R"), ("b", "Q"), ("out", "Q_n")], "Nand")
            ]
        )])
    );
}

#[test]
fn jk_ff() {
    let code = r"
    CHIP JK_FF {
        IN J, K, Clk;
        OUT Q, Q_n;
     PARTS:
        And(a=J, b=Clk, out=J_Clk);
        And(a=K, b=Clk, out=K_Clk);
        And(a=J_Clk, b=Q_n, out=R);
        And(a=K_Clk, b=Q, out=S);
        RS_FF(R=R, S=S, Q=Q, Q_n=Q_n);
    }";

    assert_eq!(
        parse(code),
        Ok(vec![ChipDef::new(
            "JK_FF",
            vec!["J", "K", "Clk"],
            vec!["Q", "Q_n"],
            vec![
                ComponentMap::new(vec![("a", "J"), ("b", "Clk"), ("out", "J_Clk")], "And"),
                ComponentMap::new(vec![("a", "K"), ("b", "Clk"), ("out", "K_Clk")], "And"),
                ComponentMap::new(vec![("a", "J_Clk"), ("b", "Q_n"), ("out", "R")], "And"),
                ComponentMap::new(vec![("a", "K_Clk"), ("b", "Q"), ("out", "S")], "And"),
                ComponentMap::new(
                    vec![("R", "R"), ("S", "S"), ("Q", "Q"), ("Q_n", "Q_n")],
                    "RS_FF"
                )
            ]
        )])
    );
}

#[test]
fn d_ff() {
    let code = r"
    CHIP D_FF {
        IN D, Clk;
        OUT Q, Q_n;
     PARTS:
        Not(a=D, out=D_n);
        And(a=D, b=Clk, out=D_Clk);
        And(a=D_n, b=Clk, out=D_n_Clk);
        RS_FF(R=D_Clk, S=D_n_Clk, Q=Q, Q_n=Q_n);
    }";

    assert_eq!(
        parse(code),
        Ok(vec![ChipDef::new(
            "D_FF",
            vec!["D", "Clk"],
            vec!["Q", "Q_n"],
            vec![
                ComponentMap::new(vec![("a", "D"), ("out", "D_n")], "Not"),
                ComponentMap::new(vec![("a", "D"), ("b", "Clk"), ("out", "D_Clk")], "And"),
                ComponentMap::new(vec![("a", "D_n"), ("b", "Clk"), ("out", "D_n_Clk")], "And"),
                ComponentMap::new(
                    vec![("R", "D_Clk"), ("S", "D_n_Clk"), ("Q", "Q"), ("Q_n", "Q_n")],
                    "RS_FF"
                )
            ]
        )])
    );
}
