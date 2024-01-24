use crate::object::BasePycObject;
use crate::object::PycObject;
use crate::object::ObjectType;
use std::fmt;

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

impl PycObject for TrueObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for TrueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "TrueObject")
    }
}