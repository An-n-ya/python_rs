use crate::object::{BasePycObject, IterObject, TupleObject};
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::{fmt, mem};
use std::ops::DerefMut;
use crate::{InputStream, PycParser};
use crate::utils::{DowncastTrait, Magic};

use crate::utils::PyObject;

#[derive(Default)]
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

    pub fn new_from_values(values: Vec<PyObject>) -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('['),
            values
        })
    }

    pub fn extend(base: PyObject, other: PyObject) -> PyObject {
        let mut base = base.downcast_refcell_mut::<ListObject>().expect("base object should be ListObject!");
        let base = base.deref_mut();
        if let Some(item) = other.downcast_refcell::<TupleObject>() {
            Self::extend_impl(base, item.values())
        } else if let Some(item) = other.downcast_refcell::<ListObject>() {
            Self::extend_impl(base, &item.values)
        } else {
            panic!("the extend object must be list or tuple");
        }
    }

    fn extend_impl(base: &mut ListObject, values: &Vec<PyObject>) -> PyObject {
        for item in values {
            base.values.push(item.clone());
        }
        let mut res = ListObject::default();
        mem::swap(&mut res, base);
        BasePycObject::new_py_object(res)

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
    fn base_object(&self) -> &BasePycObject {
        &self.base
    }

    fn to_iter(&self) -> PyObject {
        IterObject::new(self.values.clone())
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