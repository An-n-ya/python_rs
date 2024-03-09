use std::cmp::Ordering;
use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::InputStream;
use crate::utils::PyObject;

pub struct IntObject {
    base: BasePycObject,
    value: i32
}

impl IntObject {
    pub fn new(stream: &mut InputStream) -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('i'),
            value: stream.read_i32().unwrap()
        })
    }

    pub fn new_from_i32(value: i32) -> PyObject {
        BasePycObject::new_py_object(Self {
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
impl PyObjectTrait for IntObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }

    fn hash_key(&self) -> String {
        let mut res = "$Int_".to_string();
        res.push_str(&self.value.to_string());
        res
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