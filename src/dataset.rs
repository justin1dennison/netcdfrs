use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use dimension::Dimension;
use dtype::Dtype;
use variable::Variable;

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dataset {
    pub filename: Option<String>,
    pub variables: HashMap<String, Variable>,
    pub dimensions: HashMap<String, Dimension>,
    pub attributes: HashMap<String, String>,
    pub version: i32,
    pub numrecs: i32,
}

fn unpack_int(fp: &mut File) -> i32 {
    match fp.read_i32::<BigEndian>() {
        Ok(val) => val,
        Err(msg) => {
            panic!("Encountered Error: {}", msg);
        }
    }
}

fn unpack_string(fp: &mut File, length: usize) -> String {
    let mut buf = vec![0u8; length];
    fp.read(&mut buf).unwrap();
    buf.iter().map(|s| *s).map(char::from).collect::<String>()
}

impl Dataset {
    pub fn new() -> Dataset {
        Dataset {
            filename: None,
            variables: HashMap::<String, Variable>::new(),
            dimensions: HashMap::<String, Dimension>::new(),
            attributes: HashMap::<String, String>::new(),
            version: 4,
            numrecs: 0,
        }
    }

    pub fn open(filename: String) -> Dataset {
        let mut fp = File::open(filename.clone()).unwrap();
        let filetype_str = unpack_string(&mut fp, 3);
        if filetype_str != "CDF" {
            panic!("This is not a valid Dataset file")
        }
        let version_byte = fp.read_u8().unwrap();
        let numrecs = unpack_int(&mut fp);
        let dim_header = unpack_int(&mut fp);
        match dim_header {
            0 | 10 => println!("Success"),
            _ => panic!("improper header"),
        }
        let dim_count = unpack_int(&mut fp);
        let mut dimensions = HashMap::<String, Dimension>::new();
        for _ in 0..dim_count {
            let name_len = unpack_int(&mut fp);
            let name = unpack_string(&mut fp, name_len as usize);
            let forward_amt = modulo(-1i32 * name_len as i32, 4);
            unpack_string(&mut fp, forward_amt as usize);
            let size = unpack_int(&mut fp);
            dimensions.insert(
                name.clone(),
                Dimension::new(name.clone(), size as u64, Dtype::Float32),
            );
        }
        let attr_header = unpack_int(&mut fp);
        match attr_header {
            0 | 12 => println!("Success reading attrs header: {:#?}", attr_header),
            _ => panic!("improper attribute header"),
        }
        let attr_count = unpack_int(&mut fp);
        let mut attributes = HashMap::<String, String>::new();
        for _ in 0..attr_count {
            let name_len = unpack_int(&mut fp);
            let name = unpack_string(&mut fp, name_len as usize);
            let forward_amt = modulo(-1i32 * name_len as i32, 4);
            unpack_string(&mut fp, forward_amt as usize);
            let nc_type = unpack_int(&mut fp);
            let n = unpack_int(&mut fp);
            let typemap: HashMap<u8, (&str, u8)> =
                [(1, ("b", 1)), (2, ("c", 1))].iter().cloned().collect();
            let (typecode, size) = typemap[&(nc_type as u8)];
            let count = (n as i32) * (size as i32);
            let values = unpack_string(&mut fp, count as usize);
            let forward_amt = modulo(-1i32 * count as i32, 4);
            unpack_string(&mut fp, forward_amt as usize);
            attributes.insert(name, values);
        }
        let variables = HashMap::<String, Variable>::new();
        let varheader = unpack_int(&mut fp);
        match varheader {
            0 | 11 => println!("successfully read varheader: {}", varheader),
            _ => panic!("Couldn't read the varheader"),
        }
        let varcount = unpack_int(&mut fp);
        for v in 0..varcount {
            let name_len = unpack_int(&mut fp);
            let name = unpack_string(&mut fp, name_len as usize);
            println!("{:?}", name);
            let mut dims = vec![];
            // let shape = vec![];
            let dimnum = unpack_int(&mut fp);
            for _d in 0..dimnum {
                let dimid = unpack_int(&mut fp);
                dims.push(dimid);
            }
        }
        Dataset {
            filename: Some(filename),
            variables,
            dimensions,
            attributes,
            version: version_byte as i32,
            numrecs,
        }
    }

    pub fn create_dimension(&mut self, name: String, size: u64, dtype: Dtype) -> Dimension {
        let dim = Dimension {
            name: name.clone(),
            size,
            dtype,
        };
        self.dimensions.insert(name.clone(), dim.clone());
        return dim.clone();
    }

    pub fn create_variable(
        &mut self,
        name: String,
        dtype: Dtype,
        dimensions: Vec<Dimension>,
    ) -> Variable {
        let shape = dimensions
            .iter()
            .map(|r| r.size as u32)
            .collect::<Vec<u32>>();
        let variable = Variable {
            name: name.clone(),
            dtype,
            dimensions,
            shape,
        };
        self.variables.insert(name.clone(), variable.clone());
        return variable;
    }
}
