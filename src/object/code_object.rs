use crate::object::{BasePycObject, SmallTupleObject, StringObject};
use crate::object::PycObject;
use crate::object::ObjectType;
use std::fmt;
use crate::{InputStream, PycParser};

pub struct CodeObject {
    base: BasePycObject,
    num_args: u32,
    num_pos_only_args: u32,
    num_kw_only_args: u32,
    num_locals: u32,
    num_stack: u32,
    flags: u32,
    code: Vec<u8>,
    constants: Vec<Box<dyn PycObject>>,
    names: Vec<String>,
    local_names: Vec<String>,
    free_vars: Box<dyn PycObject>,
    cell_vars: Box<dyn PycObject>,
    file_name: String,
    name: String,
    first_line: u32,
    line_table: Box<dyn PycObject>,
    exception_table: Box<dyn PycObject>,

}

impl CodeObject {
    pub fn new(stream: &mut InputStream) -> Self {
        let num_args = stream.read_int().unwrap();
        let num_pos_only_args = stream.read_int().unwrap();
        let num_kw_only_args = stream.read_int().unwrap();
        let num_locals = stream.read_int().unwrap();
        let num_stack = stream.read_int().unwrap();
        let flags = stream.read_int().unwrap();
        let code = PycParser::marshal_object(stream).downcast::<StringObject>().unwrap();
        let code = code.data().clone();
        let constants = PycParser::marshal_object(stream).downcast::<SmallTupleObject>().unwrap();
        let constants = constants.take_values();
        let names = PycParser::marshal_object(stream).downcast::<SmallTupleObject>().unwrap();
        let names: Vec<String> = names.take_values().iter().map(|item| -> String {
            item.downcast_ref::<StringObject>().unwrap().to_string()
        }).collect();
        let local_names = PycParser::marshal_object(stream).downcast::<SmallTupleObject>().unwrap();
        let local_names: Vec<String> = local_names.take_values().iter().map(|item| -> String {
            item.downcast_ref::<StringObject>().unwrap().to_string()
        }).collect();

        let free_vars = PycParser::marshal_object(stream); //TODO: ref
        let cell_vars = PycParser::marshal_object(stream); //TODO: ref

        let file_name = PycParser::marshal_object(stream).downcast::<StringObject>().unwrap();
        let file_name = file_name.to_string();
        let name = PycParser::marshal_object(stream).downcast::<StringObject>().unwrap();
        let name = name.to_string();
        let first_line = stream.read_int().unwrap();
        let line_table = PycParser::marshal_object(stream);

        let exception_table = PycParser::marshal_object(stream); //TODO: ref

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
            free_vars,
            cell_vars,
            file_name,
            name,
            first_line,
            line_table,
            exception_table
        }
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
        writeln!(f, "   num_args={}", self.num_args).unwrap();
        writeln!(f, "   num_locals={}", self.num_locals).unwrap();
        writeln!(f, "   num_stack={}", self.num_stack).unwrap();
        writeln!(f, "   code={:?}", self.code).unwrap();
        writeln!(f, "   names={:?}", self.names).unwrap();
        writeln!(f, "   local_names={:?}", self.local_names).unwrap();
        writeln!(f, "   file_name={}", self.file_name).unwrap();
        writeln!(f, "   name={}", self.name).unwrap();
        writeln!(f, ")")
    }
}