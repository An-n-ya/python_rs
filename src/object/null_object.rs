use crate::object::{BasePycObject, TrueObject};
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};

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

impl PartialEq<Self> for NullObject {
    fn eq(&self, other: &Self) -> bool {
        return true;
    }
}

impl Eq for NullObject{}
impl Hash for NullObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
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