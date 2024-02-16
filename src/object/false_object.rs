use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct FalseObject {
    base: BasePycObject,
}

impl FalseObject {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('F'),
        })
    }
}

impl Hash for FalseObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}
impl PartialEq<Self> for FalseObject {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for FalseObject {}
impl PyObject for FalseObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
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