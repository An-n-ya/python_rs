use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;

pub struct NullObject {
    base: BasePycObject,
}

impl NullObject {
    pub fn new() -> Self {
        Self {
            base: BasePycObject::new_from_char('0'),
        }
    }
}

impl PyObject for NullObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for NullObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "NullObject")
    }
}