use std::collections::HashMap;
use std::fs::File;

use dtype::Dtype;
use helpers::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dimension {
    pub name: String,
    pub size: u64,
    pub dtype: Dtype,
}

impl Dimension {
    pub fn new(name: String, size: u64, dtype: Dtype) -> Dimension {
        return Dimension { name, size, dtype };
    }
}
