use dimension::Dimension;
use dtype::Dtype;
use shape::Shape;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    pub shape: Shape,
    pub dimensions: Vec<Dimension>,
    pub dtype: Dtype,
}
