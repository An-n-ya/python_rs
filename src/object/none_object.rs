use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;

pub struct NoneObject {
    base: BasePycObject,
}

impl NoneObject {
    pub fn new() -> Self {
        Self {
            base: BasePycObject::new_from_char('N'),
        }
    }
}

impl PyObject for NoneObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for NoneObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "NoneObject")
    }
}