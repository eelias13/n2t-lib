// load Bit.hdl,
// output-file Bit.out,
// compare-to Bit.cmp,
// output-list time%S1.4.1 in%B2.1.2 load%B2.1.2 out%B2.1.2;

use logos::{Lexer, Logos};
use tokenizer::{Error, Tokenizer, TypeEq};

const DOUBLE_QUOTES: &str = "#!#";

#[derive(Debug, Clone, PartialEq)]
pub enum OutType {
    Clock((usize, bool)),
    Binary(Vec<bool>),
    Decimal(isize),
}

#[derive(Debug, Clone, PartialEq)]
enum Instruction {
    Set(String, OutType),
    Tock,
    Tick,
    TickTock,
    Output,
    Eval,
    Echo(String),
    BeginRepeat(Option<usize>),
    EndRepeat,
    EndInstruction,
}

#[derive(Debug, Clone, PartialEq)]
struct OutList {
    name: String,
    triplet: (usize, usize, usize),
    out_type: OutType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TST {
    load: String,
    out_file: Option<String>,
    compare_to: Option<String>,
    out_list: Option<Vec<OutList>>,
    instruction: Vec<Instruction>,

    out: Option<CMP>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CMP {
    table: Vec<Vec<OutType>>,
    names: Vec<String>,
}

impl TST {
    pub fn new(code: &str) -> Result<Self, Error> {
        // let code = &code.replace("\"", DOUBLE_QUOTES);

        let load;
        let out_file;
        let compare_to;
        let out_list;

        let mut instruction = Vec::new();
        let mut tokenizer = Tokenizer::new(Token::lexer(code), vec![Token::Ignore((0, None))]);

        tokenizer.next();
        // get head
        expect_str(&mut tokenizer, "load")?;
        load = get_str(&mut tokenizer)?;
        if tokenizer.next_is(Token::Comma) {
            tokenizer.expect(Token::Comma)?;

            expect_str(&mut tokenizer, "output-file")?;
            out_file = Some(get_str(&mut tokenizer)?);
            tokenizer.expect(Token::Comma)?;

            expect_str(&mut tokenizer, "compare-to")?;
            compare_to = Some(get_str(&mut tokenizer)?);
            tokenizer.expect(Token::Comma)?;

            expect_str(&mut tokenizer, "output-list")?;
            out_list = Some(get_out_list(&mut tokenizer)?);
        } else {
            tokenizer.expect(Token::Semicolon)?;

            out_file = None;
            compare_to = None;
            out_list = None;
        }

        while tokenizer.current() != None {
            if tokenizer.next_is(Token::CurlyClose) {
                tokenizer.expect(Token::CurlyClose)?;
                instruction.push(Instruction::EndRepeat);
                continue;
            }

            let value = get_str(&mut tokenizer)?;
            let mut is_repeat = false;
            match value.as_str() {
                "tock" => instruction.push(Instruction::Tock),
                "tick" => instruction.push(Instruction::Tick),
                "ticktock" => instruction.push(Instruction::TickTock),
                "output" => instruction.push(Instruction::Output),
                "eval" => instruction.push(Instruction::Eval),
                "echo" => instruction.push(echo(&mut tokenizer)?),
                "repeat" => {
                    instruction.push(repeat(&mut tokenizer)?);
                    is_repeat = true;
                }
                "set" => instruction.push(set(&mut tokenizer, &out_list)?),
                _ => return Err(tokenizer.error(&format!("expected string set but got {}", value))),
            }

            if !is_repeat {
                if tokenizer.next_is(Token::Comma) {
                    tokenizer.expect(Token::Comma)?;
                } else {
                    tokenizer.expect(Token::Semicolon)?;
                    instruction.push(Instruction::EndInstruction);
                }
            }
        }

        Ok(Self {
            load,
            out_file,
            compare_to,
            out_list,
            instruction,

            out: None,
        })
    }

    pub fn validate(&self, cmp: CMP) -> Result<(), usize> {
        Ok(())
    }
}



impl CMP {
    pub fn new(code: &str, head: Vec<OutType>) -> Result<Self, String> {
        let mut table = Vec::new();
        for _ in head.iter() {
            table.push(Vec::new());
        }

        let mut lines: Vec<String> = code
            .split("\n")
            .filter(|s| s.contains("|"))
            .map(|s| s.to_string().replace(" ", "").replace("\t", ""))
            .collect();

        let names: Vec<String> = lines[0]
            .split("|")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        lines.remove(0);
        if names.len() != table.len() {
            return Err(format!(
                "head len {} dose not match names len {}",
                table.len(),
                names.len()
            ));
        }

        for line in lines {
            let values: Vec<String> = line
                .split("|")
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            if values.len() != table.len() {
                return Err(format!(
                    "table len {} mismatch with values len {}",
                    table.len(),
                    values.len()
                ));
            }

            for i in 0..table.len() {
                let mut is_clock = false;
                let mut val = values[i].clone();
                if values[i].chars().last() == Some('+') {
                    is_clock = true;
                    val = val[0..val.len() - 1].to_string();
                }

                let out_type = match head[i] {
                    OutType::Clock(_) => OutType::Clock((
                        if let Ok(res) = val.parse() {
                            res
                        } else {
                            return Err(format!("{} is not a number", val));
                        },
                        is_clock,
                    )),
                    OutType::Binary(_) => OutType::Binary(parse_bool(val)?),
                    OutType::Decimal(_) => OutType::Decimal(if let Ok(res) = val.parse() {
                        res
                    } else {
                        return Err(format!("{} is not a number", val));
                    }),
                };

                table[i].push(out_type);
            }
        }

        Ok(Self { table, names })
    }
}

#[derive(Logos, Debug, Clone, PartialEq)]
enum Token {
    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("{")]
    CurlyOpen,
    #[token("}")]
    CurlyClose,

    #[token("\t", ignore)]
    #[token(" ", ignore)]
    #[token("\n", ignore)]
    #[regex(r"(/\*([^*]|\*[^/])*\*/)|(//[^\r\n]*(\r\n|\n)?)", ignore)]
    Ignore((usize, Option<String>)),

    // DOUBLE_QUOTES
    #[regex(r"#!#[a-zA-Z_$0-9%. ]*#!#", name, priority = 6)]
    StringLit(String),

    #[regex(r"[a-zA-Z_$0-9%.]*", name)] //  priority = 5
    String(String),
    #[regex(r"[0-9]+", number)]
    Number(usize),

    #[error]
    Unknown,
}

fn name(lexer: &mut Lexer<Token>) -> Option<String> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 1];
    Some(slice.to_string())
}

