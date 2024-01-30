use crate::frame::Frame;
use crate::object::{CodeObject, PyObject as PyObjectTrait};
use crate::utils::ByteCode::*;

type PyObject = Box<dyn PyObjectTrait>;
struct Interpreter {
    cur_frame: Option<Box<Frame>>
}

impl Interpreter {
    pub fn new(code: CodeObject) -> Self {
        Self {
            cur_frame: Some(Box::new(Frame::new_from_code(code)))
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

                match bytecode {
                    RETURN_VALUE => {
                        self.next_frame();
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
                    }
                    _ => {unimplemented!()}
                }
            }
        }
    }

    fn next_frame(&mut self)  {
        let mut frame = self.cur_frame.take().unwrap();
        let next_frame = frame.parent();
        self.cur_frame = next_frame;
    }
}