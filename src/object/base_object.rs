use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use downcast_rs::{Downcast, impl_downcast};
use dyn_eq::DynEq;
use crate::utils::PyObject;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[allow(unused, non_camel_case_types)]
pub enum ObjectType{
    NULL               , // NULL often means error
    NONE               , // None is an object of undefined type
    FALSE              ,
    TRUE               ,
    STOPITER           ,
    ELLIPSIS           ,
    INT                ,
    INT64              ,
    FLOAT              ,
    BINARY_FLOAT       ,
    COMPLEX            ,
    BINARY_COMPLEX     ,
    LONG               ,
    STRING             ,
    INTERNED           ,
    REF                ,
    STRINGREF          ,
    TUPLE              ,
    LIST               ,
    DICT               ,
    CODE               ,
    UNICODE            ,
    UNKNOWN            ,
    SET                ,
    FROZENSET          ,
    ASCII              ,
    ASCII_INTERNED     ,
    SMALL_TUPLE        ,
    SHORT_ASCII        ,
    SHORT_ASCII_INTERNED ,

    // runtime object
    CALLABLE,
    ITER
}
impl Default for ObjectType {
    fn default() -> Self {
        Self::NULL
    }
}

const FLAG_REF: u8 = 0x80;

impl From<char> for ObjectType {
    fn from(value: char) -> Self {
        let value = ((value as u8) & !FLAG_REF) as char;
        match value {
            '0' => ObjectType::NULL,
            'N' => ObjectType::NONE,
            'F' => ObjectType::FALSE,
            'T' => ObjectType::TRUE,
            'S' => ObjectType::STOPITER,
            '.' => ObjectType::ELLIPSIS,
            'i' => ObjectType::INT,
            'I' => ObjectType::INT64,
            'f' => ObjectType::FLOAT,
            'g' => ObjectType::BINARY_FLOAT,
            'x' => ObjectType::COMPLEX,
            'y' => ObjectType::BINARY_COMPLEX,
            'l' => ObjectType::LONG,
            's' => ObjectType::STRING,
            't' => ObjectType::INTERNED,
            'r' => ObjectType::REF,
            'R' => ObjectType::STRINGREF,
            '(' => ObjectType::TUPLE,
            '[' => ObjectType::LIST,
            '{' => ObjectType::DICT,
            'c' => ObjectType::CODE,
            'C' => ObjectType::CALLABLE,
            'u' => ObjectType::UNICODE,
            '?' => ObjectType::UNKNOWN,
            '<' => ObjectType::SET,
            '>' => ObjectType::FROZENSET,
            'a' => ObjectType::ASCII,
            'A' => ObjectType::ASCII_INTERNED,
            ')' => ObjectType::SMALL_TUPLE,
            'z' => ObjectType::SHORT_ASCII,
            'Z' => ObjectType::SHORT_ASCII_INTERNED,
            _ => panic!("invalid object type {}", value)
        }
    }
}

pub trait PyObjectTrait: Debug + Display + Downcast + DynEq {
    fn object_type(&self) -> ObjectType;
    fn base_object(&self) -> &BasePycObject;
    fn hash_key(&self) -> String {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }

    fn to_iter(&self) -> PyObject {
        panic!("{}", format!("cannot convert {:?} to iter object", self.object_type()))
    }

    fn get_attr(&self, name: String) -> PyObject {
        if let Some(attr) = self.base_object().method.get(&("$String_".to_string() + &name)) {
            return attr.clone()
        }
        panic!("{}", format!("object {:?} doesn't have attribute {}", self.object_type(), name))
    }

    fn is_null(&self) -> bool {
        false
    }
}
impl_downcast!(PyObjectTrait);
dyn_eq::eq_trait_object!(PyObjectTrait);

#[derive(Default)]
pub struct BasePycObject {
    _type: ObjectType,
    method: HashMap<String, PyObject>
}

impl BasePycObject {
    pub fn new(_type: ObjectType) -> Self {
        Self {
            _type,
            method: HashMap::new()
        }
    }

    pub fn new_from_char(c: char) -> Self {
        let _type: ObjectType = c.into();
        Self::new(_type)
    }

    pub fn insert_method(&mut self, key: &str, callable: PyObject) {
        self.method.insert("$String_".to_string() + key, callable);
    }

    pub fn new_py_object<T>(obj: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(obj))
    }

    pub fn object_type(&self) -> ObjectType {
        self._type
    }

}
