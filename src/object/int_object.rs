use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::InputStream;

pub struct IntObject {
    base: BasePycObject,
    value: u32
}

impl IntObject {
    pub fn new(stream: &mut InputStream) -> Self {
        Self {
            base: BasePycObject::new_from_char('i'),
            value: stream.read_int().unwrap()
        }
    }
}

impl PartialEq<Self> for IntObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for IntObject {}
impl Hash for IntObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}
impl PyObject for IntObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for IntObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "IntObject({})", self.value)
    }
}