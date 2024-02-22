use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct TrueObject {
    base: BasePycObject,
}

impl TrueObject {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
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

impl Hash for TrueObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}

impl PyObjectTrait for TrueObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
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