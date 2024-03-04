use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::InputStream;
use crate::utils::PyObject;

pub struct IntLongObject {
    base: BasePycObject,
    value: i64
}

impl IntLongObject {
    pub fn new(stream: &mut InputStream) -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('I'),
            value: stream.read_i64().unwrap()
        })
    }
}

impl PartialEq<Self> for IntLongObject {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for IntLongObject{}
impl PyObjectTrait for IntLongObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }

    fn hash_key(&self) -> String {
        let mut res = "$IntLong_".to_string();
        res.push_str(&self.value.to_string());
        res
    }
}

impl fmt::Debug for IntLongObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "IntLongObject({})", self.value)
    }
}
impl fmt::Display for IntLongObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}