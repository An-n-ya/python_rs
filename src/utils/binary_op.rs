use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::object::{IntObject, PyObject as PyObjectTrait};

type PyObject = Rc<dyn PyObjectTrait>;
pub enum BinaryOp{
    Add(bool),
    And(bool),
    FloorDivide(bool),
    ShiftLeft(bool),
    MatrixMultiply(bool),
    Multiply(bool),
    Modulo(bool),
    OR(bool),
    POWER(bool),
    ShiftRight(bool),
    Minus(bool),
    Divide(bool),
    XOR(bool),
}

impl Debug for BinaryOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add(false) => write!(f, "+"),
            BinaryOp::And(false) => write!(f, "&"),
            BinaryOp::FloorDivide(false) => write!(f, "//"),
            BinaryOp::ShiftLeft(false) => write!(f, "<<"),
            BinaryOp::MatrixMultiply(false) => write!(f, "@"),
            BinaryOp::Multiply(false) => write!(f, "*"),
            BinaryOp::Modulo(false) => write!(f, "%"),
            BinaryOp::OR(false) => write!(f, "|"),
            BinaryOp::POWER(false) => write!(f, "**"),
            BinaryOp::ShiftRight(false) => write!(f, ">>"),
            BinaryOp::Minus(false) => write!(f, "-"),
            BinaryOp::Divide(false) => write!(f, "/"),
            BinaryOp::XOR(false) => write!(f, "^"),
            BinaryOp::Add(true) => write!(f, "+="),
            BinaryOp::And(true) => write!(f, "&="),
            BinaryOp::FloorDivide(true) => write!(f, "//="),
            BinaryOp::ShiftLeft(true) => write!(f, "<<="),
            BinaryOp::MatrixMultiply(true) => write!(f, "@="),
            BinaryOp::Multiply(true) => write!(f, "*="),
            BinaryOp::Modulo(true) => write!(f, "%="),
            BinaryOp::OR(true) => write!(f, "|="),
            BinaryOp::POWER(true) => write!(f, "**="),
            BinaryOp::ShiftRight(true) => write!(f, ">>="),
            BinaryOp::Minus(true) => write!(f, "-="),
            BinaryOp::Divide(true) => write!(f, "/="),
            BinaryOp::XOR(true) => write!(f, "^="),
        }
    }
}

impl From<u8> for BinaryOp {
    fn from(value: u8) -> Self {
        match value {
            0 => BinaryOp::Add(false),
            1 => BinaryOp::And(false),
            2 => BinaryOp::FloorDivide(false),
            3 => BinaryOp::ShiftLeft(false),
            4 => BinaryOp::MatrixMultiply(false),
            5 => BinaryOp::Multiply(false),
            6 => BinaryOp::Modulo(false),
            7 => BinaryOp::OR(false),
            8 => BinaryOp::POWER(false),
            9 => BinaryOp::ShiftRight(false),
            10 => BinaryOp::Minus(false),
            11 => BinaryOp::Divide(false),
            12 => BinaryOp::XOR(false),
            13 => BinaryOp::Add(true),
            14 => BinaryOp::And(true),
            15 => BinaryOp::FloorDivide(true),
            16 => BinaryOp::ShiftLeft(true),
            17 => BinaryOp::MatrixMultiply(true),
            18 => BinaryOp::Multiply(true),
            19 => BinaryOp::Modulo(true),
            20 => BinaryOp::OR(true),
            21 => BinaryOp::POWER(true),
            22 => BinaryOp::ShiftRight(true),
            23 => BinaryOp::Minus(true),
            24 => BinaryOp::Divide(true),
            25 => BinaryOp::XOR(true),
            v => panic!("unknown binary op {}", v)
        }

    }
}

impl BinaryOp {
    pub fn handle(&self, lhs: PyObject, rhs: PyObject) -> PyObject {
        match self {
            BinaryOp::Add(_) => {
                // FIXME: when it is BinaryOp::Add(true), we should operate in-place
                // but the PyObject is a Rc, which is immutable
                // we should use RefCell
                if let Ok(lhs) = lhs.clone().downcast_rc::<IntObject>() {
                    if let Ok(rhs) = rhs.clone().downcast_rc::<IntObject>() {
                        return IntObject::new_from_i32(lhs.value() + rhs.value())
                    }
                }
                panic!("cannot perform add on {lhs:?} and {rhs:?}");
            },
            BinaryOp::And(_) => unimplemented!(),
            BinaryOp::FloorDivide(_) => unimplemented!(),
            BinaryOp::ShiftLeft(_) => unimplemented!(),
            BinaryOp::MatrixMultiply(_) => unimplemented!(),
            BinaryOp::Multiply(_) => unimplemented!(),
            BinaryOp::Modulo(_) => unimplemented!(),
            BinaryOp::OR(_) => unimplemented!(),
            BinaryOp::POWER(_) => unimplemented!(),
            BinaryOp::ShiftRight(_) => unimplemented!(),
            BinaryOp::Minus(_) => unimplemented!(),
            BinaryOp::Divide(_) => unimplemented!(),
            BinaryOp::XOR(_) => unimplemented!(),
        }
    }
}
