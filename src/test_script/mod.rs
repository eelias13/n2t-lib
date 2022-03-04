mod n2t_cmp;
mod n2t_tst;

pub use n2t_cmp::N2tCmp;
pub use n2t_tst::{Instruction, N2tTst, OutList};

#[derive(Debug, Clone, PartialEq)]
pub enum OutType {
    Clock((usize, bool)),
    Binary(Vec<bool>),
    Decimal(isize),
}
