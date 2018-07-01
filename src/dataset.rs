use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::fs::File;

use attributes::Attribute;
use constants::typemap;
use dimension::Dimension;
use dtype::Dtype;
use helpers::*;
use shape::Shape;
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
        let dimensions_map = &dimensions
            .iter()
            .map(|d| (d.name.clone(), d.clone()))
            .collect::<HashMap<String, Dimension>>();
        let attributes = Attribute::from_file(&mut fp);
        let mut variables = HashMap::<String, Variable>::new();
        let varheader = unpack_int(&mut fp);
        match varheader {
            0 | 11 => varheader,
            _ => panic!("Couldn't read the varheader"),
        };
        let varcount = unpack_int(&mut fp);
        //this is the single iteration
        for v in 0..varcount {
            let name_len = unpack_int(&mut fp);
            let name = unpack_string(&mut fp, name_len as usize);
            let forward_amt = modulo(-1i32 * name_len as i32, 4);
            unpack_string(&mut fp, forward_amt as usize);
            let dimnum = unpack_int(&mut fp);
            let mut dimnames = vec![];
            let mut shape = vec![];
            for i in 0..dimnum {
                let id: i32 = unpack_int(&mut fp);
                let dim = &dimensions.get(id as usize).unwrap();
                dimnames.push(dim.name.clone());
                shape.push(&dim.size);
            }
            let var_attrs = Attribute::from_file(&mut fp);
            let nc_type = unpack_int(&mut fp);
            let vsize = unpack_int(&mut fp);
            let begin = if version_byte - 1 == 0 {
                unpack_int(&mut fp)
            } else {
                unpack_int64(&mut fp) as i32
            };
            let (typecode, size) = typemap()[&(nc_type as u8)];
            let dtype = format!(">{}", typecode);
            let variable = Variable {
                name,
                shape: shape.iter().map(|v| **v as u32).collect(),
                dtype: Dtype::Float32,
                dimensions: dimnames,
                attributes: var_attrs
            };
            variables.insert(variable.name.clone(), variable);
        }
        Dataset {
            filename: Some(filename),
            variables,
            dimensions: dimensions_map.clone(),
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
        attributes: HashMap<String, String>
    ) -> Variable {
        let shape = dimensions
            .iter()
            .map(|r| r.size as u32)
            .collect::<Vec<u32>>();
        let variable = Variable {
            name: name.clone(),
            dtype,
            dimensions: dimensions.iter().map(|d| d.name.clone()).collect(),
            shape,
            attributes
        };
        self.variables.insert(name.clone(), variable.clone());
        return variable;
    }
}