fn number(lexer: &mut Lexer<Token>) -> Option<usize> {
    let slice = lexer.slice();
    let slice = &slice[1..slice.len() - 1];
    if let Ok(val) = slice.parse() {
        Some(val)
    } else {
        None
    }
}

fn ignore(lexer: &mut Lexer<Token>) -> Option<(usize, Option<String>)> {
    let slice = lexer.slice();
    match slice {
        " " => Some((0, None)),
        "\n" => Some((0, Some("newline".to_string()))),
        "\t" => Some((0, None)),
        _ => Some((slice.matches("\n").count(), Some(slice.to_string()))),
    }
}

impl TypeEq for Token {
    fn type_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Ignore(_), Token::Ignore(_)) => true,
            (Token::String(_), Token::String(_)) => true,
            (Token::Number(_), Token::Number(_)) => true,
            (Token::StringLit(_), Token::StringLit(_)) => true,
            _ => self == other,
        }
    }
}


// helper 

fn expect_str(tokenizer: &mut Tokenizer<Token>, expect: &str) -> Result<(), Error> {
    let token = tokenizer.expect(Token::String(String::new()))?;
    if token == Token::String(expect.to_string()) {
        Ok(())
    } else {
        Err(tokenizer.error(&format!(
            "expected string {} but got token {:?}",
            expect, token
        )))
    }
}

fn get_str(tokenizer: &mut Tokenizer<Token>) -> Result<String, Error> {
    if let Token::String(value) = tokenizer.expect(Token::String(String::new()))? {
        return Ok(value);
    }
    unreachable!();
}

fn get_out_list(tokenizer: &mut Tokenizer<Token>) -> Result<Vec<OutList>, Error> {
    let mut out_list = Vec::new();
    while !tokenizer.next_is(Token::Semicolon) {
        out_list.push(get_out(tokenizer)?);
    }
    Ok(out_list)
}

fn get_out(tokenizer: &mut Tokenizer<Token>) -> Result<OutList, Error> {
    let temp = get_str(tokenizer)?;
    let mut chars = temp.chars();

    let mut name = String::new();

    while let Some(c) = chars.next() {
        if c == '%' {
            break;
        }

        name.push(c);
    }

    let out_type = match chars.next() {
        Some('B') => OutType::Binary(Vec::new()),
        Some('D') => OutType::Decimal(0),
        Some('S') => OutType::Clock((0, false)),

        Some(c) => {
            return Err(tokenizer.error(&format!("unexpectde symbol {} expected D, B or S", c)))
        }
        None => return Err(tokenizer.error("wrong format")),
    };

    let temp: String = chars.collect();
    let mut triplet = Vec::<usize>::new();
    for num in temp.split('.') {
        if let Ok(num) = num.parse() {
            triplet.push(num)
        } else {
            return Err(tokenizer.error(&format!("unexpectde symbol {} expected a number", num)));
        }
    }
    if triplet.len() != 3 {
        return Err(tokenizer.error(&format!(
            "must be a triplet expectde len 3 but got {}",
            triplet.len()
        )));
    }
    let triplet = (triplet[0], triplet[1], triplet[2]);

    Ok(OutList {
        name,
        triplet,
        out_type,
    })
}

