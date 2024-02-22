mod magic;
mod bytecode;
mod cmp_op;
mod binary_op;

use std::cell::RefCell;
use std::rc::Rc;
pub use magic::Magic;
pub use bytecode::ByteCode;
pub use cmp_op::CmpOP;
pub use binary_op::BinaryOp;

use crate::object::PyObjectTrait as PyObjectTrait;
pub type PyObject = Rc<dyn PyObjectTrait>;
