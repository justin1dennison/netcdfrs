use byteorder::ReadBytesExt;
use std::collections::HashMap;
use std::fs::File;

use constants::typemap;
use dimension::Dimension;
use dtype::Dtype;
use helpers::*;
use variable::Variable;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dataset {
    pub filename: Option<String>,
    pub variables: HashMap<String, Variable>,
    pub dimensions: HashMap<String, Dimension>,
    pub attributes: HashMap<String, String>,
    pub version: Option<u8>,
    pub numrecs: Option<i32>,
}

fn read_vars(
    fp: &mut File,
    dimensions: Vec<Dimension>,
    version_byte: u8,
) -> HashMap<String, Variable> {
    let mut variables = HashMap::<String, Variable>::new();
    let varheader = unpack_int(fp);
    if varheader != 0 | 11 {
        panic!("Couldn't read the varheader");
    }
    let varcount = unpack_int(fp);
    for _ in 0..varcount {
        let name_len = unpack_int(fp);
        let name = unpack_string(fp, name_len as usize);
        let forward_amt = modulo(-1i32 * name_len as i32, 4);
        unpack_string(fp, forward_amt as usize);
        let dimnum = unpack_int(fp);
        let mut dimnames = vec![];
        let mut shape = vec![];
        for _ in 0..dimnum {
            let id: i32 = unpack_int(fp);
            let dim = &dimensions.get(id as usize).unwrap();
            dimnames.push(dim.name.clone());
            shape.push(&dim.size);
        }
        let var_attrs = read_attrs(fp);
        let nc_type = unpack_int(fp);
        let vsize = unpack_int(fp);
        let begin = if version_byte - 1 == 0 {
            unpack_int(fp)
        } else {
            unpack_int64(fp) as i32
        };
        let (typecode, size) = typemap()[&(nc_type as u8)];
        let dtype = format!(">{}", typecode);
        variables.insert(
            name.clone(),
            Variable::new(name)
                .with_attributes(var_attrs)
                .with_typecode(typecode.to_string())
                .with_dtype(Dtype::Float32)
                .with_dimensions(dimnames)
                .with_size(shape.iter().fold(1, |acc, next| acc * **next) as i64)
                .with_shape(shape.iter().map(|v| **v as u32).collect()),
        );
    }
    variables
}

fn read_dims(fp: &mut File) -> Vec<Dimension> {
    let dim_header = unpack_int(fp);
    if dim_header != 0 | 10 {
        panic!("improper header");
    }
    let dim_count = unpack_int(fp);
    let mut dimensions = Vec::<Dimension>::new();
    for _ in 0..dim_count {
        let name_len = unpack_int(fp);
        let name = unpack_string(fp, name_len as usize);
        let forward_amt = modulo(-1i32 * name_len as i32, 4);
        unpack_string(fp, forward_amt as usize);
        let size = unpack_int(fp);
        let dimension = Dimension::new(name.clone(), size as u64, Dtype::Float32);
        dimensions.push(dimension);
    }
    dimensions
}

fn read_attrs(fp: &mut File) -> HashMap<String, String> {
    let attr_header = unpack_int(fp);
    if attr_header != 0 | 12 {
        panic!("improper attribute header");
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
        let (_typecode, size) = typemap()[&(nc_type as u8)];
        let count = (n as i32) * (size as i32);
        let values = unpack_string(fp, count as usize);
        let forward_amt = modulo(-1i32 * count as i32, 4);
        unpack_string(fp, forward_amt as usize);
        attributes.insert(name, values);
    }
    attributes
}

impl Dataset {
    pub fn new() -> Dataset {
        Dataset {
            filename: None,
            variables: HashMap::<String, Variable>::new(),
            dimensions: HashMap::<String, Dimension>::new(),
            attributes: HashMap::<String, String>::new(),
            version: None,
            numrecs: None,
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
        let dimensions = read_dims(&mut fp);
        let dimensions_map = &dimensions
            .iter()
            .map(|d| (d.name.clone(), d.clone()))
            .collect::<HashMap<String, Dimension>>();
        let attributes = read_attrs(&mut fp);
        let variables  = read_vars(&mut fp, dimensions, version_byte);
        Dataset {
            filename: Some(filename),
            variables,
            dimensions: dimensions_map.clone(),
            attributes,
            version: Some(version_byte),
            numrecs: Some(numrecs),
        }
    }
}
