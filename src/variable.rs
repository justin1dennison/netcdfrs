use std::collections::HashMap;
use dtype::Dtype;
use shape::Shape;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    pub shape: Shape,
    pub dimensions: Vec<String>,
    pub dtype: Dtype,
    pub attributes: HashMap<String, String>
}
