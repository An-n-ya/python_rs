use std::fmt::{Debug, Formatter};

pub enum CmpOP{
    LT,
    LE,
    EQ,
    NEQ,
    GT,
    GE
}

impl Debug for CmpOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CmpOP::LT => write!(f, "<"),
            CmpOP::LE => write!(f, "<="),
            CmpOP::EQ => write!(f, "=="),
            CmpOP::NEQ => write!(f, "!="),
            CmpOP::GT => write!(f, ">"),
            CmpOP::GE => write!(f, ">="),
        }
    }
}