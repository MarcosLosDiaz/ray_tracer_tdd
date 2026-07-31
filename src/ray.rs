use nalgebra::{Vector3, Vector4};
use std::rc::Rc;
use crate::intersection::{Intersection, Intersections};
use crate::sphere::Sphere;
use crate::matrix::Matrix;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    // store 4D vectors internally for easy transformation: w = 1 for origin, 0 for direction
    pub origin: Vector4<f64>,
    pub direction: Vector4<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        let origin4 = Vector4::new(origin.x, origin.y, origin.z, 1.0);
        let direction4 = Vector4::new(direction.x, direction.y, direction.z, 0.0);
        Self { origin: origin4, direction: direction4 }
    }

    pub fn origin(&self) -> Vector3<f64> {
        Vector3::new(self.origin[0], self.origin[1], self.origin[2])
    }

    pub fn direction(&self) -> Vector3<f64> {
        Vector3::new(self.direction[0], self.direction[1], self.direction[2])
    }

    pub fn position(ray: Ray, t: f64) -> Vector3<f64> {
        ray.origin() + ray.direction() * t
    }

    // Intersect a unit sphere and return intersections
    pub fn intersect_sphere(&self, sphere: Rc<Sphere>) -> Option<Intersections> {
        let r = self.transform(sphere.inverse_transformation);

        let sphere_to_ray = r.origin();
        let direction3 = r.direction();

        let a = direction3.dot(&direction3);
        let b = 2.0 * direction3.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0; // R=1

        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_sqrt= discriminant.sqrt();
        let t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let t2 = (-b + discriminant_sqrt) / (2.0 * a);

        let i1 = Intersection::new(t1, sphere.clone());
        let i2 = Intersection::new(t2, sphere);
        let mut xs = Intersections::new();
        xs.add(i1);
        xs.add(i2);
        Some(xs)
    }

    pub fn transform(&self, transformation: Matrix) -> Ray {
        let new_origin = transformation * self.origin;
        let new_dir = transformation * self.direction;
        Ray { origin: new_origin, direction: new_dir }
    }
}