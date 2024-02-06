use crate::object::{BasePycObject, TrueObject};
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::os::linux::raw::stat;
use std::rc::Rc;
use crate::InputStream;

pub struct IntLongObject {
    base: BasePycObject,
    value: u64
}

impl IntLongObject {
    pub fn new(stream: &mut InputStream) -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('I'),
            value: stream.read_long().unwrap()
        })
    }
}

impl PartialEq<Self> for IntLongObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Hash for IntLongObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
impl Eq for IntLongObject{}
impl PyObject for IntLongObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for IntLongObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "IntLongObject({})", self.value)
    }
}