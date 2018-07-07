use dtype::Dtype;
use shape::Shape;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    pub shape: Option<Shape>,
    pub typecode:  Option<String>,
    pub size: Option<u64>,
    pub dimensions: Option<Vec<String>>,
    pub attributes: Option<HashMap<String, String>>,
    pub dtype: Option<Dtype>
}
