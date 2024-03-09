use crate::object::{BasePycObject};
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::{Magic, PyObject};

pub struct DictObject {
    base: BasePycObject,
    entries: Vec<DictEntry>
}

struct DictEntry {
    key: PyObject,
    value: PyObject,
}


impl DictObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> PyObject {
        let mut entries = vec![];
        loop {
            let key = PycParser::marshal_object(stream, magic);
            if key.borrow().object_type() == ObjectType::NULL {
                break
            }
            let value = PycParser::marshal_object(stream, magic);
            entries.push(DictEntry{key, value});
        }
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('{'),
            entries
        })
    }
}

impl PartialEq<Self> for DictObject {
    fn eq(&self, other: &Self) -> bool {
        // FIXME: should we compare every entry?
        self == other
    }
}

impl Eq for DictObject {}
impl PyObjectTrait for DictObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }
}

impl fmt::Debug for DictObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "DictObject({{").unwrap();
        // FIXME: considering the nested Dict situation
        for entry in &self.entries {
            writeln!(f, "   {:?}: {:?}", entry.key, entry.value).unwrap();
        }
        writeln!(f, "}})")
    }
}
impl fmt::Display for DictObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{{").unwrap();
        for (i, entry) in self.entries.iter().enumerate() {
            if i != 0 {
                write!(f, ", ").unwrap();
            }
            write!(f, "{}: {}", entry.key.borrow(), entry.value.borrow()).unwrap();
        }
        write!(f, "}}")
    }
}
