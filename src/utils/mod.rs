mod magic;
mod bytecode;
mod cmp_op;
mod binary_op;

use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
pub use magic::Magic;
pub use bytecode::ByteCode;
pub use cmp_op::CmpOP;
pub use binary_op::BinaryOp;
use std::any::TypeId;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

use crate::object::{CallableObject, CodeObject, DictObject, FalseObject, IntLongObject, IntObject, ListObject, NoneObject, NullObject, PyObjectTrait as PyObjectTrait, SetObject, StringObject, TrueObject, TupleObject};
pub type PyObject = Rc<RefCell<dyn PyObjectTrait>>;

pub trait DowncastTrait {
    fn downcast_refcell<T: PyObjectTrait>(&self) -> Option<Ref<T>>;
    fn downcast_refcell_mut<T: PyObjectTrait>(&self) -> Option<RefMut<T>>;
}

struct DowncastErr {}

impl Debug for DowncastErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "downcast error")
    }
}

impl Display for DowncastErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "downcast error")
    }
}

impl Error for DowncastErr {}

fn is_downcastable(type_id: TypeId) -> bool {
    type_id == TypeId::of::<CodeObject>()
        || type_id == TypeId::of::<CallableObject>()
        || type_id == TypeId::of::<DictObject>()
        || type_id == TypeId::of::<IntLongObject>()
        || type_id == TypeId::of::<FalseObject>()
        || type_id == TypeId::of::<IntObject>()
        || type_id == TypeId::of::<ListObject>()
        || type_id == TypeId::of::<NoneObject>()
        || type_id == TypeId::of::<NullObject>()
        || type_id == TypeId::of::<SetObject>()
        || type_id == TypeId::of::<StringObject>()
        || type_id == TypeId::of::<TrueObject>()
        || type_id == TypeId::of::<TupleObject>()

}

impl DowncastTrait for PyObject {
    fn downcast_refcell<T: PyObjectTrait>(&self) -> Option<Ref<T>> {
        let r = self.borrow();
        let type_id = (*r).type_id();
        if is_downcastable(type_id)
        {
            if r.deref().downcast_ref::<T>().is_none() {
                return None
            } else {
                Some(Ref::map(r, |x| x.downcast_ref::<T>().unwrap()))
            }
        } else {
            None
        }
    }

    fn downcast_refcell_mut<T: PyObjectTrait>(&self) -> Option<RefMut<T>> {
        let r = self.borrow_mut();
        let type_id = (*r).type_id();
        if is_downcastable(type_id)
        {
            if r.deref().downcast_ref::<T>().is_none() {
                return None
            } else {
                Some(RefMut::map(r, |x| x.downcast_mut::<T>().unwrap()))
            }
        } else {
            None
        }

    }
}

