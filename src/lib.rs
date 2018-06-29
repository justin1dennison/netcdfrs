extern crate byteorder;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod variable;
pub mod dimension;
pub mod shape;
pub mod dataset;
pub mod dtype;

pub mod prelude {
    pub use variable::*;
    pub use dimension::*;
    pub use shape::*;
    pub use dataset::*;
    pub use dtype::*; 
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
