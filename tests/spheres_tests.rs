use approx::assert_relative_eq;
use cucumber::{World, given, when, then};
use std::collections::HashMap;
use nalgebra::Vector3;
use std::rc::Rc;
use ray_tracer_tdd::ray::Ray;
use ray_tracer_tdd::matrix::Matrix;
use ray_tracer_tdd::material::Material;
mod common;
use common::math_parser::parse_expr;


#[derive(Debug, Default, World)]
pub struct SphereWorld {
    pub rays: HashMap<String, Ray>,
    pub spheres: HashMap<String, ray_tracer_tdd::sphere::Sphere>,
    pub matrices: HashMap<String, Matrix>,
    pub vectors: HashMap<String, Vector3<f64>>,
    pub materials: HashMap<String, Material>,
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

#[given(expr = "{word} ← material\\(\\)" )]
fn given_material(world: &mut SphereWorld, name: String) {
    world.materials.insert(name, Material::new());
}

#[given(expr = "{word}.ambient ← {float}")]
fn given_material_ambient(world: &mut SphereWorld, name: String, val: f64) {
    let m = world.materials.get_mut(&name).expect("material not found");
    m.ambient = val;
}

#[given(expr = "{word} ← translation\\({float}, {float}, {float}\\)")]
fn given_translation(world: &mut SphereWorld, name: String, x: f64, y: f64, z: f64) {
    let m = Matrix::identity().translate(Vector3::new(x, y, z));
    world.matrices.insert(name, m);
}

#[given(expr = "set_transform\\({word}, translation\\({float}, {float}, {float}\\)\\)")]
fn given_set_transform_translation(world: &mut SphereWorld, s_name: String, x: f64, y: f64, z: f64) {
    let m = Matrix::identity().translate(Vector3::new(x, y, z));
    set_sphere_transform(world, s_name, m);
}

#[given(expr = "{word} ← scaling\\({float}, {float}, {float}\\) * rotation_z\\({word}\\)")]
fn given_scaling_rotation_z(world: &mut SphereWorld, name: String, sx: f64, sy: f64, sz: f64, rad_str: String) {
    let rad = parse_expr(&rad_str);
    let m = Matrix::identity().scale(Vector3::new(sx, sy, sz)).rotate_z(rad);
    world.matrices.insert(name, m);
}

#[given(expr = "set_transform\\({word}, {word}\\)")]
fn given_set_transform(world: &mut SphereWorld, s_name: String, t_name: String) {
    let m = *world.matrices.get(&t_name).expect("matrix not found");
    set_sphere_transform(world, s_name, m);
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

fn set_sphere_transform(world: &mut SphereWorld, s_name: String, transform: Matrix) {
    let s = world.spheres.get(&s_name).expect("sphere not found");
    let updated = ray_tracer_tdd::sphere::Sphere::with_transform(&s.name, transform);
    world.spheres.insert(s_name, updated);
}

#[when(expr = "set_transform\\({word}, {word}\\)")]
fn when_set_transform(world: &mut SphereWorld, s_name: String, t_name: String) {
    let m = if t_name == "identity_matrix" {
        Matrix::identity()
    } else {
        *world.matrices.get(&t_name).expect("matrix not found")
    };
    set_sphere_transform(world, s_name, m);
}

#[when(expr = "set_transform\\({word}, scaling\\({float}, {float}, {float}\\)\\)")]
fn when_set_transform_scaling(world: &mut SphereWorld, s_name: String, x: f64, y: f64, z: f64) {
    let m = Matrix::identity().scale(Vector3::new(x, y, z));
    set_sphere_transform(world, s_name, m);
}

#[when(expr = "set_transform\\({word}, translation\\({float}, {float}, {float}\\)\\)")]
fn when_set_transform_translation(world: &mut SphereWorld, s_name: String, x: f64, y: f64, z: f64) {
    let m = Matrix::identity().translate(Vector3::new(x, y, z));
    set_sphere_transform(world, s_name, m);
}

#[when(expr = "{word} ← normal_at\\({word}, point\\({word}, {word}, {word}\\)\\)")]
fn when_normal_at_str(world: &mut SphereWorld, n_name: String, s_name: String, px: String, py: String, pz: String) {
    let pt = Vector3::new(parse_expr(&px), parse_expr(&py), parse_expr(&pz));
    let s = world.spheres.get(&s_name).expect("sphere not found");
    let norm = s.normal_at(pt);
    world.vectors.insert(n_name, norm);
}

#[when(expr = "{word} ← {word}.material")]
fn when_get_material(world: &mut SphereWorld, m_name: String, s_name: String) {
    let s = world.spheres.get(&s_name).expect("sphere not found");
    world.materials.insert(m_name, s.material);
}

#[when(expr = "{word}.material ← {word}")]
fn when_set_sphere_material(world: &mut SphereWorld, s_name: String, m_name: String) {
    let m = *world.materials.get(&m_name).expect("material not found");
    let s = world.spheres.get_mut(&s_name).expect("sphere not found");
    s.material = m;
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

#[then(expr = "{word} = vector\\({word}, {word}, {word}\\)")]
fn then_vector_equals_str(world: &mut SphereWorld, n_name: String, vx: String, vy: String, vz: String) {
    let v = world.vectors.get(&n_name).expect("vector not found");
    let expected = Vector3::new(parse_expr(&vx), parse_expr(&vy), parse_expr(&vz));
    assert_relative_eq!(v.x, expected.x, epsilon = 1e-4);
    assert_relative_eq!(v.y, expected.y, epsilon = 1e-4);
    assert_relative_eq!(v.z, expected.z, epsilon = 1e-4);
}

#[then(expr = "{word} = normalize\\({word}\\)")]
fn then_vector_normalized(world: &mut SphereWorld, n1: String, n2: String) {
    let v1 = world.vectors.get(&n1).expect("vector not found");
    let v2 = world.vectors.get(&n2).expect("vector not found");
    let norm = v2.normalize();
    assert_relative_eq!(v1.x, norm.x, epsilon = 1e-4);
    assert_relative_eq!(v1.y, norm.y, epsilon = 1e-4);
    assert_relative_eq!(v1.z, norm.z, epsilon = 1e-4);
}

#[then(expr = "{word} = material\\(\\)")]
fn then_material_equals_default(world: &mut SphereWorld, m_name: String) {
    let m = world.materials.get(&m_name).expect("material not found");
    assert_eq!(*m, Material::default());
}

#[then(expr = "{word}.material = {word}")]
fn then_sphere_material_equals(world: &mut SphereWorld, s_name: String, m_name: String) {
    let s = world.spheres.get(&s_name).expect("sphere not found");
    let m = world.materials.get(&m_name).expect("material not found");
    assert_eq!(s.material, *m);
}

fn main() {
    futures::executor::block_on(SphereWorld::run(
        "tests/features/spheres.feature"
    ));
}


