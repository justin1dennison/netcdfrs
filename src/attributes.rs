use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use helpers::*;

pub struct Attribute;

impl Attribute {
    pub fn from_file(fp: &mut File) -> HashMap<String, String> {
        let attr_header = unpack_int(fp);
        match attr_header {
            0 | 12 => println!("Success reading attrs header: {:#?}", attr_header),
            _ => panic!("improper attribute header"),
        }
        let attr_count = unpack_int(fp);
        let mut attributes = HashMap::<String, String>::new();
        for _ in 0..attr_count {
            let name_len = unpack_int(fp);
            let name = unpack_string(fp, name_len as usize);
            let forward_amt = modulo(-1i32 * name_len as i32, 4);
            unpack_string(fp, forward_amt as usize);
            let nc_type = unpack_int(fp);
            let n = unpack_int(fp);
            let typemap: HashMap<u8, (&str, u8)> =
                [(1, ("b", 1)), (2, ("c", 1))].iter().cloned().collect();
            let (_typecode, size) = typemap[&(nc_type as u8)];
            let count = (n as i32) * (size as i32);
            let values = unpack_string(fp, count as usize);
            let forward_amt = modulo(-1i32 * count as i32, 4);
            unpack_string(fp, forward_amt as usize);
            attributes.insert(name, values);
        }
        attributes
    }
}
