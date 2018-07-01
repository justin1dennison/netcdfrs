use byteorder::{BigEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;

pub fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

pub fn unpack_int(fp: &mut File) -> i32 {
    match fp.read_i32::<BigEndian>() {
        Ok(val) => val,
        Err(msg) => {
            panic!("Encountered Error: {}", msg);
        }
    }
}

pub fn unpack_string(fp: &mut File, length: usize) -> String {
    let mut buf = vec![0u8; length];
    fp.read(&mut buf).unwrap();
    buf.iter().map(|s| *s).map(char::from).collect::<String>()
}