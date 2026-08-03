use approx::{assert_relative_eq, relative_eq};
use cucumber::{World, given, then, when};
use std::collections::HashMap;
use nalgebra::{Vector3, Vector4};

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    pub tuples: HashMap<String, Vector4<f64>>,
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float}\\)")]
fn given_tuple(world: &mut TupleWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.tuples.insert(name, Vector4::new(x, y, z, w));
}

#[then(expr = "{word}.{word} = {float}")]
fn check_tuple_component(world: &mut TupleWorld, name: String, component: String, expected: f64) {
    let tuple = world.tuples.get(&name).expect("Tuple not found");
    let actual = match component.as_str() {
        "x" => tuple.x,
        "y" => tuple.y,
        "z" => tuple.z,
        "w" => tuple.w,
        _ => panic!("Unknown component: {}", component),
    };
    assert_relative_eq!(actual, expected);
}

#[then(expr = "{word} is a {word}")]
fn check_tuple_is_type(world: &mut TupleWorld, name: String, object_type: String) {
    let tuple = world.tuples.get(&name).expect("Tuple not found");
    let w = tuple.w;
    let expected = match object_type.as_str() {
        "point" => 1.0,
        "vector" => 0.0,
        _ => panic!("Unknown object type: {}", object_type),
    };
    assert_eq!(w, expected);
}

#[then(expr = "{word} is not a {word}")]
fn check_tuple_is_not_type(world: &mut TupleWorld, name: String, object_type: String) {
    let tuple = world.tuples.get(&name).expect("Tuple not found");
    let w = tuple.w;
    let expected = match object_type.as_str() {
        "point" => 1.0,
        "vector" => 0.0,
        _ => panic!("Unknown object type: {}", object_type),
    };
    assert!(!relative_eq!(w, expected));
}

#[given(expr = "{word} ← {word}\\({word}, {word}, {word}\\)")]
fn assign_point_or_vector(world: &mut TupleWorld, name: String, type_name: String, x: String, y: String, z: String) {
    let xv = parse_expr(&x);
    let yv = parse_expr(&y);
    let zv = parse_expr(&z);
    let tuple = match type_name.as_str() {
        "point" => Vector4::new(xv, yv, zv, 1.0),
        "vector" => Vector4::new(xv, yv, zv, 0.0),
        "color" => Vector4::new(xv, yv, zv, 0.0),
        _ => panic!("Unknown constructor: {}", type_name),
    };
    world.tuples.insert(name, tuple);
}

#[then(regex = r"^(\w+) = tuple\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn check_tuple_equality(world: &mut TupleWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple = world.tuples.get(&name).expect("Tuple not found");
    assert_relative_eq!(tuple, &Vector4::new(x, y, z, w));
}

#[then(expr = "{word} - {word} = {word}\\({float}, {float}, {float}\\)")]
fn check_subtraction(world: &mut TupleWorld, a: String, b: String, type_name: String, x: f64, y: f64, z: f64) {
    let t1 = world.tuples.get(&a).expect("Tuple not found");
    let t2 = world.tuples.get(&b).expect("Tuple not found");
    let result = t1 - t2;
    let expected = match type_name.as_str() {
        "point" => Vector4::new(x, y, z, 1.0),
        "vector" => Vector4::new(x, y, z, 0.0),
        _ => panic!("Unknown type"),
    };
    assert_relative_eq!(result, expected);
}

#[then(regex = r"^-(\w+) = tuple\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn check_negation(world: &mut TupleWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = -t;
    let expected = Vector4::new(x, y, z, w);
    assert_relative_eq!(result, expected);
}

#[then(expr = "{word} * {float} = tuple\\({float}, {float}, {float}, {float}\\)")]
fn check_scalar_multiplication(world: &mut TupleWorld, name: String, scalar: f64, x: f64, y: f64, z: f64, w: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = t * scalar;
    let expected = Vector4::new(x, y, z, w);
    assert_relative_eq!(result, expected);
}

#[then(expr = "{word} \\/ {float} = tuple\\({float}, {float}, {float}, {float}\\)")]
fn check_scalar_division(world: &mut TupleWorld, name: String, scalar: f64, x: f64, y: f64, z: f64, w: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = t / scalar;
    let expected = Vector4::new(x, y, z, w);
    assert_relative_eq!(result, expected);
}

