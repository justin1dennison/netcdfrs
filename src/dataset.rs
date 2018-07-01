use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::fs::File;

use attributes::Attribute;
use dimension::Dimension;
use dtype::Dtype;
use helpers::{unpack_int, unpack_string};
use variable::Variable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dataset {
    pub filename: Option<String>,
    pub variables: HashMap<String, Variable>,
    pub dimensions: HashMap<String, Dimension>,
    pub attributes: HashMap<String, String>,
    pub version: i32,
    pub numrecs: i32,
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
        let dimensions = Dimension::from_file(&mut fp);
        let attributes = Attribute::from_file(&mut fp);
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
