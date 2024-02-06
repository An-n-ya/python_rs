use crate::object::{BasePycObject, TrueObject};
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

pub struct DictObject {
    base: BasePycObject,
    entries: Vec<DictEntry>
}

struct DictEntry {
    key: Rc<dyn PyObject>,
    value: Rc<dyn PyObject>,
}


impl DictObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Rc<Self> {
        let mut entries = vec![];
        loop {
            let key = PycParser::marshal_object(stream, magic);
            if key.object_type() == ObjectType::NULL {
                break
            }
            let value = PycParser::marshal_object(stream, magic);
            entries.push(DictEntry{key, value});
        }
        Rc::new(Self {
            base: BasePycObject::new_from_char('{'),
            entries
        })
    }
}

impl Hash for DictObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}
impl PartialEq<Self> for DictObject {
    fn eq(&self, other: &Self) -> bool {
        // FIXME: should we compare every entry?
        self == other
    }
}

impl Eq for DictObject {}
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