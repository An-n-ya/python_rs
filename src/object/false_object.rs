use crate::object::BasePycObject;
use crate::object::PycObject;
use crate::object::ObjectType;
use std::fmt;

pub struct FalseObject {
    base: BasePycObject,
}

impl FalseObject {
    pub fn new() -> Self {
        Self {
            base: BasePycObject::new_from_char('F'),
        }
    }
}

impl PycObject for FalseObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for FalseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "FalseObject")
    }
}