extern crate byteorder;

use std::fs::{File};
use std::io::Read;
use std::collections::HashMap;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub enum Dtype {
    Float32,
    Float64,
    Int16,
    Int32,
    Int64,
    Str
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    shape: Shape,
    dimensions: Vec<Dimension>,
    dtype: Dtype
}
#[derive(Debug, Clone)]
pub struct Dimension {
    name: String,
    size: u64,
    dtype: Dtype
}

impl Dimension {
    pub fn new(name: String, size: u64, dtype: Dtype) -> Dimension {
        return Dimension {
            name,
            size, 
            dtype
        }
    }
}

pub type Shape = Vec<u32>;

#[derive(Debug, Clone)]
pub struct NetCDF {
    filename: Option<String>,
    variables: HashMap<String, Variable>,
    dimensions: HashMap<String, Dimension>,
    version: u32,
    numrecs: u32,
}

impl NetCDF {
    pub fn new() -> NetCDF {
        NetCDF{
            filename: None,
            variables: HashMap::<String, Variable>::new(),
            dimensions: HashMap::<String, Dimension>::new(),
            version: 4,
            numrecs: 0
        }  
    }
    pub fn open(filename: String) -> NetCDF {
        let mut fp = File::open(filename.clone()).unwrap();
        let mut filetype = [0; 3];
        if let Err(msg) = fp.read(&mut filetype) {
            panic!("Read Error: {:?}", msg)
        }
        let filetype_str = filetype.iter().map(|n| *n).map(char::from).collect::<String>();
        if filetype_str != "CDF" {
            panic!("This is not a valid NetCDF file") 
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
        let pymodulo = |a, b| { ((a % b) + b) % b };
        for _ in 0..dim_count {
            let name_len = fp.read_u32::<BigEndian>().unwrap();
            let mut name = vec![0u8; name_len as usize];
            fp.read(&mut name).unwrap();
            let mut buf = vec![0; pymodulo(-1i32 * name_len as i32, 4) as usize];
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
        NetCDF {
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create_a_netcdf_file() {
        let expected = "awesome.nc".to_string();
        let actual = NetCDF::open("awesome.nc".to_string());
        assert_eq!(expected, actual.filename.unwrap());
    }
}
