mod object;
mod utils;
mod interpreter;
mod frame;

use core::fmt;
use std::{fs::File, io::{Cursor, Read, Seek, self}, mem};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::process::{Command};
use std::rc::Rc;

use chrono::NaiveDateTime;
use clap::Parser;
use object::IntObject;
use crate::interpreter::Interpreter;
use crate::object::{CodeObject, DictObject, FalseObject, IntLongObject, ListObject, NoneObject, NullObject, ObjectType, SetObject, StringObject, TrueObject, TupleObject};
use crate::utils::{DowncastTrait, Magic, PyObject};


pub(crate) struct InputStream {
    cursor: Cursor<Vec<u8>>,
    refs: Vec<PyObject>,
    depths: u8
}

impl InputStream {
    pub fn new(contents: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(contents),
            depths: 0,
            refs: Vec::default()
        }
    }
    pub fn new_from_file(mut file: File) -> Self {
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        Self::new(contents)
    }
    pub fn read(&mut self) -> io::Result<u8> {
        let mut buf = [0;1];
        let n = self.cursor.read(&mut buf)?;
        if n == 0 {
        }
        Ok(buf[0])
    }
    pub fn read_u16(&mut self) -> io::Result<u16> {
        let mut buf = [0; 2];
        self.cursor.read(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
    pub fn read_i32(&mut self) -> io::Result<i32> {
        let mut buf = [0; 4];
        self.cursor.read(&mut buf)?;
        Ok(i32::from_le_bytes(buf))
    }
    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.cursor.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    pub fn read_i64(&mut self) -> io::Result<i64> {
        let mut buf = [0; 8];
        self.cursor.read(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
    #[allow(dead_code)]
    pub fn read_long(&mut self) -> io::Result<u64> {
        let mut buf = [0; 8];
        self.cursor.read(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    #[allow(dead_code)]
    pub fn unread(&mut self, n: usize) {
        self.cursor.seek(io::SeekFrom::Current(-(n as i64))).unwrap();
    }
    pub fn jump_offset(&mut self, n: i64) {
        self.cursor.seek(io::SeekFrom::Current(n)).unwrap();
    }
    #[allow(dead_code)]
    pub fn forward(&mut self, n: usize) {
        self.cursor.seek(io::SeekFrom::Current(n as i64)).unwrap();
    }
    pub fn finish(&self) -> bool {
        if self.cursor.position() as usize == self.cursor.get_ref().len() {
            return true;
        }
        return false;
    }

    pub fn inc_depth(&mut self){
        self.depths += 1;
    }
    pub fn dec_depth(&mut self){
        self.depths -= 1;
    }
    #[allow(dead_code)]
    pub fn depth(&self) -> u8 {
        self.depths
    }

    pub fn push_ref(&mut self, r: PyObject) {
        #[cfg(feature = "debug_marshal")]
        println!("[DEBUG] push ref {}:{r:?}", self.refs.len());
        self.refs.push(r);
    }

    pub fn reserve_ref(&mut self) ->usize {
        let res = self.refs.len();
        self.refs.push(NullObject::new());
        res
    }

    pub fn set_ref(&mut self, index: usize, object: PyObject) {
        assert!(index < self.refs.len());
        self.refs[index] = object;
    }

    pub fn get_ref(&self, index: usize) -> PyObject {
        self.refs.get(index).unwrap().clone()
    }
}

struct PycParser {
    header: PycHeader,
    code_object: Rc<RefCell<CodeObject>>
}

struct PycHeader {
    magic: Magic,
    flags: u32,
    timestamp: NaiveDateTime,
    size: u32
}


impl fmt::Display for PycHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[header]\nversion: {}\nflags: {}\ntimestamp: {}\nsize: {}",
                self.magic,
                self.flags,
                self.timestamp,
                self.size
            )
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, short, action)]
    info: bool,
    #[arg(long, short, action)]
    no_run: bool,
    file: String,
}

fn main() {
    let args = Args::parse();
    if !std::path::Path::new(&args.file).exists() {
        println!("cannot find file {}", args.file);
        std::process::exit(0);
    }

    Command::new("/home/annya/miniconda3/bin/compileall2")
        .arg(&args.file)
        .output()
        .expect(&format!("failed to compile {}", &args.file));

    const PYC_SUFFIX: &str = ".cpython-311.pyc";
    let file_path = std::path::Path::new(&args.file);
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let len = file_name.len();
    let file_name = &file_name[0..len - 3];
    let parent_path = file_path.parent().unwrap();
    let pyc_path = parent_path.to_str().unwrap().to_string() + "/__pycache__/" + file_name + PYC_SUFFIX;



    let file = File::open(pyc_path).expect("Failed to open file");
    let parser = PycParser::new(file);
    let mut interpreter = Interpreter::new(parser.code_object.clone());
    if !args.no_run {
        interpreter.run();
    }
    if args.info {
        parser.print_info();
    }
}

impl PycParser {
    pub fn new(file: File) -> Self {
        let mut stream = InputStream::new_from_file(file);
        let magic = Self::get_magic(&mut stream);
        let flags = Self::get_flags(&mut stream);
        let timestamp = Self::get_timestamp(&mut stream);
        let size = Self::get_size(&mut stream);
        let header = PycHeader {
            magic,
            flags,
            timestamp,
            size
        };
        let code_object = Self::marshal_object(&mut stream, magic);
        let mut code_object = code_object.downcast_refcell_mut::<CodeObject>().unwrap();
        let code_object = code_object.deref_mut();
        let mut new_code_obj = CodeObject::default();
        mem::swap(&mut new_code_obj, code_object);
        Self {header, code_object: Rc::new(RefCell::new(new_code_obj))}
    }

    pub fn print_info(&self) {
        println!("{}", self.header);
        println!("{:?}", self.code_object);
    }

    fn get_magic(stream: &mut InputStream) -> Magic {
        let magic = stream.read_u32().unwrap();
        magic.into()
        // FIXME: different magic have different header
    }

    fn get_flags(stream: &mut InputStream) -> u32 {
        stream.read_u32().unwrap()
    }
    fn get_size(stream: &mut InputStream) -> u32 {
        stream.read_u32().unwrap()
    }
    fn get_timestamp(stream: &mut InputStream) -> NaiveDateTime {
        let timestamp = stream.read_u32().unwrap();
        NaiveDateTime::from_timestamp_opt(timestamp.into(), 0).unwrap()
    }

    pub fn marshal_object(stream: &mut InputStream, magic: Magic) -> PyObject {
        let object_type: ObjectType = (stream.read().unwrap() as char).into();
        stream.inc_depth();

        let ret: PyObject = match object_type {
            ObjectType::NULL => NullObject::new(),
            ObjectType::NONE => NoneObject::new(),
            ObjectType::FALSE => FalseObject::new(),
            ObjectType::TRUE => TrueObject::new(),
            ObjectType::INT => {
                let ret = IntObject::new(stream);
                stream.push_ref(ret.clone());
                ret
            },
            ObjectType::INT64 => {
                let ret = IntLongObject::new(stream);
                stream.push_ref(ret.clone());
                ret
            },
            ObjectType::STRING
             | ObjectType::ASCII
             | ObjectType::ASCII_INTERNED => {
                let ret = StringObject::new(stream);
                // FIXME: why we cannot add ref to STRING, but SHORT_ASCII is ok?
                // stream.push_ref(ret.clone());
                ret
            },
            ObjectType::SHORT_ASCII
             | ObjectType::SHORT_ASCII_INTERNED => {
                let ret = StringObject::new_from_short(stream);
                stream.push_ref(ret.clone());
                ret
            },
            ObjectType::UNICODE => {
                let ret = StringObject::new_from_unicode(stream);
                stream.push_ref(ret.clone());
                ret
            },
            ObjectType::DICT => {
                let ret = DictObject::new(stream, magic);
                stream.push_ref(ret.clone());
                ret
            },
            ObjectType::LIST => {
                let ret = ListObject::new(stream, magic);
                stream.push_ref(ret.clone());
                ret
            },
            ObjectType::TUPLE => {
                let index = stream.reserve_ref();
                #[cfg(feature = "debug_marshal")]
                println!("[DEBUG] reserved index: {index}");
                let ret = TupleObject::new(stream, magic);
                stream.set_ref(index, ret.clone());
                ret
            },
            ObjectType::SMALL_TUPLE => {
                let index = stream.reserve_ref();
                #[cfg(feature = "debug_marshal")]
                println!("[DEBUG] reserved index: {index}");
                let ret = TupleObject::new_from_short(stream, magic);
                stream.set_ref(index, ret.clone());
                ret
            },
            ObjectType::SET => {
                let ret = SetObject::new(stream, magic);
                stream.push_ref(ret.clone());
                ret
            },
            ObjectType::REF => {
                let index = stream.read_u32().unwrap(); // index
                #[cfg(feature = "debug_marshal")]
                println!("[DEBUG] index: {index}");
                stream.get_ref(index as usize)
            },
            ObjectType::CODE => CodeObject::new(stream, magic),
            _ => unimplemented!()
        };
        stream.dec_depth();
        ret
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_stream() {
        let arr: Vec<u8> = vec![1,2,3,4,5];
        let mut stream = InputStream::new(arr);
        assert_eq!(stream.read_u32().unwrap(), 0x04030201);
        assert_eq!(stream.read().unwrap(), 5);
        stream.unread(1);
        assert_eq!(stream.read().unwrap(), 5);
    }

    #[test]
    fn test_one_plus_one() {
        let file = File::open("./tests/__pycache__/one_plus_one.cpython-311.pyc").expect("Failed to open file");
        let parser = PycParser::new(file);
        let mut interpreter = Interpreter::new(parser.code_object);
        interpreter.run();
        let return_value = interpreter.return_value();
        assert!(return_value.is_some());
        let return_value = return_value.unwrap();
        let return_value = return_value.downcast_refcell::<NoneObject>().expect("return value should be NoneObject");
        assert_eq!(*return_value, NoneObject::new_raw());
    }

    // #[test]
    // fn test_function() {
    //     let file = File::open("./tests/__pycache__/function.cpython-311.pyc").expect("Failed to open file");
    //     let parser = PycParser::new(file);
    //     let mut interpreter = Interpreter::new(parser.code_object);
    //     interpreter.run();
    //     let return_value = interpreter.return_value();
    //     assert!(return_value.is_some());
    //     let return_value = return_value.unwrap();
    //     let return_value = return_value.downcast_rc::<NoneObject>().expect("return value should be NoneObject");
    //     assert_eq!(return_value, NoneObject::new());
    // }
}

