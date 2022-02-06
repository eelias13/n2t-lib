use crate::cpu::{Comp, Dest, CPUInstruction, Jump};

pub struct HackCpu {
    d_reg: i16,
    a_reg: i16,
    pc: usize,

    ram: Vec<i16>,
    rom: Vec<CPUInstruction>,
}

impl HackCpu {
    pub fn new(program: Vec<CPUInstruction>) -> Self {
        Self {
            d_reg: 0,
            a_reg: 0,
            pc: 0,

            ram: vec![0; 0xffff],
            rom: program,
        }
    }

    pub fn step(&mut self) {
        match self.rom[self.pc].clone() {
            CPUInstruction::AInstruc(val) => self.a_reg = val,
            CPUInstruction::CInstruc(comp, dest, jump) => {
                let val = self.compute(comp);
                self.store_dest(dest, val);
                self.jump(jump, val);
            }
        }
    }

    fn jump(&mut self, jump: Jump, val: i16) {
        let should_jmp = match jump {
            Jump::Null => false,
            Jump::JGT => val > 0,
            Jump::JEQ => val == 0,
            Jump::JGE => val >= 0,
            Jump::JLT => val < 0,
            Jump::JNE => val != 0,
            Jump::JLE => val <= 0,
            Jump::JMP => true,
        };

        if should_jmp {
            self.pc = self.a_reg as usize;
        } else {
            self.pc += 1;
        }
    }

    fn store_dest(&mut self, dest: Dest, val: i16) {
        match dest {
            Dest::Null => (),
            Dest::M => self.ram[self.a_reg as usize] = val,
            Dest::D => self.d_reg = val,
            Dest::MD => {
                self.ram[self.a_reg as usize] = val;
                self.d_reg = val;
            }
            Dest::A => self.a_reg = val,
            Dest::AM => {
                self.a_reg = val;
                self.ram[self.a_reg as usize] = val;
            }
            Dest::AD => {
                self.a_reg = val;
                self.d_reg = val;
            }
            Dest::AMD => {
                self.a_reg = val;
                self.ram[self.a_reg as usize] = val;
                self.d_reg = val;
            }
        }
    }

    fn compute(&self, comp: Comp) -> i16 {
        match comp {
            Comp::Zero => 0,
            Comp::One => 1,
            Comp::MinusOne => -1,
            Comp::D => self.d_reg,
            Comp::A => self.a_reg,
            Comp::NotD => !self.d_reg,
            Comp::NotA => !self.a_reg,
            Comp::MinusD => -1 * self.d_reg,
            Comp::MinusA => -1 * self.d_reg,
            Comp::DPulsOne => self.d_reg + 1,
            Comp::APulsOne => self.a_reg + 1,
            Comp::DMinusOne => self.d_reg - 1,
            Comp::AMinusOne => self.a_reg - 1,
            Comp::DPulsA => self.d_reg + self.a_reg,
            Comp::DMinusA => self.d_reg - self.a_reg,
            Comp::AMinusD => self.a_reg - self.d_reg,
            Comp::DAndA => self.d_reg & self.a_reg,
            Comp::DOrA => self.d_reg | self.a_reg,
            Comp::M => self.ram[self.a_reg as usize],
            Comp::NotM => !self.ram[self.a_reg as usize],
            Comp::MPlusOne => self.ram[self.a_reg as usize] + 1,
            Comp::MMinusOne => self.ram[self.a_reg as usize] - 1,
            Comp::DPulsM => self.d_reg + self.ram[self.a_reg as usize],
            Comp::DMinusM => self.d_reg - self.ram[self.a_reg as usize],
            Comp::MMinusD => self.ram[self.a_reg as usize] - self.d_reg,
            Comp::DAndM => self.d_reg & self.ram[self.a_reg as usize],
            Comp::DOrM => self.d_reg | self.ram[self.a_reg as usize],
        }
    }
}
