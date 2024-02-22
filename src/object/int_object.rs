use std::cmp::Ordering;
use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::InputStream;

pub struct IntObject {
    base: BasePycObject,
    value: i32
}

impl IntObject {
    pub fn new(stream: &mut InputStream) -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('i'),
            value: stream.read_i32().unwrap()
        })
    }

    pub fn new_from_i32(value: i32) -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('i'),
            value
        })
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl PartialOrd for IntObject {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl Ord for IntObject {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialEq<Self> for IntObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for IntObject {}
impl Hash for IntObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}
impl PyObjectTrait for IntObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Display for IntObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}
impl fmt::Debug for IntObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "IntObject({})", self.value)
    }
}