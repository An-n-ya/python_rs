mod object;
mod utils;
mod interpreter;
mod frame;

use core::fmt;
use std::{env::args, fs::File, io::{Cursor, Read, Seek, self}};
use std::rc::Rc;

use chrono::NaiveDateTime;
use object::PyObject;
use object::IntObject;
use crate::interpreter::Interpreter;
use crate::object::{CodeObject, DictObject, FalseObject, IntLongObject, ListObject, NoneObject, NullObject, ObjectType, SetObject, StringObject, TrueObject, TupleObject, UnicodeObject};
use crate::utils::Magic;


pub(crate) struct InputStream {
    cursor: Cursor<Vec<u8>>
}

impl InputStream {
    pub fn new(contents: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(contents)
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
    pub fn read_int(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.cursor.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    pub fn read_long(&mut self) -> io::Result<u64> {
        let mut buf = [0; 8];
        self.cursor.read(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    pub fn unread(&mut self, n: usize) {
        self.cursor.seek(io::SeekFrom::Current(-(n as i64))).unwrap();
    }
    pub fn finish(&self) -> bool {
        if self.cursor.position() as usize == self.cursor.get_ref().len() {
            return true;
        }
        return false;
    }
}

struct PycParser {
    stream: InputStream,
    header: PycHeader,
    code_object: Rc<CodeObject>
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

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("please provide a file");
        std::process::exit(0);
    }
    if !std::path::Path::new(&args[1]).exists() {
        println!("cannot find file {}", args[1]);
        std::process::exit(0);
    }

    let file = File::open(&args[1]).expect("Failed to open file");
    let stream = InputStream::new_from_file(file);
    let parser = PycParser::new(stream);
    let mut interpreter = Interpreter::new(parser.code_object.clone());
    interpreter.run();
    parser.print_info();
}

impl PycParser {
    pub fn new(mut stream: InputStream) -> Self {
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
        let code_object = Self::marshal_object(&mut stream, magic).downcast_rc::<CodeObject>().unwrap();
        Self {stream, header, code_object}
    }

    pub fn print_info(&self) {
        println!("{}", self.header);
        println!("{:?}", self.code_object);
    }

    fn get_magic(stream: &mut InputStream) -> Magic {
        let magic = stream.read_int().unwrap();
        magic.into()
        // FIXME: different magic have different header
    }

    fn get_flags(stream: &mut InputStream) -> u32 {
        stream.read_int().unwrap()
    }
    fn get_size(stream: &mut InputStream) -> u32 {
        stream.read_int().unwrap()
    }
    fn get_timestamp(stream: &mut InputStream) -> NaiveDateTime {
        let timestamp = stream.read_int().unwrap();
        NaiveDateTime::from_timestamp_opt(timestamp.into(), 0).unwrap()
    }

    pub fn marshal_object(stream: &mut InputStream, magic: Magic) -> Rc<dyn PyObject> {
        let object_type: ObjectType = (stream.read().unwrap() as char).into();
        match object_type {
            ObjectType::NULL => NullObject::new(),
            ObjectType::NONE => NoneObject::new(),
            ObjectType::FALSE => FalseObject::new(),
            ObjectType::TRUE => TrueObject::new(),
            ObjectType::INT => IntObject::new(stream),
            ObjectType::INT64 => IntLongObject::new(stream),
            ObjectType::STRING
             | ObjectType::ASCII
             | ObjectType::ASCII_INTERNED => StringObject::new(stream),
            ObjectType::SHORT_ASCII
             | ObjectType::SHORT_ASCII_INTERNED => StringObject::new_from_short(stream),
            ObjectType::UNICODE => StringObject::new_from_unicode(stream),
            ObjectType::DICT => DictObject::new(stream, magic),
            ObjectType::LIST => ListObject::new(stream, magic),
            ObjectType::TUPLE => TupleObject::new(stream, magic),
            ObjectType::SMALL_TUPLE => TupleObject::new_from_short(stream, magic),
            ObjectType::SET => SetObject::new(stream, magic),
            ObjectType::REF => {
                //TODO: ref unimplemented
                stream.read_int().unwrap(); // index
                NullObject::new()
            },
            ObjectType::CODE => CodeObject::new(stream, magic),
            _ => unimplemented!()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_stream() {
        let arr: Vec<u8> = vec![1,2,3,4,5];
        let mut stream = InputStream::new(arr);
        assert_eq!(stream.read_int().unwrap(), 0x04030201);
        assert_eq!(stream.read().unwrap(), 5);
        stream.unread(1);
        assert_eq!(stream.read().unwrap(), 5);
    }

    #[test]
    fn test_one_plus_one() {
        let file = File::open("./tests/__pycache__/one_plus_one.cpython-311.pyc").expect("Failed to open file");
        let stream = InputStream::new_from_file(file);
        let parser = PycParser::new(stream);
        let mut interpreter = Interpreter::new(parser.code_object);
        interpreter.run();
        let return_value = interpreter.return_value();
        assert!(return_value.is_some());
        let return_value = return_value.unwrap();
        let return_value = return_value.downcast_rc::<NoneObject>().expect("return value should be NoneObject");
        assert_eq!(return_value, Rc::new(NoneObject::new()));
    }
}

