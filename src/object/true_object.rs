use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::utils::PyObject;

pub struct TrueObject {
    base: BasePycObject,
}

impl TrueObject {
    pub fn new() -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('T'),
        })
    }
}

impl PartialEq<Self> for TrueObject {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for TrueObject {}

impl PyObjectTrait for TrueObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }
}

impl fmt::Display for TrueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "True")
    }
}
impl fmt::Debug for TrueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "TrueObject")
    }
}