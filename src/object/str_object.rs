use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::os::linux::raw::stat;
use crate::InputStream;

pub struct StringObject {
    base: BasePycObject,
    data: Vec<u8>
}

impl StringObject {
    pub fn new(stream: &mut InputStream) -> Self {
        let length = stream.read_int().unwrap();
        Self::_new(stream, length)
    }
    pub fn new_from_short(stream: &mut InputStream) -> Self {
        let length = stream.read().unwrap();
        Self::_new(stream, length as u32)
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

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.data.clone()).expect("invalid utf8 data")
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
impl PyObject for StringObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "StringObject({:?})", self.to_string())
    }
}

pub struct UnicodeObject {
    base: BasePycObject,
    data: String
}

impl crate::object::UnicodeObject {
    pub fn new(stream: &mut InputStream) -> Self {
        let length = stream.read_int().unwrap();
        let mut data = vec![];
        for _ in 0..length {
            data.push(stream.read().unwrap());
        }
        let s = String::from_utf8(data).expect("invalid unicode stream");
        Self {
            base: BasePycObject::new_from_char('u'),
            data: s,
        }
    }
}

impl PartialEq<Self> for UnicodeObject {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for UnicodeObject {}
impl Hash for UnicodeObject {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state)
    }
}
impl PyObject for UnicodeObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for UnicodeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "UnicodeObject({:?})", self.data)
    }
}
