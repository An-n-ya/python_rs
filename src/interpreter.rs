use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::frame::Frame as FrameRaw;
use crate::object::{CallableObject, CodeObject, FalseObject, IntObject, NoneObject, NullObject, TrueObject};
use crate::utils::ByteCode::*;
use crate::utils::{BinaryOp, ByteCode, CmpOP, DowncastTrait, PyObject};


type Frame = Option<Box<FrameRaw>>;
pub struct Interpreter {
    cur_frame: Frame,
    return_value: Option<PyObject>,
    builtins: HashMap<String, PyObject>
}

fn native_print(args: Vec<PyObject>) -> PyObject {
    for arg in args {
        // FIXME: print problem in function.py
        print!("{}", arg.borrow());
        // print!("hello");
    }
    println!();
    NoneObject::new()
}

impl Interpreter {
    const CMP_OP: [CmpOP; 6] = [CmpOP::LT, CmpOP::LE, CmpOP::EQ, CmpOP::NEQ, CmpOP::GT, CmpOP::GE];
    pub fn new(code: Rc<RefCell<CodeObject>>) -> Self {
        let builtins = Self::setup_builtins();

        Self {
            cur_frame: Some(Box::new(FrameRaw::new_from_code(code))),
            return_value: None,
            builtins
        }
    }

    fn setup_builtins() -> HashMap<String, PyObject> {
        let mut builtins: HashMap<String, PyObject> = HashMap::new();
        builtins.insert("$String_True".to_string(), TrueObject::new());
        builtins.insert("$String_False".to_string(), FalseObject::new());
        builtins.insert("$String_None".to_string(), NoneObject::new());
        builtins.insert("$String_print".to_string(), CallableObject::new_native(Box::new(native_print)));

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
                // println!("interpret bytecode: {:?}", bytecode);

                match bytecode {
                    CALL => {
                        let arg = arg.unwrap();
                        let mut fn_arg: Vec<PyObject> = vec![];
                        for _ in 0..arg {
                            fn_arg.push(cur_frame.pop())
                        }
                        let obj = cur_frame.pop();
                        let obj2 = cur_frame.pop();
                        if obj2.downcast_refcell::<NullObject>().is_some() {
                            let callable = obj.downcast_refcell::<CallableObject>().expect("invalid callable object");
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
                        let code = cur_frame.pop();
                        let code = code.downcast_refcell::<CodeObject>().expect("invalid code object");
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
                        } else if let Some(obj) = self.builtins.get(&name.borrow().hash_key()) {
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
                        } else if let Some(obj) = self.builtins.get(&name.borrow().hash_key()) {
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
                    COMPARE_OP => {
                        let arg = arg.unwrap();
                        let rhs = cur_frame.pop();
                        let lhs = cur_frame.pop();
                        // we suppose that lhs and rhs should be number
                        let lhs = lhs
                            .downcast_refcell::<IntObject>()
                            .expect(&format!("{:?} cannot be compared", lhs.borrow().object_type()));
                        let rhs = rhs.downcast_refcell::<IntObject>()
                            .expect(&format!("{:?} cannot be compared", rhs.borrow().object_type()));
                        let op = &Self::CMP_OP[arg as usize];
                        match op {
                            CmpOP::GT =>cur_frame.push(Self::new_bool_object(lhs.value() > rhs.value())),
                            CmpOP::GE =>cur_frame.push(Self::new_bool_object(lhs.value() >= rhs.value())),
                            CmpOP::LT =>cur_frame.push(Self::new_bool_object(lhs.value() < rhs.value())),
                            CmpOP::LE =>cur_frame.push(Self::new_bool_object(lhs.value() <= rhs.value())),
                            CmpOP::EQ =>cur_frame.push(Self::new_bool_object(lhs.value() == rhs.value())),
                            CmpOP::NEQ =>cur_frame.push(Self::new_bool_object(lhs.value() != rhs.value())),
                        }
                    },
                    POP_JUMP_BACKWARD_IF_NOT_NONE | POP_JUMP_FORWARD_IF_NOT_NONE => {
                        let arg = arg.unwrap();
                        let tos = cur_frame.pop();
                        if tos.downcast_refcell::<NoneObject>().is_none() {
                            cur_frame.jump_offset(Self::get_jump_offset(bytecode, arg));
                        }
                    },
                    POP_JUMP_FORWARD_IF_NONE | POP_JUMP_BACKWARD_IF_NONE => {
                        let arg = arg.unwrap();
                        let tos = cur_frame.pop();
                        if tos.downcast_refcell::<NoneObject>().is_some() {
                            cur_frame.jump_offset(Self::get_jump_offset(bytecode, arg));
                        }
                    },
                    POP_JUMP_BACKWARD_IF_TRUE | POP_JUMP_FORWARD_IF_TRUE =>{
                        let arg = arg.unwrap() ;
                        let tos = cur_frame.pop();
                        if tos.downcast_refcell::<TrueObject>().is_some() {
                            cur_frame.jump_offset(Self::get_jump_offset(bytecode, arg));
                        }
                    },
                    POP_JUMP_BACKWARD_IF_FALSE | POP_JUMP_FORWARD_IF_FALSE =>{
                        let arg = arg.unwrap();
                        let tos = cur_frame.pop();
                        if tos.downcast_refcell::<FalseObject>().is_some() {
                            cur_frame.jump_offset(Self::get_jump_offset(bytecode, arg));
                        }
                    },
                    JUMP_BACKWARD | JUMP_FORWARD => {
                        let arg = arg.unwrap();
                        cur_frame.jump_offset(Self::get_jump_offset(bytecode, arg));
                    }
                    BINARY_OP => {
                        let op:BinaryOp = arg.unwrap().into();
                        let tos = cur_frame.pop();
                        let tos1 = cur_frame.pop();
                        cur_frame.push(op.handle(tos1, tos));
                    },
                    RESUME | PRECALL | CACHE | KW_NAMES => {
                        // nop
                    }
                    _ => {unimplemented!("command {:?} unimplemented", bytecode)}
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


    pub fn new_bool_object(val: bool) -> PyObject {
        if val {
            TrueObject::new()
        } else {
            FalseObject::new()
        }
    }

    const BACKWARD_JUMP: [ByteCode; 6] = [
        POP_JUMP_BACKWARD_IF_NONE,
        POP_JUMP_BACKWARD_IF_FALSE,
        POP_JUMP_BACKWARD_IF_TRUE,
        JUMP_BACKWARD,
        POP_JUMP_BACKWARD_IF_NOT_NONE,
        JUMP_BACKWARD_NO_INTERRUPT
    ];
    fn get_jump_offset(bytecode: ByteCode, arg: u8) -> i64 {
        let caches = bytecode.cache_num() as i64;
        let mut arg = arg as i64;

        if Self::BACKWARD_JUMP.contains(&bytecode) {
            arg = -arg;
        }
        // refer to Cpython(Lib/dis.py:_get_jump_target)
        2 * caches + arg * 2
    }

    #[allow(dead_code)]
    pub fn return_value(&self) -> Option<PyObject> {
        self.return_value.clone()
    }
}