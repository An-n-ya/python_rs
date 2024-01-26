mod object;
mod utils;

use core::fmt;
use std::{env::args, fs::File, io::{Cursor, Read, Seek, self}};

use chrono::NaiveDateTime;
use object::PycObject;
use object::IntObject;
use crate::object::{CodeObject, DictObject, FalseObject, IntLongObject, ListObject, NoneObject, NullObject, ObjectType, SetObject, SmallTupleObject, StringObject, TrueObject, TupleObject, UnicodeObject};
use crate::utils::Magic;


pub(crate) struct InputStream {
    cursor: Cursor<Vec<u8>>
}

impl InputStream {
    fn new(contents: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(contents)
        }
    }
    fn new_from_file(mut file: File) -> Self {
        let mut contents = vec![];
        file.read_to_end(&mut contents).unwrap();
        Self::new(contents)
    }
    fn read(&mut self) -> io::Result<u8> {
        let mut buf = [0;1];
        self.cursor.read(&mut buf)?;
        Ok(buf[0])
    }
    fn read_int(&mut self) -> io::Result<u32> {
        let mut buf = [0; 4];
        self.cursor.read(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    fn read_long(&mut self) -> io::Result<u64> {
        let mut buf = [0; 8];
        self.cursor.read(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    fn unread(&mut self, n: usize) {
        self.cursor.seek(io::SeekFrom::Current(-(n as i64))).unwrap();
    }
}

struct PycParser {
    stream: InputStream,
    header: PycHeader,
    code_object: CodeObject
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
        let code_object = *Self::marshal_object(&mut stream).downcast::<CodeObject>().unwrap();
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

    pub fn marshal_object(stream: &mut InputStream) -> Box<dyn PycObject> {
        let object_type: ObjectType = (stream.read().unwrap() as char).into();
        match object_type {
            ObjectType::NULL => Box::new(NullObject::new()),
            ObjectType::NONE => Box::new(NoneObject::new()),
            ObjectType::FALSE => Box::new(FalseObject::new()),
            ObjectType::TRUE => Box::new(TrueObject::new()),
            ObjectType::INT => Box::new(IntObject::new(stream)),
            ObjectType::INT64 => Box::new(IntLongObject::new(stream)),
            ObjectType::STRING
             | ObjectType::ASCII
             | ObjectType::ASCII_INTERNED => Box::new(StringObject::new(stream)),
            ObjectType::SHORT_ASCII
             | ObjectType::SHORT_ASCII_INTERNED => Box::new(StringObject::new_from_short(stream)),
            ObjectType::UNICODE => Box::new(UnicodeObject::new(stream)),
            ObjectType::DICT => Box::new(DictObject::new(stream)),
            ObjectType::LIST => Box::new(ListObject::new(stream)),
            ObjectType::TUPLE => Box::new(TupleObject::new(stream)),
            ObjectType::SMALL_TUPLE => Box::new(SmallTupleObject::new(stream)),
            ObjectType::SET => Box::new(SetObject::new(stream)),
            ObjectType::REF => {
                //TODO: ref unimplemented
                stream.read_int().unwrap(); // index
                Box::new(NullObject::new())
            },
            ObjectType::CODE => Box::new(CodeObject::new(stream)),
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
}

