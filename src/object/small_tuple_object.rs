use crate::object::BasePycObject;
use crate::object::PycObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};

pub struct SmallTupleObject {
    base: BasePycObject,
    values: Vec<Box<dyn PycObject>>
}

impl SmallTupleObject {
    pub fn new(stream: &mut InputStream) -> Self {
        let length = stream.read().unwrap();
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream));
        }
        Self {
            base: BasePycObject::new_from_char('('),
            values
        }
    }

    pub fn values(&self) -> &Vec<Box<dyn PycObject>> {
        &self.values
    }
    pub fn take_values(self) -> Vec<Box<dyn PycObject>> {
        self.values
    }
}

impl PycObject for SmallTupleObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for SmallTupleObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "SmallTupleObject({:?})", self.values)
    }
}