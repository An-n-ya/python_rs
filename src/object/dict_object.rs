use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

pub struct DictObject {
    base: BasePycObject,
    entries: Vec<DictEntry>
}

struct DictEntry {
    key: Box<dyn PyObject>,
    value: Box<dyn PyObject>,
}


impl DictObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Self {
        let mut entries = vec![];
        loop {
            let key = PycParser::marshal_object(stream, magic);
            if key.object_type() == ObjectType::NULL {
                break
            }
            let value = PycParser::marshal_object(stream, magic);
            entries.push(DictEntry{key, value});
        }
        Self {
            base: BasePycObject::new_from_char('{'),
            entries
        }
    }
}

impl PyObject for DictObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
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