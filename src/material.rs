use nalgebra::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Vector3<f64>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Self {
            color: Vector3::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}
