use crate::object::{BasePycObject, CallableObject, NullObject};
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use std::ops::DerefMut;
use crate::utils::DowncastTrait;

use crate::utils::PyObject;

pub struct IterObject {
    base: BasePycObject,
    cur_index: usize,
    values: Vec<PyObject>
}
pub fn iter_next(args: Vec<PyObject>) -> PyObject {
    let object = args[0].clone();
    let mut iter = object.downcast_refcell_mut::<IterObject>().expect("invalid iter object");
    let iter = iter.deref_mut();

    if let Some (obj) = iter.next() {
        obj
    } else {
        NullObject::new()
    }
}

impl IterObject {
    pub fn new(values: Vec<PyObject>) -> PyObject {
        let mut base = BasePycObject::new(ObjectType::ITER);
        base.insert_method("next", CallableObject::new_native(Box::new(iter_next)));
        BasePycObject::new_py_object(Self {
            base,
            cur_index: 0,
            values
        })
    }

    pub fn next(&mut self) -> Option<PyObject> {
        if self.cur_index >= self.values.len() {
            return None;
        }
        let res = self.values[self.cur_index].clone();
        self.cur_index += 1;
        Some(res)
    }

}

impl PartialEq<Self> for IterObject {
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

impl Eq for IterObject {}

impl PyObjectTrait for IterObject {
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
}

impl fmt::Display for IterObject {
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
impl fmt::Debug for IterObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "IterObject({:?})", self.values)
    }
}