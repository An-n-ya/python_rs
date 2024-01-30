use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

pub struct SmallTupleObject {
    base: BasePycObject,
    values: Vec<Box<dyn PyObject>>
}

impl SmallTupleObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Self {
        let length = stream.read().unwrap();
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream, magic));
        }
        Self {
            base: BasePycObject::new_from_char('('),
            values
        }
    }

    pub fn values(&self) -> &Vec<Box<dyn PyObject>> {
        &self.values
    }
    pub fn take_values(self) -> Vec<Box<dyn PyObject>> {
        self.values
    }
}

impl PyObject for SmallTupleObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for SmallTupleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "SmallTupleObject({:?})", self.values)
    }
}