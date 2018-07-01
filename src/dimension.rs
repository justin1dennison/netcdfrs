use std::fs::File;
use std::collections::HashMap;

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

    pub fn from_file(fp: &mut File) -> HashMap<String, Dimension> {
        let dim_header = unpack_int(fp);
        match dim_header {
            0 | 10 => println!("Success"),
            _ => panic!("improper header"),
        }
        let dim_count = unpack_int(fp);
        let mut dimensions = HashMap::<String, Dimension>::new();
        for _ in 0..dim_count {
            let name_len = unpack_int(fp);
            let name = unpack_string(fp, name_len as usize);
            let forward_amt = modulo(-1i32 * name_len as i32, 4);
            unpack_string(fp, forward_amt as usize);
            let size = unpack_int(fp);
            dimensions.insert(
                name.clone(),
                Dimension::new(name.clone(), size as u64, Dtype::Float32),
            );
        }
        dimensions
    }

}
