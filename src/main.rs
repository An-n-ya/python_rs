use core::{fmt, time};
use std::{env::args, fs::File, io::{Cursor, Read, Seek, self}};

use chrono::NaiveDateTime;

enum Magic {
    MAGIC1_0 = 0x00999902,
    MAGIC1_1 = 0x00999903, /* Also covers 1.2 */
    MAGIC1_3 = 0x0A0D2E89,
    MAGIC1_4 = 0x0A0D1704,
    MAGIC1_5 = 0x0A0D4E99,
    MAGIC1_6 = 0x0A0DC4FC,

    MAGIC2_0 = 0x0A0DC687,
    MAGIC2_1 = 0x0A0DEB2A,
    MAGIC2_2 = 0x0A0DED2D,
    MAGIC2_3 = 0x0A0DF23B,
    MAGIC2_4 = 0x0A0DF26D,
    MAGIC2_5 = 0x0A0DF2B3,
    MAGIC2_6 = 0x0A0DF2D1,
    MAGIC2_7 = 0x0A0DF303,

    MAGIC3_0 = 0x0A0D0C3A,
    MAGIC3_1 = 0x0A0D0C4E,
    MAGIC3_2 = 0x0A0D0C6C,
    MAGIC3_3 = 0x0A0D0C9E,
    MAGIC3_4 = 0x0A0D0CEE,
    MAGIC3_5 = 0x0A0D0D16,
    MAGIC3_5_3 = 0x0A0D0D17,
    MAGIC3_6 = 0x0A0D0D33,
    MAGIC3_7 = 0x0A0D0D42,
    MAGIC3_8 = 0x0A0D0D55,
    MAGIC3_9 = 0x0A0D0D61,
    MAGIC3_10 = 0x0A0D0D6F,
    MAGIC3_11 = 0x0A0D0DA7,
}

impl fmt::Display for Magic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Magic::MAGIC1_0 => write!(f, "1.0"),
            Magic::MAGIC1_1 => write!(f, "1.1 or 1.2"),
            Magic::MAGIC1_3 => write!(f, "1.3"),
            Magic::MAGIC1_4 => write!(f, "1.4"),
            Magic::MAGIC1_5 => write!(f, "1.5"),
            Magic::MAGIC1_6 => write!(f, "1.6"),
            Magic::MAGIC2_0 => write!(f, "2.0"),
            Magic::MAGIC2_1 => write!(f, "2.1"),
            Magic::MAGIC2_2 => write!(f, "2.2"),
            Magic::MAGIC2_3 => write!(f, "2.3"),
            Magic::MAGIC2_4 => write!(f, "2.4"),
            Magic::MAGIC2_5 => write!(f, "2.5"),
            Magic::MAGIC2_6 => write!(f, "2.6"),
            Magic::MAGIC2_7 => write!(f, "2.7"),
            Magic::MAGIC3_0 => write!(f, "3.0"),
            Magic::MAGIC3_1 => write!(f, "3.1"),
            Magic::MAGIC3_2 => write!(f, "3.2"),
            Magic::MAGIC3_3 => write!(f, "3.3"),
            Magic::MAGIC3_4 => write!(f, "3.4"),
            Magic::MAGIC3_5 => write!(f, "3.5"),
            Magic::MAGIC3_5_3 => write!(f, ".3.5.3"),
            Magic::MAGIC3_6 => write!(f, "3.6"),
            Magic::MAGIC3_7 => write!(f, "3.7"),
            Magic::MAGIC3_8 => write!(f, "3.8"),
            Magic::MAGIC3_9 => write!(f, "3.9"),
            Magic::MAGIC3_10 => write!(f, "3.10"),
            Magic::MAGIC3_11 => write!(f, "3.11"),
        }
    }
}

impl From<u32> for Magic {
    fn from(value: u32) -> Self {
        match value {
            0x00999902 => Magic::MAGIC1_0,
            0x00999903 => Magic::MAGIC1_1,
            0x0A0D2E89 => Magic::MAGIC1_3,
            0x0A0D1704 => Magic::MAGIC1_4,
            0x0A0D4E99 => Magic::MAGIC1_5,
            0x0A0DC4FC => Magic::MAGIC1_6,

            0x0A0DC687 => Magic::MAGIC2_0,
            0x0A0DEB2A => Magic::MAGIC2_1,
            0x0A0DED2D => Magic::MAGIC2_2,
            0x0A0DF23B => Magic::MAGIC2_3,
            0x0A0DF26D => Magic::MAGIC2_4,
            0x0A0DF2B3 => Magic::MAGIC2_5,
            0x0A0DF2D1 => Magic::MAGIC2_6,
            0x0A0DF303 => Magic::MAGIC2_7,

            0x0A0D0C3A => Magic::MAGIC3_0,
            0x0A0D0C4E => Magic::MAGIC3_1,
            0x0A0D0C6C => Magic::MAGIC3_2,
            0x0A0D0C9E => Magic::MAGIC3_3,
            0x0A0D0CEE => Magic::MAGIC3_4,
            0x0A0D0D16 => Magic::MAGIC3_5,
            0x0A0D0D17 => Magic::MAGIC3_5_3,
            0x0A0D0D33 => Magic::MAGIC3_6,
            0x0A0D0D42 => Magic::MAGIC3_7,
            0x0A0D0D55 => Magic::MAGIC3_8,
            0x0A0D0D61 => Magic::MAGIC3_9,
            0x0A0D0D6F => Magic::MAGIC3_10,
            0x0A0D0DA7 => Magic::MAGIC3_11,
            _ => panic!("invalid magic number: {}", value)
        }
    }
}


struct InputStream {
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
    fn unread(&mut self, n: usize) {
        self.cursor.seek(io::SeekFrom::Current(-(n as i64))).unwrap();
    }
}

struct PycParser {
    stream: InputStream,
    header: PycHeader,
    code_object: PycObject
}

struct PycHeader {
    magic: Magic,
    flags: u32,
    timestamp: NaiveDateTime,
    size: u32
}

struct PycObject {

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
        let code_object = PycObject {};
        Self {stream, header, code_object}
    }

    pub fn print_info(&self) {
        println!("{}", self.header);
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

