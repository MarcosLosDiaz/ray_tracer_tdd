use approx::assert_relative_eq;
use cucumber::{World, given, when, then};
use std::collections::HashMap;
use nalgebra::Vector3;
use ray_tracer_tdd::matrix::Matrix;
use ray_tracer_tdd::ray::Ray;

#[derive(Debug, Default, World)]
pub struct RayWorld {
    pub vectors: HashMap<String, Vector3<f64>>,
    pub rays: HashMap<String, Ray>,
    pub matrices: HashMap<String, Matrix>,
}

#[given(expr = "{word} ← point\\({float}, {float}, {float}\\)")]
fn given_point(world: &mut RayWorld, name: String, x: f64, y: f64, z: f64) {
    world.vectors.insert(name, Vector3::new(x, y, z));
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float}\\)")]
fn given_vector(world: &mut RayWorld, name: String, x: f64, y: f64, z: f64) {
    world.vectors.insert(name, Vector3::new(x, y, z));
}

#[given(expr = "{word} ← translation\\({float}, {float}, {float}\\)")]
fn given_translation(world: &mut RayWorld, name: String, x: f64, y: f64, z: f64) {
    let translation = Matrix::new(Matrix::translation_matrix(x, y, z));
    world.matrices.insert(name, translation);
}

#[given(expr = "{word} ← scaling\\({float}, {float}, {float}\\)")]
fn given_scaling(world: &mut RayWorld, name: String, x: f64, y: f64, z: f64) {
    let scaling = Matrix::new(Matrix::scaling_matrix(x, y, z));
    world.matrices.insert(name, scaling);
}

#[when(expr = "{word} ← transform\\({word}, {word}\\)")]
fn when_transform_ray(world: &mut RayWorld, result_name: String, r_name: String, m_name: String) {
    let ray = *world.rays.get(&r_name).expect("ray not found");
    let m = world.matrices.get(&m_name).expect("matrix not found");

    let origin4 = ray.origin;
    let dir4 = ray.direction;

    let new_origin4 = m.data * origin4;
    let new_dir4 = m.data * dir4;

    let new_origin = Vector3::new(new_origin4[0], new_origin4[1], new_origin4[2]);
    let new_dir = Vector3::new(new_dir4[0], new_dir4[1], new_dir4[2]);

    world.rays.insert(result_name, Ray::new(new_origin, new_dir));
}

#[when(expr = "{word} ← ray\\({word}, {word}\\)")]
fn when_ray_from_names(world: &mut RayWorld, name: String, origin_name: String, direction_name: String) {
    let origin = *world.vectors.get(&origin_name).expect("origin not found");
    let direction = *world.vectors.get(&direction_name).expect("direction not found");
    world.rays.insert(name, Ray::new(origin, direction));
}

#[given(expr = "{word} ← ray\\(point\\({float}, {float}, {float}\\), vector\\({float}, {float}, {float}\\)\\)")]
fn given_ray_with_inline_point_vector(world: &mut RayWorld, name: String,
                                       px: f64, py: f64, pz: f64,
                                       vx: f64, vy: f64, vz: f64) {
    let origin = Vector3::new(px, py, pz);
    let direction = Vector3::new(vx, vy, vz);
    world.rays.insert(name, Ray::new(origin, direction));
}

#[then(expr = "{word}.origin = {word}")]
fn then_ray_origin_equals(world: &mut RayWorld, ray_name: String, vec_name: String) {
    let ray = world.rays.get(&ray_name).expect("ray not found");
    let expected = *world.vectors.get(&vec_name).expect("vector not found");
    assert_relative_eq!(ray.origin(), expected);
}

#[then(expr = "{word}.direction = {word}")]
fn then_ray_direction_equals(world: &mut RayWorld, ray_name: String, vec_name: String) {
    let ray = world.rays.get(&ray_name).expect("ray not found");
    let expected = *world.vectors.get(&vec_name).expect("vector not found");
    assert_relative_eq!(ray.direction(), expected);
}

#[then(expr = "{word}.origin = point\\({float}, {float}, {float}\\)")]
fn then_ray_origin_inline(world: &mut RayWorld, ray_name: String, x: f64, y: f64, z: f64) {
    let ray = world.rays.get(&ray_name).expect("ray not found");
    let expected = Vector3::new(x, y, z);
    assert_relative_eq!(ray.origin(), expected);
}

#[then(expr = "{word}.direction = vector\\({float}, {float}, {float}\\)")]
fn then_ray_direction_inline(world: &mut RayWorld, ray_name: String, x: f64, y: f64, z: f64) {
    let ray = world.rays.get(&ray_name).expect("ray not found");
    let expected = Vector3::new(x, y, z);
    assert_relative_eq!(ray.direction(), expected);
}

#[then(expr = "position\\({word}, {float}\\) = point\\({float}, {float}, {float}\\)")]
fn then_position_equals_point(world: &mut RayWorld, ray_name: String, t: f64, x: f64, y: f64, z: f64) {
    let ray = *world.rays.get(&ray_name).expect("ray not found");
    let pos = Ray::position(ray, t);
    assert_relative_eq!(pos, Vector3::new(x, y, z));
}

fn main() {
    futures::executor::block_on(RayWorld::run(
        "tests/features/rays.feature"
    ));
}
