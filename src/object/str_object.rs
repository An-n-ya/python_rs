use crate::object::{BasePycObject, IterObject};
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::InputStream;
use crate::utils::PyObject;

pub struct StringObject {
    base: BasePycObject,
    data: Vec<u8>
}

impl StringObject {
    pub fn new(stream: &mut InputStream) -> PyObject {
        let length = stream.read_u32().unwrap();
        BasePycObject::new_py_object(Self::_new(stream, length))
    }
    pub fn new_from_short(stream: &mut InputStream) -> PyObject {
        let length = stream.read().unwrap();
        BasePycObject::new_py_object(Self::_new(stream, length as u32))
    }

    pub fn new_from_unicode(stream: &mut InputStream) -> PyObject {
        let length = stream.read_u32().unwrap();
        BasePycObject::new_py_object(Self::_new(stream, length))
    }

    pub fn new_from_str(s: &str) -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('s'),
            data: s.to_string().as_bytes().to_vec(),
        })
    }

    fn _new(stream: &mut InputStream, length: u32) -> Self {
        let mut data = vec![];
        for _ in 0..length {
            data.push(stream.read().unwrap());
        }
        Self {
            base: BasePycObject::new_from_char('s'),
            data,
        }
    }

    pub fn string(&self) -> String {
        // String::from_utf8(self.data.clone()).expect("invalid utf8 string")
        String::from_utf8(self.data.clone()).unwrap_or("".to_string())
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

}

impl PartialEq<Self> for StringObject {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for StringObject {}
impl Hash for StringObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}
impl PyObjectTrait for StringObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }
    fn hash_key(&self) -> String {
        let mut res = "$String_".to_string();
        res.push_str(&self.string());
        res
    }
    fn to_iter(&self) -> PyObject {
        let s = self.string();
        let mut values = vec![];
        for c in s.chars() {
            values.push(StringObject::new_from_str(&c.to_string()));
        }
        IterObject::new(values)
    }
}

impl fmt::Display for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.string())
    }
}

impl fmt::Debug for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "StringObject({:?})", self.string())
    }
}

