
use std::collections::HashMap;
use std::fs::{File};
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};


use variable::{Variable};
use dimension::{Dimension};
use dtype::{Dtype};

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dataset {
    filename: Option<String>,
    variables: HashMap<String, Variable>,
    dimensions: HashMap<String, Dimension>,
    version: u32,
    numrecs: u32,
}

impl Dataset {
    pub fn new() -> Dataset {
        Dataset{
            filename: None,
            variables: HashMap::<String, Variable>::new(),
            dimensions: HashMap::<String, Dimension>::new(),
            version: 4,
            numrecs: 0
        }  
    }

    pub fn open(filename: String) -> Dataset {
        let mut fp = File::open(filename.clone()).unwrap();
        let mut filetype = [0; 3];
        if let Err(msg) = fp.read(&mut filetype) {
            panic!("Read Error: {:?}", msg)
        }
        let filetype_str = filetype.iter().map(|n| *n).map(char::from).collect::<String>();
        if filetype_str != "CDF" {
            panic!("This is not a valid Dataset file") 
        }
        let version_byte = fp.read_u8().unwrap();
        let numrecs = fp.read_u32::<BigEndian>().unwrap();
        let dim_header = fp.read_u32::<BigEndian>().unwrap();
        match dim_header {
            0 | 10 => println!("Success"),
            _ => panic!("improper header")
        }
        let dim_count = fp.read_u32::<BigEndian>().unwrap();
        let mut dimensions = HashMap::<String, Dimension>::new();
        for _ in 0..dim_count {
            let name_len = fp.read_u32::<BigEndian>().unwrap();
            let mut name = vec![0u8; name_len as usize];
            fp.read(&mut name).unwrap();
            let mut buf = vec![0; modulo(-1i32 * name_len as i32, 4) as usize];
            fp.read(&mut buf).unwrap();
            let size = fp.read_u32::<BigEndian>().unwrap();
            let name_string = &String::from_utf8(name).unwrap();
            dimensions.insert(name_string.to_string(), Dimension::new(
                name_string.clone(),
                size as u64,
                Dtype::Float32
            ));
        }

        let variables = HashMap::<String, Variable>::new();
        Dataset {
            filename: Some(filename),
            variables,
            dimensions,
            version: version_byte as u32,
            numrecs
        } 
    }

    pub fn create_dimension(&mut self, name: String, size: u64, dtype: Dtype) -> Dimension {
        let dim = Dimension { name: name.clone(), size, dtype };
        self.dimensions.insert(name.clone(), dim.clone());
        return dim.clone();
    }

    pub fn create_variable(&mut self, name: String, dtype: Dtype, dimensions: Vec<Dimension>) -> Variable{
        let shape = dimensions.iter().map(|r| r.size as u32).collect::<Vec<u32>>();
        let variable = Variable{ name: name.clone(), dtype, dimensions, shape };
        self.variables.insert(name.clone(), variable.clone());
        return variable;
    }
}