use std::collections::HashMap;
use crate::InputStream;
use crate::object::{CodeObject, PyObject as PyObjectTrait};

type PyObject = Box<dyn PyObjectTrait>;
struct Interpreter {
    code: InputStream,
    stack: Vec<PyObject>,
    constants: HashMap<u8, PyObject>,
    names: HashMap<u8, PyObject>
}

impl Interpreter {
    pub fn new(code: CodeObject) -> Self {
        Self {
            code: InputStream::new(code.code()),
            stack: vec![],
            constants: Default::default(),
            names: Default::default(),
        }
    }

    pub fn run(&mut self) {

    }
}