#[then(expr = "magnitude\\({word}\\) = {float}")]
fn check_magnitude(world: &mut TupleWorld, name: String, expected: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    assert_relative_eq!(t.norm(), expected);
}

#[then(expr = "magnitude\\({word}\\) = √{float}")]
fn check_magnitude_sqrt(world: &mut TupleWorld, name: String, radicand: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    assert_relative_eq!(t.norm(), radicand.sqrt());
}

#[then(expr = "normalize\\({word}\\) = {word}\\({float}, {float}, {float}\\)")]
#[then(expr = "normalize\\({word}\\) = approximately {word}\\({float}, {float}, {float}\\)")]
fn check_normalization(world: &mut TupleWorld, name: String, type_name: String, x: f64, y: f64, z: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = t.normalize();
    let expected = match type_name.as_str() {
        "point" => Vector4::new(x, y, z, 1.0),
        "vector" => Vector4::new(x, y, z, 0.0),
        _ => panic!("Unknown type"),
    };
    assert_relative_eq!(result, expected, epsilon = 1e-3);
}

#[when(expr = "{word} ← normalize\\({word}\\)")]
fn assign_normalized(world: &mut TupleWorld, name: String, source: String) {
    let t = world.tuples.get(&source).expect("Tuple not found");
    world.tuples.insert(name, t.normalize());
}

#[then(expr = "dot\\({word}, {word}\\) = {float}")]
fn check_dot_product(world: &mut TupleWorld, a: String, b: String, expected: f64) {
    let t1 = world.tuples.get(&a).expect("Tuple not found");
    let t2 = world.tuples.get(&b).expect("Tuple not found");
    assert_relative_eq!(t1.dot(t2), expected);
}

#[then(expr = "cross\\({word}, {word}\\) = vector\\({float}, {float}, {float}\\)")]
fn check_cross_product(world: &mut TupleWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let t1 = world.tuples.get(&a).expect("Tuple not found");
    let t2 = world.tuples.get(&b).expect("Tuple not found");
    
    let v1 = Vector3::new(t1.x, t1.y, t1.z);
    let v2 = Vector3::new(t2.x, t2.y, t2.z);
    let cross = v1.cross(&v2);
    let result = Vector4::new(cross.x, cross.y, cross.z, 0.0);

    let expected = Vector4::new(x, y, z, 0.0);
    assert_relative_eq!(result, expected);
}

#[then(expr = "{word} * {word} = color\\({float}, {float}, {float}\\)")]
fn check_color_multiplication(world: &mut TupleWorld, a: String, b: String, r: f64, g: f64, bl: f64) {
    let t1 = world.tuples.get(&a).expect("Tuple not found");
    let t2 = world.tuples.get(&b).expect("Tuple not found");
    let result = t1.component_mul(t2);
    let expected = Vector4::new(r, g, bl, 0.0);
    assert_relative_eq!(result, expected);
}

#[then(regex = r"^(\w+) = (?:vector|color)\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn check_vector_result(world: &mut TupleWorld, name: String, x: f64, y: f64, z: f64) {
    let tuple = world.tuples.get(&name).expect("Tuple not found");
    let expected = Vector4::new(x, y, z, 0.0);
    assert_relative_eq!(tuple, &expected, epsilon = 1e-4);
}

mod common;
use common::math_parser::parse_expr;


#[when(expr = "{word} ← reflect\\({word}, {word}\\)")]
fn when_reflect(world: &mut TupleWorld, r_name: String, v_name: String, n_name: String) {
    let v = world.tuples.get(&v_name).expect("v not found");
    let n = world.tuples.get(&n_name).expect("n not found");
    let v3 = Vector3::new(v.x, v.y, v.z);
    let n3 = Vector3::new(n.x, n.y, n.z);
    let r3 = ray_tracer_tdd::phong_shader::PhongShader::reflect(v3, n3);
    world.tuples.insert(r_name, Vector4::new(r3.x, r3.y, r3.z, 0.0));
}



fn main() {
    futures::executor::block_on(TupleWorld::run(
        "tests/features/tuples.feature")
    );
}