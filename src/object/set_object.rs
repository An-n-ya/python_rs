use crate::object::{BasePycObject, TrueObject, TupleObject};
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use crate::{InputStream, PycParser};
use crate::utils::Magic;

pub struct SetObject {
    base: BasePycObject,
    values: Vec<Rc<dyn PyObject>>
}

impl SetObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Rc<Self> {
        let length = stream.read_int().unwrap();
        let mut values = vec![];
        for _ in 0..length {
            values.push(PycParser::marshal_object(stream, magic));
        }
        Rc::new(Self {
            base: BasePycObject::new_from_char('<'),
            values
        })
    }
}

impl PartialEq<Self> for SetObject {
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

impl Eq for SetObject {}
impl Hash for SetObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}
impl PyObject for SetObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for SetObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "SetObject({:?})", self.values)
    }
}