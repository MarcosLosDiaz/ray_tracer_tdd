use nalgebra::{Vector3, Vector4};

use crate::matrix::Matrix;
use crate::material::Material;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub name: String,
    pub transformation: Matrix,
    pub inverse_transformation: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new(name: &str) -> Self {
        Sphere { 
            name: name.to_string(),
            transformation: Matrix::identity(),
            inverse_transformation: Matrix::identity(),
            material: Material::default(),
        }
    }

    pub fn with_transform(name: &str, transform: Matrix) -> Self {
        Sphere { 
            name: name.to_string(),
            transformation: transform,
            inverse_transformation: transform.inverse(),
            material: Material::default(),
        }
    }

    pub fn normal_at(&self, world_point: Vector3<f64>) -> Vector3<f64> {
        let object_point = self.inverse_transformation * Vector4::new(world_point.x, world_point.y, world_point.z, 1.0);
        let object_normal = Vector4::new(object_point.x, object_point.y, object_point.z, 0.0);
        let mut world_normal = self.inverse_transformation.transpose() * object_normal;
        // make sure to clean w in case of translations
        world_normal.w = 0.0;
        Vector3::new(world_normal.x, world_normal.y, world_normal.z).normalize()
    }
}

