use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub struct NullObject {
    base: BasePycObject,
}

impl NullObject {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('0'),
        })
    }
}

impl PartialEq<Self> for NullObject {
    fn eq(&self, _other: &Self) -> bool {
        return true;
    }
}

impl Eq for NullObject{}
impl Hash for NullObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}
impl PyObjectTrait for NullObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Display for NullObject {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        // We shouldn't print NullObject
        panic!("cannot print NullObject")
    }
}
impl fmt::Debug for NullObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "NullObject")
    }
}