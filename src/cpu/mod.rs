mod hack_cpu;
mod parser;

pub use hack_cpu::HackCpu;
pub use parser::{asm2ml, ml2asm, parse};

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Comp {
    // a = 0
    Zero = 0b0101010,
    One = 0b0111111,
    MinusOne = 0b0111010,
    D = 0b0001100,
    A = 0b0110000,
    NotD = 0b0001101,
    NotA = 0b0110001,
    MinusD = 0b0001111,
    MinusA = 0b0110011,
    DPulsOne = 0b0011111,
    APulsOne = 0b0110111,
    DMinusOne = 0b0001110,
    AMinusOne = 0b0110010,
    DPulsA = 0b0000010,
    DMinusA = 0b0010011,
    AMinusD = 0b0000111,
    DAndA = 0b0000000,
    DOrA = 0b0010101,
    // a = 1
    M = 0b1110000,
    NotM = 0b1110001,
    MPlusOne = 0b1110111,
    MMinusOne = 0b1110010,
    DPulsM = 0b1000010,
    DMinusM = 0b1010011,
    MMinusD = 0b1000111,
    DAndM = 0b1000000,
    DOrM = 0b1010101,
}

impl TryInto<Comp> for u8 {
    type Error = String;
    fn try_into(self) -> Result<Comp, Self::Error> {
        match self {
            0b0101010 => Ok(Comp::Zero),
            0b0111111 => Ok(Comp::One),
            0b0111010 => Ok(Comp::MinusOne),
            0b0001100 => Ok(Comp::D),
            0b0110000 => Ok(Comp::A),
            0b0001101 => Ok(Comp::NotD),
            0b0110001 => Ok(Comp::NotA),
            0b0001111 => Ok(Comp::MinusD),
            0b0110011 => Ok(Comp::MinusA),
            0b0011111 => Ok(Comp::DPulsOne),
            0b0110111 => Ok(Comp::APulsOne),
            0b0001110 => Ok(Comp::DMinusOne),
            0b0110010 => Ok(Comp::AMinusOne),
            0b0000010 => Ok(Comp::DPulsA),
            0b0010011 => Ok(Comp::DMinusA),
            0b0000111 => Ok(Comp::AMinusD),
            0b0000000 => Ok(Comp::DAndA),
            0b0010101 => Ok(Comp::DOrA),
            0b1110000 => Ok(Comp::M),
            0b1110001 => Ok(Comp::NotM),
            0b1110111 => Ok(Comp::MPlusOne),
            0b1110010 => Ok(Comp::MMinusOne),
            0b1000010 => Ok(Comp::DPulsM),
            0b1010011 => Ok(Comp::DMinusM),
            0b1000111 => Ok(Comp::MMinusD),
            0b1000000 => Ok(Comp::DAndM),
            0b1010101 => Ok(Comp::DOrM),
            _ => Err(format!("Comp {:#b} is supported", self)),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Dest {
    Null = 0b000,
    M = 0b001,
    D = 0b010,
    MD = 0b011,
    A = 0b100,
    AM = 0b101,
    AD = 0b110,
    AMD = 0b111,
}

impl TryInto<Dest> for u8 {
    type Error = String;
    fn try_into(self) -> Result<Dest, Self::Error> {
        match self {
            0b000 => Ok(Dest::Null),
            0b001 => Ok(Dest::M),
            0b010 => Ok(Dest::D),
            0b011 => Ok(Dest::MD),
            0b100 => Ok(Dest::A),
            0b101 => Ok(Dest::AM),
            0b110 => Ok(Dest::AD),
            0b111 => Ok(Dest::AMD),
            _ => Err(format!("Dest {:#b} is supported", self)),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum Jump {
    Null = 0b000,
    JGT = 0b001,
    JEQ = 0b010,
    JGE = 0b011,
    JLT = 0b100,
    JNE = 0b101,
    JLE = 0b110,
    JMP = 0b111,
}

impl TryInto<Jump> for u8 {
    type Error = String;
    fn try_into(self) -> Result<Jump, Self::Error> {
        match self {
            0b000 => Ok(Jump::Null),
            0b001 => Ok(Jump::JGT),
            0b010 => Ok(Jump::JEQ),
            0b011 => Ok(Jump::JGE),
            0b100 => Ok(Jump::JLT),
            0b101 => Ok(Jump::JNE),
            0b110 => Ok(Jump::JLE),
            0b111 => Ok(Jump::JMP),
            _ => Err(format!("Jump {:#b} is supported", self)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CPUInstruction {
    AInstruc(i16),
    CInstruc(Comp, Dest, Jump),
}
