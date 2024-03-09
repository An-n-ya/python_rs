use std::cell::{Ref, RefCell};
use crate::object::{BasePycObject, CodeObject};
use crate::object::PyObjectTrait;
use crate::object::ObjectType;
use std::{fmt, mem};
use std::ops::Deref;
use std::rc::Rc;
use crate::utils::PyObject;

type NativeFn = Box<dyn Fn(Vec<PyObject>) -> PyObject>;

pub struct CallableObject {
    base: BasePycObject,
    code: Option<Rc<RefCell<CodeObject>>>,
    defaults: Vec<PyObject>,
    is_native: bool,
    native_fn: Option<NativeFn>
}

impl CallableObject {
    pub fn new(code: Ref<CodeObject>, defaults: Vec<PyObject>) -> PyObject {
        let code = code.deref();
        // FIXME: maybe we should use mem::swap?
        let code:CodeObject = unsafe {mem::transmute_copy(code)};
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('C'),
            code: Some(Rc::new(RefCell::new(code))),
            defaults,
            is_native: false,
            native_fn: None
        })
    }

    pub fn new_native(f: NativeFn) -> PyObject {
        BasePycObject::new_py_object(Self {
            base: BasePycObject::new_from_char('C'),
            code: None,
            defaults: vec![],
            is_native: true,
            native_fn: Some(f),
        })
    }

    pub fn code(&self) -> Rc<RefCell<CodeObject>>{
        assert!(!self.is_native);
        self.code.clone().unwrap().clone()
    }
    pub fn defaults(&self) -> &Vec<PyObject>{
        assert!(!self.is_native);
        &self.defaults
    }
    pub fn call_native(&self, args: Vec<PyObject>) -> PyObject {
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


impl PyObjectTrait for CallableObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
    fn base_object(&self) -> &BasePycObject {
        &self.base
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