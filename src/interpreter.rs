use std::collections::HashMap;
use std::rc::Rc;
use crate::frame::Frame as FrameRaw;
use crate::object::{CallableObject, CodeObject, FalseObject, NoneObject, NullObject, PyObject as PyObjectTrait, StringObject, TrueObject};
use crate::utils::ByteCode::*;

type PyObject = Rc<dyn PyObjectTrait>;
type Frame = Option<Box<FrameRaw>>;
pub struct Interpreter {
    cur_frame: Frame,
    return_value: Option<PyObject>,
    builtins: HashMap<PyObject, PyObject>
}

fn native_print(args: Vec<PyObject>) -> PyObject {
    for arg in args {
        // FIXME: print problem in function.py
        print!("{}", arg);
        // print!("hello");
    }
    println!();
    NoneObject::new()
}

impl Interpreter {
    pub fn new(code: Rc<CodeObject>) -> Self {
        let builtins = Self::setup_builtins();

        Self {
            cur_frame: Some(Box::new(FrameRaw::new_from_code(code))),
            return_value: None,
            builtins
        }
    }

    fn setup_builtins() -> HashMap<PyObject, PyObject> {
        let mut builtins: HashMap<PyObject, PyObject> = HashMap::new();
        builtins.insert(StringObject::new_from_str("True"), TrueObject::new());
        builtins.insert(StringObject::new_from_str("False"), FalseObject::new());
        builtins.insert(StringObject::new_from_str("None"), NoneObject::new());
        builtins.insert(StringObject::new_from_str("print"), CallableObject::new_native(Box::new(native_print)));

        builtins
    }

    pub fn run(&mut self)  {
        while let Some(mut cur_frame) = self.cur_frame.take() {
            let mut next_frame = None;
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
                    CALL => {
                        let arg = arg.unwrap();
                        let mut fn_arg: Vec<PyObject> = vec![];
                        for _ in 0..arg {
                            fn_arg.push(cur_frame.pop())
                        }
                        let obj = cur_frame.pop();
                        let obj2 = cur_frame.pop();
                        if obj2.downcast_rc::<NullObject>().is_ok() {
                            let callable = obj.downcast_rc::<CallableObject>().expect("invalid callable object");
                            if !callable.is_native() {
                                next_frame = Some(Box::new(FrameRaw::new_from_callable(callable, fn_arg)));
                                break
                            } else {
                                let return_value = callable.call_native(fn_arg);
                                cur_frame.push(return_value);
                            }
                        } else {
                            unimplemented!("method unimplemented")
                        }
                    },
                    MAKE_FUNCTION => {
                        let code = cur_frame.pop().downcast_rc::<CodeObject>().expect("invalid code object");
                        let arg = arg.unwrap();
                        let mut defaults: Vec<PyObject> = vec![];
                        for _ in 0..arg {
                            defaults.push(cur_frame.pop());
                        }
                        let callable = CallableObject::new(code, defaults);
                        cur_frame.push(callable);
                    },
                    PUSH_NULL => {
                        cur_frame.push(NullObject::new())   ;
                    },
                    RETURN_VALUE => {
                        self.return_value = Some(cur_frame.pop());
                        next_frame = cur_frame.parent();
                        break
                    },
                    LOAD_CONST => {
                        let obj = cur_frame.get_const(arg.unwrap() as usize);
                        cur_frame.push(obj);
                    },
                    LOAD_NAME => {
                        // LEGB
                        let name = cur_frame.get_name(arg.unwrap() as usize);
                        if let Some(obj) = cur_frame.look_up_name(name.clone()) {
                            cur_frame.push(obj);
                        } else if let Some(obj) = self.builtins.get(&name) {
                            cur_frame.push(obj.clone());
                        } else {
                            // TODO: enclosing missing
                            // cannot find name
                            panic!("cannot find variable {:?}", name);
                        }
                    },
                    LOAD_GLOBAL => {
                        // FIXME: every Callable object should have its own global
                        let arg = arg.unwrap();
                        if arg & 1 == 1 {
                            cur_frame.push(NullObject::new());
                        }
                        let arg = arg >> 1;
                        let name = cur_frame.get_name(arg as usize);
                        if let Some(obj) = cur_frame.look_up_global(name.clone()) {
                            cur_frame.push(obj);
                        } else if let Some(obj) = self.builtins.get(&name) {
                            cur_frame.push(obj.clone());
                        } else {
                            // TODO: enclosing missing
                            // cannot find name
                            panic!("cannot find variable {:?}", name);
                        }

                    },
                    LOAD_FAST => {
                        let arg = arg.unwrap();
                        cur_frame.push(cur_frame.load_fast(arg));
                    },
                    POP_TOP => {
                        cur_frame.pop();
                    },
                    STORE_NAME => {
                        let name = cur_frame.get_name(arg.unwrap() as usize);
                        let tos = cur_frame.pop();
                        cur_frame.set_local(name, tos);
                    },
                    RESUME | PRECALL | CACHE | KW_NAMES => {
                        // nop
                    }
                    _ => {unimplemented!()}
                }
            }
            if next_frame.is_some() {
                let mut next_frame = next_frame.take().unwrap();
                next_frame.set_parent(Some(cur_frame));
                if self.return_value.is_some() {
                    next_frame.push(self.return_value.clone().unwrap());
                }
                self.cur_frame = Some(next_frame);
            }
        }
    }



    pub fn return_value(&self) -> Option<PyObject> {
        self.return_value.clone()
    }
}