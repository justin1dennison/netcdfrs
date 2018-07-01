extern crate byteorder;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod dataset;
pub mod dimension;
pub mod dtype;
pub mod shape;
pub mod variable;

pub mod prelude {
    pub use dataset::*;
    pub use dimension::*;
    pub use dtype::*;
    pub use shape::*;
    pub use variable::*;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn can_create_a_netcdf_file() {
//         let expected = "awesome.nc".to_string();
//         let actual = dataset::Dataset::open("awesome.nc".to_string());
//         assert_eq!(expected, actual.filename.unwrap());
//     }
// }
