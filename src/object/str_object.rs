use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::InputStream;

pub struct StringObject {
    base: BasePycObject,
    data: Vec<u8>
}

impl StringObject {
    pub fn new(stream: &mut InputStream) -> Rc<Self> {
        let length = stream.read_int().unwrap();
        Rc::new(Self::_new(stream, length))
    }
    pub fn new_from_short(stream: &mut InputStream) -> Rc<Self> {
        let length = stream.read().unwrap();
        Rc::new(Self::_new(stream, length as u32))
    }

    pub fn new_from_unicode(stream: &mut InputStream) -> Rc<Self> {
        let length = stream.read_int().unwrap();
        Rc::new(Self::_new(stream, length))
    }

    pub fn new_from_str(s: &str) -> Rc<Self> {
        Rc::new(Self {
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
        String::from_utf8(self.data.clone()).expect("invalid utf8 string")
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
impl PyObject for StringObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for StringObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "StringObject({:?})", self.string())
    }
}

