use approx::assert_relative_eq;
use cucumber::{given, then, when, World};
use nalgebra::{SMatrix, SVector, Vector3};
use ray_tracer_tdd::matrix::Matrix;
use std::collections::HashMap;
use std::f64::consts::PI;

#[derive(Debug, Default, World)]
pub struct TransformationWorld {
    matrices: HashMap<String, Matrix>,
    tuples: HashMap<String, SVector<f64, 4>>,
}

#[given(expr = "{word} ← translation\\({float}, {float}, {float}\\)")]
fn given_translation(world: &mut TransformationWorld, name: String, x: f64, y: f64, z: f64) {
    let identity = Matrix::new(SMatrix::<f64, 4, 4>::identity());
    let translation = identity.translate(Vector3::new(x, y, z));
    world.matrices.insert(name, translation);
}

#[given(expr = "{word} ← scaling\\({float}, {float}, {float}\\)")]
fn given_scaling(world: &mut TransformationWorld, name: String, x: f64, y: f64, z: f64) {
    let identity = Matrix::new(SMatrix::<f64, 4, 4>::identity());
    let scaling = identity.scale(Vector3::new(x, y, z));
    world.matrices.insert(name, scaling);
}

#[given(expr = "{word} ← rotation_x\\(π \\/ {float}\\)")]
fn given_rotation_x(world: &mut TransformationWorld, name: String, divisor: f64) {
    let identity = Matrix::new(SMatrix::<f64, 4, 4>::identity());
    let rotation = identity.rotate_x(PI / divisor);
    world.matrices.insert(name, rotation);
}

#[given(expr = "{word} ← rotation_y\\(π \\/ {float}\\)")]
fn given_rotation_y(world: &mut TransformationWorld, name: String, divisor: f64) {
    let identity = Matrix::new(SMatrix::<f64, 4, 4>::identity());
    let rotation = identity.rotate_y(PI / divisor);
    world.matrices.insert(name, rotation);
}

#[given(expr = "{word} ← rotation_z\\(π \\/ {float}\\)")]
fn given_rotation_z(world: &mut TransformationWorld, name: String, divisor: f64) {
    let identity = Matrix::new(SMatrix::<f64, 4, 4>::identity());
    let rotation = identity.rotate_z(PI / divisor);
    world.matrices.insert(name, rotation);
}

#[given(expr = "{word} ← shearing\\({float}, {float}, {float}, {float}, {float}, {float}\\)")]
fn given_shearing(world: &mut TransformationWorld, name: String, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) {
    let identity = Matrix::new(SMatrix::<f64, 4, 4>::identity());
    let shearing = identity.shear(xy, xz, yx, yz, zx, zy);
    world.matrices.insert(name, shearing);
}

#[given(expr = "{word} ← point\\({float}, {float}, {float}\\)")]
fn given_point(world: &mut TransformationWorld, name: String, x: f64, y: f64, z: f64) {
    let point = SVector::<f64, 4>::new(x, y, z, 1.0);
    world.tuples.insert(name, point);
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float}\\)")]
fn given_vector(world: &mut TransformationWorld, name: String, x: f64, y: f64, z: f64) {
    let vector = SVector::<f64, 4>::new(x, y, z, 0.0);
    world.tuples.insert(name, vector);
}

fn parse_coord(s: &str) -> f64 {
    match s.trim() {
        "√2/2" => 2.0f64.sqrt() / 2.0,
        "-√2/2" => -2.0f64.sqrt() / 2.0,
        val => val.parse::<f64>().unwrap(),
    }
}

#[then(regex = r"^(\w+) \* (\w+) = point\((.+), (.+), (.+)\)$")]
fn check_transform_point_regex(
    world: &mut TransformationWorld,
    matrix_name: String,
    point_name: String,
    x_str: String,
    y_str: String,
    z_str: String,
) {
    let x = parse_coord(&x_str);
    let y = parse_coord(&y_str);
    let z = parse_coord(&z_str);

    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let point = world.tuples.get(&point_name).expect("Point not found");

    let result = matrix.data * point;
    let expected = SVector::<f64, 4>::new(x, y, z, 1.0);

    assert_relative_eq!(result, expected, epsilon = 1e-5);
}

#[then(expr = "{word} * {word} = vector\\({float}, {float}, {float}\\)")]
fn check_transform_vector(
    world: &mut TransformationWorld,
    matrix_name: String,
    vector_name: String,
    x: f64,
    y: f64,
    z: f64,
) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let vector = world.tuples.get(&vector_name).expect("Vector not found");

    let result = matrix.data * vector;
    let expected = SVector::<f64, 4>::new(x, y, z, 0.0);

    assert_relative_eq!(result, expected, epsilon = 1e-5);
}

#[then(expr = "{word} * {word} = {word}")]
fn check_transform_vector_equality(
    world: &mut TransformationWorld,
    matrix_name: String,
    vector_name: String,
    expected_name: String,
) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let vector = world.tuples.get(&vector_name).expect("Vector not found");
    let expected = world
        .tuples
        .get(&expected_name)
        .expect("Expected vector not found");

    let result = matrix.data * vector;

    assert_relative_eq!(result, expected);
}

#[given(expr = "{word} ← inverse\\({word}\\)")]
fn given_inverse(world: &mut TransformationWorld, inv_name: String, matrix_name: String) {
    let matrix = world.matrices.get(&matrix_name).expect("Matrix not found");
    let inv_data = matrix.data.try_inverse().expect("Matrix is not invertible");
    world.matrices.insert(inv_name, Matrix::new(inv_data));
}

#[when(expr = "{word} ← {word} * {word}")]
fn when_transform(world: &mut TransformationWorld, result_name: String, m_name: String, t_name: String) {
    let m1 = world.matrices.get(&m_name).expect("Matrix not found");
    if let Some(m2) = world.matrices.get(&t_name) {
        // matrix * matrix
        let result = Matrix::new(m1.data * m2.data);
        world.matrices.insert(result_name, result);
    } else {
        // matrix * tuple
        let tuple = world.tuples.get(&t_name).expect("Tuple not found");
        let result = m1.data * tuple;
        world.tuples.insert(result_name, result);
    }
}

#[when(expr = "{word} ← {word} * {word} * {word}")]
fn when_chain_transform(world: &mut TransformationWorld, res_name: String, m1_name: String, m2_name: String, m3_name: String) {
    let m1 = world.matrices.get(&m1_name).expect("Matrix not found");
    let m2 = world.matrices.get(&m2_name).expect("Matrix not found");
    let m3 = world.matrices.get(&m3_name).expect("Matrix not found");
    let result_matrix = Matrix::new(m1.data * m2.data * m3.data);
    world.matrices.insert(res_name, result_matrix);
}

#[then(regex = r"^(\w+) = point\((.+), (.+), (.+)\)$")]
fn then_check_point_equality(world: &mut TransformationWorld, name: String, x_str: String, y_str: String, z_str: String) {
    let p = world.tuples.get(&name).expect("Point not found");
    let x = parse_coord(&x_str);
    let y = parse_coord(&y_str);
    let z = parse_coord(&z_str);
    let expected = SVector::<f64, 4>::new(x, y, z, 1.0);
    assert_relative_eq!(*p, expected, epsilon = 1e-5);
}

fn main() {
    futures::executor::block_on(TransformationWorld::run(
        "tests/features/transformations.feature",
    ));
}