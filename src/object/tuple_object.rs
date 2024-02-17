use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

pub struct TupleObject {
    base: BasePycObject,
    values: Vec<Rc<dyn PyObject>>
}

impl TupleObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Rc<Self> {
        let length = stream.read_u32().unwrap();
        Rc::new(Self::_new(stream, magic, length))
    }

    pub fn new_from_short(stream: &mut InputStream, magic: Magic) -> Rc<Self> {
        let length = stream.read().unwrap();
        Rc::new(Self::_new(stream, magic, length as u32))
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
    pub fn take_values(self) -> Vec<Rc<dyn PyObject>> {
        self.values
    }
    pub fn values(&self) -> &Vec<Rc<dyn PyObject>> {
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
        // TODO: tuple should be hashable
        unimplemented!()
    }
}

impl PyObject for TupleObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Display for TupleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "(").unwrap();
        for (i, entry) in self.values.iter().enumerate() {
            if i != 0 {
                write!(f, ", ").unwrap();
            }
            write!(f, "{}", entry).unwrap();
        }
        write!(f, ")")
    }
}
impl fmt::Debug for TupleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "TupleObject({:?})", self.values)
    }
}