use crate::object::BasePycObject;
use crate::object::PycObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

pub struct SetObject {
    base: BasePycObject,
    values: Vec<Box<dyn PycObject>>
}

impl SetObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Self {
        let length = stream.read_int().unwrap();
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream, magic));
        }
        Self {
            base: BasePycObject::new_from_char('<'),
            values
        }
    }
}

impl PycObject for SetObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for SetObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "SetObject({:?})", self.values)
    }
}