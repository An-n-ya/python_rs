use crate::object::{BasePycObject, NullObject, TrueObject};
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};

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

impl PartialEq<Self> for NoneObject {
    fn eq(&self, other: &Self) -> bool {
        return true;
    }
}

impl Eq for NoneObject{}
impl Hash for NoneObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
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