
use std::fs::{File};

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

pub type Shape = Vec<u32>;

#[derive(Debug, Clone)]
pub struct NetCDF {
    filename: Option<String>,
    variables: Vec<Variable>,
    dimensions: Vec<Dimension>
}

impl NetCDF {
    pub fn new() -> NetCDF {
        NetCDF{
            filename: None,
            variables: vec![],
            dimensions: vec![]
        }  
    }
    pub fn open(filename: String) -> NetCDF {
        let variables = vec![];
        let dimensions = vec![];
        NetCDF {
            filename: Some(filename),
            variables,
            dimensions
        } 
    }

    pub fn add_dimension(&mut self, name: String, size: u64, dtype: Dtype) -> Dimension {
        let dim = Dimension { name, size, dtype };
        self.dimensions.push(dim.clone());
        return dim;
    }

    pub fn add_variable(&mut self, name: String, dtype: Dtype, dimensions: Vec<Dimension>) -> Variable{
        let shape = dimensions.iter().map(|r| r.size as u32).collect::<Vec<u32>>();
        let variable = Variable{ name, dtype, dimensions, shape };
        self.variables.push(variable.clone());
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
        assert_eq!(expected, actual.filename);
    }
}
