use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use crate::InputStream;

pub struct IntLongObject {
    base: BasePycObject,
    value: u64
}

impl IntLongObject {
    pub fn new(stream: &mut InputStream) -> Self {
        Self {
            base: BasePycObject::new_from_char('I'),
            value: stream.read_long().unwrap()
        }
    }
}

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