use crate::matrix::Matrix;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub name: String,
    pub transformation: Matrix,
    pub inverse_transformation: Matrix,
}

impl Sphere {
    pub fn new(name: &str) -> Self {
        Sphere { 
            name: name.to_string(),
            transformation: Matrix::identity(),
            inverse_transformation: Matrix::identity(),
        }
    }

    pub fn with_transform(name: &str, transform: Matrix) -> Self {
        Sphere { 
            name: name.to_string(),
            transformation: transform,
            inverse_transformation: transform.inverse(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transformation = transform;
        self.inverse_transformation = transform.inverse();
    }
}
