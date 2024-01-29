use crate::object::{BasePycObject, SmallTupleObject, StringObject};
use crate::object::PycObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};
use crate::utils::ByteCode;
use crate::utils::Magic::{self, *};

type PyObject = Option<Box<dyn PycObject>>;
pub struct CodeObject {
    base: BasePycObject,
    num_args: Option<u32>,
    num_pos_only_args: Option<u32>,
    num_kw_only_args: Option<u32>,
    num_locals: Option<u32>,
    num_stack: Option<u32>,
    flags: Option<u32>,
    code: Option<Vec<u8>>,
    constants: PyObject,
    names: PyObject,
    local_names: PyObject,
    local_kinds: PyObject,
    free_vars: PyObject,
    cell_vars: PyObject,
    file_name: PyObject,
    name: PyObject,
    qualified_name: PyObject,
    first_line: Option<u32>,
    line_table: PyObject,
    exception_table: PyObject,

}

impl CodeObject {
    pub fn new(stream: &mut InputStream, magic: Magic) -> Self {
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
            num_args = Some(stream.read_int().unwrap());
        }
        if magic >= MAGIC3_8 {
            num_pos_only_args = Some(stream.read_int().unwrap());
        }
        if magic >= MAGIC3_0 {
            num_kw_only_args = Some(stream.read_int().unwrap());
        }
        if magic >= MAGIC1_3 && magic < MAGIC2_3 {
            num_locals = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 && magic < MAGIC3_11 {
            num_locals = Some(stream.read_int().unwrap());
        }
        if magic >= MAGIC1_5 && magic < MAGIC2_3 {
            num_stack = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 {
            num_stack = Some(stream.read_int().unwrap());
        }
        if magic >= MAGIC1_5 && magic < MAGIC2_3 {
            flags = Some(stream.read_u16().unwrap() as u32);
        } else if magic >= MAGIC2_3 {
            flags = Some(stream.read_int().unwrap());
        }
        code = Some(PycParser::marshal_object(stream, magic).downcast::<StringObject>().unwrap().data().clone());
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
            first_line = Some(stream.read_int().unwrap());
        }
        if magic >= MAGIC1_5 {
            line_table = Some(PycParser::marshal_object(stream, magic));
        }

        if magic >= MAGIC3_11 {
            exception_table = Some(PycParser::marshal_object(stream, magic));
        }

        Self {
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
        }
    }

    pub fn dump_code(&self) -> String {
        let mut res = "".to_string();
        let mut cursor = InputStream::new(self.code());
        while !cursor.finish() {
            let op_code = cursor.read().unwrap();
            let bytecode: ByteCode = op_code.into();
            if bytecode == ByteCode::CACHE {
                // FIXME: why we have extra CACHE?
                continue;
            }
            res.push_str(&*format!("{:?}", bytecode));
            if bytecode.have_arg() {
                let arg = cursor.read().unwrap();
                res.push_str(&*format!("  arg={}", arg));
            }
            if bytecode.cache_num() > 0 {
                res.push_str(&*format!("  cache_num={}", bytecode.cache_num()));
                for _ in 0..bytecode.cache_num() {
                    // FIXME: why cache is 16 bit wide?
                    cursor.read_u16().unwrap();
                }
            }
            res.push('\n');
        }
        return res;
    }

    pub fn code(&self) -> Vec<u8> {
        self.code.clone().unwrap().clone()
    }
}

impl PycObject for CodeObject {
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