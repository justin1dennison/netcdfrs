use dtype::Dtype;
use shape::Shape;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Variable {
    pub name: String,
    pub shape: Option<Shape>,
    pub typecode: Option<String>,
    pub size: Option<i64>,
    pub dimensions: Option<Vec<String>>,
    pub attributes: Option<HashMap<String, String>>,
    pub dtype: Option<Dtype>,
    pub data: Option<Vec<f64>>,
}

impl Variable {
    pub fn new(name: String) -> Variable {
        Variable {
            name,
            shape: None,
            typecode: None,
            size: None,
            dimensions: None,
            attributes: None,
            dtype: None,
            data: None,
        }
    }

    pub fn with_shape(mut self, shape: Shape) -> Variable {
        self.shape = Some(shape);
        self
    }

    pub fn with_typecode(mut self, typecode: String) -> Variable {
        self.typecode = Some(typecode);
        self
    }

    pub fn with_size(mut self, size: i64) -> Variable {
        self.size = Some(size);
        self
    }

    pub fn with_dimensions(mut self, dimensions: Vec<String>) -> Variable {
        self.dimensions = Some(dimensions);
        self
    }

    pub fn with_attributes(mut self, attributes: HashMap<String, String>) -> Variable {
        self.attributes = Some(attributes);
        self
    }

    pub fn with_dtype(mut self, dtype: Dtype) -> Variable {
        self.dtype = Some(dtype);
        self
    }

    pub fn with_data(mut self, data: Vec<f64>) -> Variable {
        self.data = Some(data);
        self
    }
}
