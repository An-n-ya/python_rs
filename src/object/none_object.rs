use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct NoneObject {
    base: BasePycObject,
}

impl NoneObject {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('N'),
        })
    }
}

impl PartialEq<Self> for NoneObject {
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Eq for NoneObject{}
impl Hash for NoneObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}
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