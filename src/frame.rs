use std::collections::HashMap;
use std::rc::Rc;
use crate::InputStream;
use crate::object::{CodeObject, PyObject as PyObjectTrait};
use crate::utils::ByteCode;

type PyObject = Rc<dyn PyObjectTrait>;

pub struct Frame {
    stack: Vec<PyObject>,
    code: InputStream,
    locals: HashMap<PyObject, PyObject>,
    globals: HashMap<PyObject, PyObject>,
    names: Vec<PyObject>,
    consts: Vec<PyObject>,
    parent: Option<Box<Frame>>
}

impl Frame {
    pub fn new_from_code(mut code: Rc<CodeObject>) -> Self {
        Self {
            stack: vec![],
            code: InputStream::new(code.code()),
            locals: Default::default(),
            globals: Default::default(),
            names: code.names(),
            consts: code.consts(),
            parent: None
        }
    }

    pub fn finish(&self) -> bool {
        self.code.finish()
    }

    pub fn get_byte_code(&mut self) -> ByteCode {
        self.code.read().unwrap().into()
    }

    pub fn get_arg(&mut self) -> u8 {
        self.code.read().unwrap()
    }

    pub fn skip_codes_of(&mut self, n: usize) {
        for _ in 0..n {
            self.code.read().unwrap();
        }
    }

    pub fn parent(&mut self) -> Option<Box<Frame>> {
        self.parent.take()
    }

    pub fn pop(&mut self) -> PyObject {
        self.stack.pop().take().unwrap()
    }

    pub fn push(&mut self, obj: PyObject) {
        self.stack.push(obj)
    }

    pub fn get_const(&self, index: usize) -> PyObject {
        self.consts.get(index).unwrap().clone()
    }
    pub fn get_name(&self, index: usize) -> PyObject {
        self.names.get(index).unwrap().clone()
    }

    pub fn set_local(&mut self, key: PyObject, value: PyObject) {
        self.locals.insert(key, value);
    }
}