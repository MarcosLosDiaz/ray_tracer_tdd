use approx::assert_relative_eq;
use cucumber::{World, given, when, then};
use std::collections::HashMap;
use nalgebra::Vector3;
use std::rc::Rc;
use ray_tracer_tdd::ray::Ray;
use ray_tracer_tdd::matrix::Matrix;

#[derive(Debug, Default, World)]
pub struct SphereWorld {
    pub rays: HashMap<String, Ray>,
    pub spheres: HashMap<String, ray_tracer_tdd::sphere::Sphere>,
    pub matrices: HashMap<String, Matrix>,
    // Cache: intersection results as Option<Vec<f64>>
    pub cached_intersections: HashMap<String, Option<Vec<f64>>>,
}

#[given(expr = "{word} ← ray\\(point\\({float}, {float}, {float}\\), vector\\({float}, {float}, {float}\\)\\)")]
fn given_ray_inline(world: &mut SphereWorld, name: String,
                    px: f64, py: f64, pz: f64,
                    vx: f64, vy: f64, vz: f64) {
    let origin = Vector3::new(px, py, pz);
    let direction = Vector3::new(vx, vy, vz);
    world.rays.insert(name, Ray::new(origin, direction));
}

// Create and store a default sphere instance
#[given(expr = "{word} ← sphere\\(\\)")]
fn given_sphere(world: &mut SphereWorld, name: String) {
    world.spheres.insert(name.clone(), ray_tracer_tdd::sphere::Sphere::new(&name));
}

#[given(expr = "{word} ← translation\\({float}, {float}, {float}\\)")]
fn given_translation(world: &mut SphereWorld, name: String, x: f64, y: f64, z: f64) {
    let m = Matrix::identity().translate(Vector3::new(x, y, z));
    world.matrices.insert(name, m);
}

#[when(expr = "{word} ← intersect\\({word}, {word}\\)")]
fn when_intersect(world: &mut SphereWorld, xs_name: String, s_name: String, r_name: String) {
    let ray = *world.rays.get(&r_name).expect("ray not found");
    let s_val = world.spheres.get(&s_name).expect("sphere not found").clone();
    let s_rc = Rc::new(s_val);
    let t_values = match ray.intersect_sphere(s_rc) {
        None => None,
        Some(xs) => Some(xs.all().iter().map(|i| i.t).collect()),
    };
    world.cached_intersections.insert(xs_name, t_values);
}

#[when(expr = "set_transform\\({word}, {word}\\)")]
fn when_set_transform(world: &mut SphereWorld, s_name: String, t_name: String) {
    let m = if t_name == "identity_matrix" {
        Matrix::identity()
    } else {
        *world.matrices.get(&t_name).expect("matrix not found")
    };

    let s = world.spheres.get_mut(&s_name).expect("sphere not found");
    s.set_transform(m);
}

#[when(expr = "set_transform\\({word}, scaling\\({float}, {float}, {float}\\)\\)")]
fn when_set_transform_scaling(world: &mut SphereWorld, s_name: String, x: f64, y: f64, z: f64) {
    let m = Matrix::identity().scale(Vector3::new(x, y, z));
    let s = world.spheres.get_mut(&s_name).expect("sphere not found");
    s.set_transform(m);
}

#[when(expr = "set_transform\\({word}, translation\\({float}, {float}, {float}\\)\\)")]
fn when_set_transform_translation(world: &mut SphereWorld, s_name: String, x: f64, y: f64, z: f64) {
    let m = Matrix::identity().translate(Vector3::new(x, y, z));
    let s = world.spheres.get_mut(&s_name).expect("sphere not found");
    s.set_transform(m);
}

#[then(expr = "{word}.transform = {word}")]
fn then_transform_equals(world: &mut SphereWorld, s_name: String, t_name: String) {
    let s = world.spheres.get(&s_name).expect("sphere not found");
    let expected = if t_name == "identity_matrix" {
        Matrix::identity()
    } else {
        *world.matrices.get(&t_name).expect("matrix not found")
    };
    assert_relative_eq!(s.transformation.data, expected.data);
}

#[then(expr = "{word}.count = 0")]
fn then_count_zero(world: &mut SphereWorld, xs_name: String) {
    let opt = world.cached_intersections.get(&xs_name).expect("intersection not found");
    assert!(opt.is_none());
}

#[then(expr = "{word}.count = 2")]
fn then_count_two(world: &mut SphereWorld, xs_name: String) {
    let opt = world.cached_intersections.get(&xs_name).expect("intersection not found");
    assert!(opt.is_some(), "expected intersections but found None");
    let t_values = opt.as_ref().unwrap();
    assert_eq!(t_values.len(), 2, "intersection size is different than 2");
}

#[then(expr = "{word}[{int}] = {float}")]
#[then(expr = "{word}[{int}].t = {float}")]
fn then_index_equals(world: &mut SphereWorld, xs_name: String, index: usize, expected: f64) {
    let opt = world.cached_intersections.get(&xs_name).expect("intersection not found");
    let t_values = opt.as_ref().expect("expected intersections but found None");
    let actual = t_values.get(index).expect("index out of bounds");
    assert_relative_eq!(*actual, expected);
}

fn main() {
    futures::executor::block_on(SphereWorld::run(
        "tests/features/spheres.feature"
    ));
}
