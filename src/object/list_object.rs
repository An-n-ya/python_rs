use crate::object::BasePycObject;
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

use crate::utils::PyObject;

pub struct ListObject {
    base: BasePycObject,
    values: Vec<PyObject>
}

impl ListObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> PyObject {
        let length = stream.read_u32().unwrap();
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream, magic));
        }
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('['),
            values
        })
    }
}

impl PartialEq<Self> for ListObject {
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

impl Eq for ListObject{}
impl PyObjectTrait for ListObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Display for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "[").unwrap();
        for (i, entry) in self.values.iter().enumerate() {
            if i != 0 {
                write!(f, ", ").unwrap();
            }
            write!(f, "{}", entry.borrow()).unwrap();
        }
        write!(f, "]")
    }
}
impl fmt::Debug for ListObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "ListObject({:?})", self.values)
    }
}