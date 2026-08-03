use nalgebra::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    pub position: Vector3<f64>,
    pub intensity: Vector3<f64>,
}

impl Light {
    pub fn new(position: Vector3<f64>, intensity: Vector3<f64>) -> Self {
        Self { position, intensity }
    }
}
