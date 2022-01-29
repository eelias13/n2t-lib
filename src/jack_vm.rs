use crate::{Instruction, Segment};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JackVM {
    stack: Vec<isize>,

    this: HashMap<usize, isize>,
    that: HashMap<usize, isize>,
    local: HashMap<usize, isize>,
    argument: HashMap<usize, isize>,
    pointer: HashMap<usize, isize>,
    temp: HashMap<usize, isize>,
    static_seg: HashMap<usize, isize>,

    program_counter: usize,
    program: Vec<Instruction>,

    is_runing: bool,
}

impl JackVM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            is_runing: program.len() != 0,

            stack: Vec::new(),

            this: HashMap::new(),
            that: HashMap::new(),
            local: HashMap::new(),
            argument: HashMap::new(),
            pointer: HashMap::new(),
            temp: HashMap::new(),
            static_seg: HashMap::new(),

            program_counter: 0,
            program,
        }
    }

    pub fn step(&mut self) {
        if self.is_runing {
            if self.program.len() >= self.program_counter {
                self.is_runing = false;
                return;
            }
            self.execute(self.program[self.program_counter].clone());
            self.program_counter += 1;
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Push(seg, addr) => self.push(seg, addr),
            Instruction::PushConst(value) => self.stack.push(value),
            Instruction::Pop(seg, addr) => {
                if let Some(val) = self.stack_pop() {
                    self.get_seg(seg).insert(addr, val);
                }
            }

            Instruction::Add => {
                if let Some(val1) = self.stack_pop() {
                    if let Some(val2) = self.stack_pop() {
                        self.stack.push(val1 + val2);
                    }
                }
            }
            Instruction::Sub => {
                if let Some(val1) = self.stack_pop() {
                    if let Some(val2) = self.stack_pop() {
                        self.stack.push(val1 - val2);
                    }
                }
            }
            Instruction::And => {
                if let Some(val1) = self.stack_pop() {
                    if let Some(val2) = self.stack_pop() {
                        self.stack.push(val1 & val2);
                    }
                }
            }
            Instruction::Or => {
                if let Some(val1) = self.stack_pop() {
                    if let Some(val2) = self.stack_pop() {
                        self.stack.push(val1 | val2);
                    }
                }
            }

            Instruction::Neg => {
                if let Some(val) = self.stack_pop() {
                    self.stack.push(!val);
                }
            }
            Instruction::Not => {
                if let Some(val) = self.stack_pop() {
                    self.stack.push(!val);
                }
            }

            Instruction::Eq => {
                if let Some(val1) = self.stack_pop() {
                    if let Some(val2) = self.stack_pop() {
                        self.stack.push(if val1 == val2 { 0 } else { 1 });
                    }
                }
            }
            Instruction::Gt => {
                if let Some(val1) = self.stack_pop() {
                    if let Some(val2) = self.stack_pop() {
                        self.stack.push(if val1 > val2 { 0 } else { 1 });
                    }
                }
            }
            Instruction::Lt => {
                if let Some(val1) = self.stack_pop() {
                    if let Some(val2) = self.stack_pop() {
                        self.stack.push(if val1 < val2 { 0 } else { 1 });
                    }
                }
            }

            Instruction::Function(_, _) => todo!(),
            Instruction::Call(addr, argc) => todo!(),
            Instruction::Return => todo!(),

            Instruction::Label(_) => (),
            Instruction::IfGoto(addr) => {
                if let Some(val) = self.stack_pop() {
                    if val == 0 {
                        self.program_counter = addr;
                    }
                }
            }
            Instruction::Goto(addr) => self.program_counter = addr,
        }
    }

    fn get_seg(&mut self, seg: Segment) -> &mut HashMap<usize, isize> {
        match seg {
            Segment::Argument => &mut self.argument,
            Segment::Local => &mut self.local,
            Segment::Pointer => &mut self.pointer,
            Segment::Static => &mut self.static_seg,
            Segment::Temp => &mut self.temp,
            Segment::That => &mut self.that,
            Segment::This => &mut self.this,
        }
    }

    pub fn seg_val(&mut self, seg: Segment, addr: usize) -> Option<&isize> {
        self.get_seg(seg).get(&addr)
    }

    fn stack_pop(&mut self) -> Option<isize> {
        if let Some(val) = self.stack.pop() {
            Some(val)
        } else {
            self.error("stack is empty");
            None
        }
    }

    fn push(&mut self, seg: Segment, addr: usize) {
        if let Some(&val) = self.seg_val(seg.clone(), addr) {
            self.stack.push(val);
        } else {
            self.error(&format!(
                "Segment {:?} is uninitialized at addres {} ",
                seg, addr
            ));
        }
    }

    fn error(&mut self, msg: &str) {
        println!("Error: {} at line {}", msg, self.program_counter);
        self.is_runing = false;
    }
}
