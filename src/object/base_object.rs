use std::fmt::Debug;
use downcast_rs::{Downcast, impl_downcast};
use dyn_eq::DynEq;
use dyn_hash::DynHash;

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
    CALLABLE
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

pub trait PyObject: Debug + Downcast + DynEq + DynHash {
    fn object_type(&self) -> ObjectType;
}
impl_downcast!(PyObject);
dyn_eq::eq_trait_object!(PyObject);
dyn_hash::hash_trait_object!(PyObject);

pub(crate) struct BasePycObject {
    _type: ObjectType
}

impl BasePycObject {
    pub fn new(_type: ObjectType) -> Self {
        Self {
            _type
        }
    }
    pub fn new_from_char(c: char) -> Self {
        let _type: ObjectType = c.into();
        Self::new(_type)
    }

    pub fn object_type(&self) -> ObjectType {
        self._type
    }
}
