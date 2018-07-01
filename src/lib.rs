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
mod helpers;

pub mod prelude {
    pub use dataset::Dataset;
    pub use dimension::Dimension;
    pub use dtype::Dtype;
    pub use shape::Shape;
    pub use variable::Variable;
}

