use crate::object::{BasePycObject, CodeObject};
use crate::object::PyObject;
use crate::object::ObjectType;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

type NativeFn = Box<dyn Fn(Vec<Rc<dyn PyObject>>) -> Rc<dyn PyObject>>;

pub struct CallableObject {
    base: BasePycObject,
    code: Option<Rc<CodeObject>>,
    defaults: Vec<Rc<dyn PyObject>>,
    is_native: bool,
    native_fn: Option<NativeFn>
}

impl CallableObject {
    pub fn new(code: Rc<CodeObject>, defaults: Vec<Rc<dyn PyObject>>) -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('C'),
            code: Some(code),
            defaults,
            is_native: false,
            native_fn: None
        })
    }

    pub fn new_native(f: NativeFn) -> Rc<Self> {
        Rc::new(Self {
            base: BasePycObject::new_from_char('C'),
            code: None,
            defaults: vec![],
            is_native: true,
            native_fn: Some(f),
        })
    }

    pub fn code(&self) -> Rc<CodeObject>{
        assert!(!self.is_native);
        self.code.clone().unwrap().clone()
    }
    pub fn defaults(&self) -> &Vec<Rc<dyn PyObject>>{
        assert!(!self.is_native);
        &self.defaults
    }
    pub fn call_native(&self, args: Vec<Rc<dyn PyObject>>) -> Rc<dyn PyObject> {
        assert!(self.is_native);
        assert!(self.native_fn.is_some());
        let f = self.native_fn.as_ref().unwrap();
        f(args)
    }

    pub fn is_native(&self) -> bool {
        self.is_native
    }
}

impl PartialEq<Self> for CallableObject {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for CallableObject {}

impl Hash for CallableObject {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        panic!("{}", format!("cannot hash {:?}", self.object_type()))
    }
}

impl PyObject for CallableObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Display for CallableObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "")
    }
}
impl fmt::Debug for CallableObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "CallableObject")
    }
}