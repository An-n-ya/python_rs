use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use crate::InputStream;
use crate::object::{CallableObject, CodeObject};
use crate::utils::ByteCode;
use crate::utils::PyObject;


pub struct Frame {
    stack: Vec<PyObject>,
    code: InputStream,
    locals: HashMap<String, PyObject>,
    fast_locals: HashMap<u8, PyObject>,
    globals: HashMap<String, PyObject>,
    names: Vec<PyObject>,
    consts: Vec<PyObject>,
    parent: Option<Box<Frame>>
}

impl Frame {
    pub fn new_from_code(code: Rc<RefCell<CodeObject>>) -> Self {
        let code = code.borrow();
        Self {
            stack: Vec::with_capacity(code.num_stack() as usize),
            code: InputStream::new(code.code()),
            locals: Default::default(),
            globals: Default::default(),
            fast_locals: Default::default(),
            names: code.names(),
            consts: code.consts(),
            parent: None
        }
    }

    pub fn new_from_callable(callable: Ref<CallableObject>, args: Vec<PyObject>) -> Self {
        let mut fast_locals: HashMap<u8, PyObject> = HashMap::new();
        let code = callable.code();
        let code = code.borrow();
        let num_args = code.num_args();
        let default_args = callable.defaults().len();
        let mut j = num_args;
        for i in (0..default_args).rev() {
            j -= 1;
            fast_locals.insert(j as u8, callable.defaults().get(i).unwrap().clone());
        }
        for i in 0..args.len() {
            fast_locals.insert(i as u8, args.get(i).unwrap().clone());
        }

        Self {
            stack: vec![],
            code: InputStream::new(code.code()),
            locals: Default::default(),
            globals: Default::default(),
            fast_locals,
            names: code.names(),
            consts: code.consts(),
            parent: None,
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

    #[allow(dead_code)]
    pub fn skip_codes_of(&mut self, n: usize) {
        for _ in 0..n {
            self.code.read().unwrap();
        }
    }

    #[allow(dead_code)]
    pub fn backward_code(&mut self, n: usize) {
        self.code.unread(n);
    }
    pub fn jump_offset(&mut self, n: i64) {
        self.code.jump_offset(n);
    }
    #[allow(dead_code)]
    pub fn forward_code(&mut self, n: usize) {
        self.code.forward(n);
    }

    pub fn parent(&mut self) -> Option<Box<Frame>> {
        self.parent.take()
    }
    pub fn set_parent(&mut self, parent: Option<Box<Frame>>) {
        self.parent = parent;
    }

    pub fn pop(&mut self) -> PyObject {
        self.stack.pop().take().unwrap()
    }

    pub fn top(&mut self) -> PyObject{
        self.stack.last().unwrap().clone()
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

    pub fn load_fast(&self, key: u8) -> PyObject {
        self.fast_locals.get(&key).unwrap().clone()
    }

    pub fn set_local(&mut self, key: PyObject, value: PyObject) {
        self.locals.insert(key.borrow().hash_key(), value);
    }

    pub fn look_up_name(&self, name: PyObject) -> Option<PyObject> {
        if let Some(res) = self.locals.get(&name.borrow().hash_key()) {
            return Some(res.clone());
        }
        if let Some(res) = self.globals.get(&name.borrow().hash_key()) {
            return Some(res.clone());
        }
        None
    }

    pub fn look_up_global(&self, name: PyObject) -> Option<PyObject> {
        if let Some(res) = self.globals.get(&name.borrow().hash_key()) {
            return Some(res.clone());
        }
        None
    }
}