fn set(
    tokenizer: &mut Tokenizer<Token>,
    out_list: &Option<Vec<OutList>>,
) -> Result<Instruction, Error> {
    let var = get_str(tokenizer)?;
    let mut out_type = if let Some(out_list) = out_list {
        let mut found = None;
        for out in out_list {
            if out.name == var {
                found = Some(out.out_type.clone());
                break;
            }
        }
        if let Some(found) = found {
            found
        } else {
            return Err(tokenizer.error(&format!(
                "var {} was not initalized in the output-list",
                var
            )));
        }
    } else {
        return Err(tokenizer.error("no output-list supplied"));
    };

    if let OutType::Binary(_) = out_type {
        if tokenizer.next_is(Token::Number(0)) {
            if let Token::Number(num) = tokenizer.expect(Token::Number(0))? {
                if num == 0 {
                    out_type = OutType::Binary(vec![false]);
                } else if num == 1 {
                    out_type = OutType::Binary(vec![true]);
                } else {
                    return Err(tokenizer.error(&format!("{} is not a binary number", num)));
                }
            } else {
                unreachable!();
            }
        } else {
            let b_num = get_str(tokenizer)?;
            if &b_num[0..2] != "B%" {
                return Err(tokenizer.error(&format!(
                    "binary number {} dose not to start with B%",
                    b_num
                )));
            }

            let mut result = Vec::new();
            for c in b_num.chars().skip(2) {
                if c == '0' {
                    result.push(false);
                } else if c == '1' {
                    result.push(true);
                } else {
                    return Err(tokenizer.error(&format!("{} is not a binary number", b_num)));
                }
            }

            out_type = OutType::Binary(result);
        }
    } else if let OutType::Decimal(_) = out_type {
        out_type = OutType::Decimal(0);
    } else {
        return Err(tokenizer.error("no output-list supplied"));
    }

    Ok(Instruction::Set(var, out_type))
}

fn repeat(tokenizer: &mut Tokenizer<Token>) -> Result<Instruction, Error> {
    let num = if tokenizer.next_is(Token::Number(0)) {
        if let Token::Number(num) = tokenizer.expect(Token::Number(0))? {
            Some(num)
        } else {
            unreachable!();
        }
    } else {
        None
    };
    tokenizer.expect(Token::CurlyOpen)?;
    Ok(Instruction::BeginRepeat(num))
}

fn echo(tokenizer: &mut Tokenizer<Token>) -> Result<Instruction, Error> {
    if let Token::StringLit(string) = tokenizer.expect(Token::StringLit(String::new()))? {
        let string = string.replace(DOUBLE_QUOTES, "");

        Ok(Instruction::Echo(string))
    } else {
        unreachable!();
    }
}

pub fn parse_bool(val: String) -> Result<Vec<bool>, String> {
    let mut res = Vec::new();
    for c in val.chars() {
        res.push(match c {
            '0' => false,
            '1' => true,
            _ => return Err(format!("unexpected cahr '{}' expected '0' or '1'", c)),
        });
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
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

        let cmp = CMP {
            table: vec![
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
            names: vec![
                "time".to_string(),
                "RAM[24576]".to_string(),
                "load".to_string(),
            ],
        };

        assert_eq!(
            Ok(cmp),
            CMP::new(
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
    #[ignore ]
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

        let tst = TST{ load: "tst.hdl".to_string(),
        out_file: Some("tst.out".to_string()),
        compare_to: Some("tst.cmp".to_string()),
        out_list: Some(vec![OutList{name:"".to_string(), out_type:OutType::Binary(vec![false]), triplet: (0,0,0)}]),
        instruction:vec![
            Instruction::Set("a".to_string(), OutType::Binary(vec![false])), Instruction::Set("b".to_string(), OutType::Binary(vec![true])), 
            Instruction::BeginRepeat(Some(10)),Instruction::TickTock, Instruction::EndRepeat],

    
        out: None};

        assert_eq!(TST::new(code), Ok(tst));
    }

    #[test]
    #[ignore ]
    fn tst_fill() {
        let code = r#"
        load Fill.asm;
        echo "Make sure that 'No Animation' is selected. Then, select the keyboard, press any key for some time, and inspect the screen.";
        
        repeat {
          ticktock;
        }
        "#;

        let tst = TST{ load: "Fill.asm".to_string(),
            out_file: None,
            compare_to: None,
            out_list:None,
            instruction:vec![
                Instruction::Echo("Make sure that 'No Animation' is selected. Then, select the keyboard, press any key for some time, and inspect the screen.".to_string()), 
                Instruction::BeginRepeat(None),Instruction::TickTock, Instruction::EndRepeat],

        
            out: None};

            assert_eq!(TST::new(code), Ok(tst));
    }
}
