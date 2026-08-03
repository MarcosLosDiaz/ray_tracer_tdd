use nalgebra::Vector3;
use crate::material::Material;
use crate::light::Light;

pub const COLOR_BLACK: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);

pub struct PhongShader;

impl PhongShader {
    pub fn reflect(in_vec: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
        in_vec - normal * 2.0 * in_vec.dot(&normal)
    }

    pub fn lighting(
        material: &Material,
        light: &Light,
        point: Vector3<f64>,
        eyev: Vector3<f64>,
        normalv: Vector3<f64>,
    ) -> Vector3<f64> {
        let effective_color = material.color.component_mul(&light.intensity);
        let lightv = (light.position - point).normalize();

        let ambient = effective_color * material.ambient;

        let light_dot_normal = lightv.dot(&normalv);

        if light_dot_normal < 0.0 {
            return ambient;
        }

        let diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = Self::reflect(-lightv, normalv);
        let reflect_dot_eye = reflectv.dot(&eyev);

        let specular = if reflect_dot_eye <= 0.0 {
            COLOR_BLACK
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            light.intensity * material.specular * factor
        };

        ambient + diffuse + specular
    }
}


