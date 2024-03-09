use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::utils::PyObject;

pub struct FalseObject {
    base: BasePycObject,
}

impl FalseObject {
    pub fn new() -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('F'),
        })
    }
}
impl PartialEq<Self> for FalseObject {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for FalseObject {}
impl PyObjectTrait for FalseObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }
}

impl fmt::Debug for FalseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "FalseObject")
    }
}
impl fmt::Display for FalseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "False")
    }
}