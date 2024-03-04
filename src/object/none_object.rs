use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::utils::PyObject;

pub struct NoneObject {
    base: BasePycObject,
}

impl NoneObject {
    pub fn new() -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('N'),
        })
    }
    #[allow(dead_code)]
    pub fn new_raw() -> Self {
        Self {
            base: BasePycObject::new_from_char('N'),
        }
    }
}

impl PartialEq<Self> for NoneObject {
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Eq for NoneObject{}
impl PyObjectTrait for NoneObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for NoneObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "NoneObject")
    }
}
impl fmt::Display for NoneObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "None")
    }
}