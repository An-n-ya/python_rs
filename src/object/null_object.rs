use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::utils::PyObject;

pub struct NullObject {
    base: BasePycObject,
}

impl NullObject {
    pub fn new() -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('0'),
        })
    }
}

impl PartialEq<Self> for NullObject {
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Eq for NullObject{}
impl PyObjectTrait for NullObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }

    fn is_null(&self) -> bool {
        true
    }
}

impl fmt::Display for NullObject {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // We shouldn't print NullObject
        panic!("cannot print NullObject")
    }
}
impl fmt::Debug for NullObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "NullObject")
    }
}