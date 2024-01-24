use crate::object::BasePycObject;
use crate::object::PycObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};

pub struct TupleObject {
    base: BasePycObject,
    values: Vec<Box<dyn PycObject>>
}

impl TupleObject {
    pub fn new(stream: &mut InputStream) -> Self {
        let length = stream.read_int().unwrap();
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream));
        }
        Self {
            base: BasePycObject::new_from_char('('),
            values
        }
    }
}

impl PycObject for TupleObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for TupleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "TupleObject({:?})", self.values)
    }
}