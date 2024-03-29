use crate::object::{BasePycObject, IterObject};
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::{InputStream, PycParser};
use crate::utils::Magic;

use crate::utils::PyObject;

pub struct TupleObject {
    base: BasePycObject,
    values: Vec<PyObject>
}

impl TupleObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> PyObject {
        let length = stream.read_u32().unwrap();
        BasePycObject::new_py_object(Self::_new(stream, magic, length))
    }

    pub fn new_from_short(stream: &mut InputStream, magic: Magic) -> PyObject {
        let length = stream.read().unwrap();
        BasePycObject::new_py_object(Self::_new(stream, magic, length as u32))
    }

    fn _new(stream: &mut InputStream, magic: Magic, length: u32) -> Self {
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream, magic));
        }
        Self {
            base: BasePycObject::new_from_char('('),
            values
        }
    }

    #[allow(dead_code)]
    pub fn take_values(self) -> Vec<PyObject> {
        self.values
    }
    pub fn values(&self) -> &Vec<PyObject> {
        &self.values
    }
}

impl PartialEq<Self> for TupleObject {
    fn eq(&self, other: &Self) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }

        for i in 0..self.values.len() {
            if self.values.get(i).unwrap() != other.values.get(i).unwrap() {
                return false;
            }
        }

        return true;
    }
}

impl Eq for TupleObject {}

impl Hash for TupleObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
    }
}

impl PyObjectTrait for TupleObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }

    fn hash_key(&self) -> String {
        // TODO: tuple should be hashable
        unimplemented!()
    }

    fn to_iter(&self) -> PyObject {
        IterObject::new(self.values.clone())
    }
}

impl fmt::Display for TupleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "(").unwrap();
        for (i, entry) in self.values.iter().enumerate() {
            if i != 0 {
                write!(f, ", ").unwrap();
            }
            write!(f, "{}", entry.borrow()).unwrap();
        }
        write!(f, ")")
    }
}
impl fmt::Debug for TupleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "TupleObject({:?})", self.values)
    }
}