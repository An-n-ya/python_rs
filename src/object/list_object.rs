use crate::object::BasePycObject;
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

pub struct ListObject {
    base: BasePycObject,
    values: Vec<Box<dyn PyObject>>
}

impl ListObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Self {
        let length = stream.read_int().unwrap();
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream, magic));
        }
        Self {
            base: BasePycObject::new_from_char('['),
            values
        }
    }
}

impl PyObject for ListObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "ListObject({:?})", self.values)
    }
}