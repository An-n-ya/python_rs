use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};

pub struct TrueObject {
    base: BasePycObject,
}

impl TrueObject {
    pub fn new() -> Self {
        Self {
            base: BasePycObject::new_from_char('T'),
        }
    }
}

impl PartialEq<Self> for TrueObject {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for TrueObject {}

impl Hash for TrueObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}

impl PyObject for TrueObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for TrueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "TrueObject")
    }
}