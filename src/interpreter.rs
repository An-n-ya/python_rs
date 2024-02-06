use std::rc::Rc;
use crate::frame::Frame;
use crate::object::{CodeObject, PyObject as PyObjectTrait};
use crate::utils::ByteCode::*;

type PyObject = Rc<dyn PyObjectTrait>;
pub struct Interpreter {
    cur_frame: Option<Box<Frame>>,
    return_value: Option<PyObject>
}

impl Interpreter {
    pub fn new(code: Rc<CodeObject>) -> Self {
        Self {
            cur_frame: Some(Box::new(Frame::new_from_code(code))),
            return_value: None
        }
    }

    pub fn run(&mut self)  {
        while let Some(cur_frame) = &mut self.cur_frame.take() {
            while !cur_frame.finish() {
                let bytecode = cur_frame.get_byte_code();
                let mut arg = None;
                if bytecode.have_arg() {
                    arg = Some(cur_frame.get_arg());
                }
                cur_frame.skip_codes_of(bytecode.cache_num() as usize);
                if cfg!(test) {
                    println!("interpret bytecode: {:?}", bytecode);
                }

                match bytecode {
                    RETURN_VALUE => {
                        self.return_value = Some(cur_frame.pop());
                        self.next_frame(cur_frame.parent());
                    },
                    LOAD_CONST => {
                        let obj = cur_frame.get_const(arg.unwrap() as usize);
                        cur_frame.push(obj);
                    },
                    LOAD_NAME => {
                        let obj = cur_frame.get_name(arg.unwrap() as usize);
                        cur_frame.push(obj);
                    },
                    STORE_NAME => {
                        let name = cur_frame.get_name(arg.unwrap() as usize);
                        let tos = cur_frame.pop();
                        cur_frame.set_local(name, tos);
                    },
                    RESUME | PRECALL | CACHE => {
                        // nop
                    }
                    _ => {unimplemented!()}
                }
            }
        }
    }

    fn next_frame(&mut self, frame: Option<Box<Frame>>)  {
        self.cur_frame = frame;
    }

    pub fn return_value(&self) -> Option<PyObject> {
        self.return_value.clone()
    }
}