use crate::object::{BasePycObject, StringObject, TupleObject};
use crate::object::PyObjectTrait as PyObjectTrait;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::{ByteCode, DowncastTrait};
use crate::utils::Magic::{self, *};

use crate::utils::PyObject;
type PyObjectOption = Option<PyObject>;
#[derive(Default)]
#[allow(unused)]
pub struct CodeObject {
    base: BasePycObject,
    num_args: Option<u32>,
    num_pos_only_args: Option<u32>,
    num_kw_only_args: Option<u32>,
    num_locals: Option<u32>,
    num_stack: Option<u32>,
    flags: Option<u32>,
    code: Option<Vec<u8>>,
    constants: PyObjectOption,
    names: PyObjectOption,
    local_names: PyObjectOption,
    local_kinds: PyObjectOption,
    free_vars: PyObjectOption,
    cell_vars: PyObjectOption,
    file_name: PyObjectOption,
    name: PyObjectOption,
    qualified_name: PyObjectOption,
    first_line: Option<u32>,
    line_table: PyObjectOption,
    exception_table: PyObjectOption,

}

impl CodeObject {
    #[allow(unused_assignments)]
    pub fn new(stream: &mut InputStream, magic: Magic) -> PyObject {
        let mut num_args = None;
        let mut num_pos_only_args = None;
        let mut num_kw_only_args = None;
        let mut num_locals = None;
        let mut num_stack = None;
        let mut flags = None;
        let mut code = None;
        let mut constants = None;
        let mut names = None;
        let mut local_names = None;
        let mut local_kinds = None;
        let mut free_vars = None;
        let mut cell_vars = None;
        let mut file_name = None;
        let mut name = None;
        let mut first_line = None;
        let mut line_table = None;
        let mut exception_table = None;
        let mut qualified_name = None;
        if magic >= MAGIC1_3 && magic < MAGIC2_3 {
            num_args = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 {
            num_args = Some(stream.read_u32().unwrap());
        }
        if magic >= MAGIC3_8 {
            num_pos_only_args = Some(stream.read_u32().unwrap());
        }
        if magic >= MAGIC3_0 {
            num_kw_only_args = Some(stream.read_u32().unwrap());
        }
        if magic >= MAGIC1_3 && magic < MAGIC2_3 {
            num_locals = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 && magic < MAGIC3_11 {
            num_locals = Some(stream.read_u32().unwrap());
        }
        if magic >= MAGIC1_5 && magic < MAGIC2_3 {
            num_stack = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 {
            num_stack = Some(stream.read_u32().unwrap());
        }
        if magic >= MAGIC1_5 && magic < MAGIC2_3 {
            flags = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 {
            flags = Some(stream.read_u32().unwrap());
        }
        code = Some(PycParser::marshal_object(stream, magic).downcast_refcell::<StringObject>().unwrap().data().clone());
        constants = Some(PycParser::marshal_object(stream, magic));
        names = Some(PycParser::marshal_object(stream, magic));

        if magic >= MAGIC1_3  {
            local_names = Some(PycParser::marshal_object(stream, magic));
        }
        if magic >= MAGIC3_11  {
            local_kinds = Some(PycParser::marshal_object(stream, magic));
        }

        if magic >= MAGIC2_1 && magic < MAGIC3_11 {
            free_vars = Some(PycParser::marshal_object(stream, magic));
            cell_vars = Some(PycParser::marshal_object(stream, magic));
        }

        file_name = Some(PycParser::marshal_object(stream, magic));
        name = Some(PycParser::marshal_object(stream, magic));

        if magic >= MAGIC3_11 {
            qualified_name = Some(PycParser::marshal_object(stream, magic));
        }

        if magic >= MAGIC1_5 && magic < MAGIC2_3 {
            first_line = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 {
            first_line = Some(stream.read_u32().unwrap());
        }
        if magic >= MAGIC1_5 {
            line_table = Some(PycParser::marshal_object(stream, magic));
        }

        if magic >= MAGIC3_11 {
            exception_table = Some(PycParser::marshal_object(stream, magic));
        }

        let code = Self {
            base: BasePycObject::new_from_char('c'),
            num_args,
            num_pos_only_args,
            num_kw_only_args,
            num_locals,
            num_stack,
            flags,
            code,
            constants,
            names,
            local_names,
            local_kinds,
            free_vars,
            cell_vars,
            file_name,
            name,
            qualified_name,
            first_line,
            line_table,
            exception_table
        };
        // code.strip_cache();
        BasePycObject::new_py_object(code)
    }

    pub fn dump_code(&self) -> String {
        let mut res = "".to_string();
        let mut cursor = InputStream::new(self.code());
        let mut pc = 0u32;
        while !cursor.finish() {
            let mut inc = 0u32;
            let mut cur_line = "".to_string();
            let op_code = cursor.read().unwrap();
            inc += 1;

            let bytecode: ByteCode = op_code.into();
            if bytecode == ByteCode::CACHE {
                // FIXME: why we have extra CACHE?
                pc += inc;
                continue;
            }
            cur_line.push_str(&*format!("{:?}", bytecode));
            if bytecode.have_arg() {
                let arg = cursor.read().unwrap();
                inc += 1;
                cur_line.push_str(&*format!("  arg={}", arg));
            }
            if bytecode.cache_num() > 0 {
                cur_line.push_str(&*format!("  cache_num={}", bytecode.cache_num()));
                for _ in 0..bytecode.cache_num() {
                    cursor.read_u16().unwrap();
                    inc += 2;
                }
            }
            cur_line.push('\n');
            cur_line = format!("{pc}: ") + &cur_line;
            res.push_str(&cur_line);
            pc += inc;
        }
        return res;
    }

    pub fn consts(&self) -> Vec<PyObject> {
        // constants should be tuple
        let constants = self.constants.clone().unwrap();
        let tuple = constants.downcast_refcell::<TupleObject>().unwrap();
        tuple.values().clone()
    }
    pub fn names(&self) -> Vec<PyObject> {
        // constants should be tuple
        let names = self.names.clone().unwrap();
        let tuple = names.downcast_refcell::<TupleObject>().unwrap();
        tuple.values().clone()
    }
    pub fn code(&self) -> Vec<u8> {
        self.code.clone().unwrap().clone()
    }
    pub fn num_stack(&self) -> u32 {
        self.num_stack.unwrap()
    }

    #[allow(dead_code)]
    pub fn strip_cache(&mut self) {
        let mut new_code: Vec<u8> = vec![];
        let mut cursor = InputStream::new(self.code());
        while !cursor.finish() {
            let bytecode: ByteCode = cursor.read().unwrap().into();
            if bytecode != ByteCode::CACHE {
                new_code.push(bytecode.into());
            }
            if bytecode.have_arg() {
                new_code.push(cursor.read().unwrap());
            }
        }
        self.code = Some(new_code);
    }

    pub fn num_args(&self) -> u32 {
        self.num_args.unwrap()
    }
}

impl PartialEq<Self> for CodeObject {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Eq for CodeObject{}

impl PyObjectTrait for CodeObject {
    fn object_type(&self) -> ObjectType {
        self.base.object_type()
    }
}

impl fmt::Debug for CodeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        writeln!(f, "[code]").unwrap();
        writeln!(f, "CodeObject(").unwrap();
        writeln!(f, "   num_args={:?}", self.num_args).unwrap();
        writeln!(f, "   num_locals={:?}", self.num_locals).unwrap();
        writeln!(f, "   num_stack={:?}", self.num_stack).unwrap();
        writeln!(f, "   code=\n<<\n{}", self.dump_code()).unwrap();
        writeln!(f, ">>").unwrap();
        writeln!(f, "   names={:?}", self.names).unwrap();
        writeln!(f, "   local_names={:?}", self.local_names).unwrap();
        writeln!(f, "   file_name={:?}", self.file_name).unwrap();
        writeln!(f, "   name={:?}", self.name).unwrap();
        writeln!(f, ")")
    }
}
impl fmt::Display for CodeObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "")
    }
}

