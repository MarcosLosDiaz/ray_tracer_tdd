use cucumber::{World, given, then, when};
use std::collections::HashMap;
use ray_tracer_tdd::primitives;

#[derive(Debug, Default, World)]
pub struct TupleWorld {
    pub tuples: HashMap<String, primitives::Tuple>,
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float}\\)")]
fn given_tuple(world: &mut TupleWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.tuples.insert(name, primitives::Tuple { x, y, z, w });
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
    assert!(primitives::float_are_equal(actual, expected));
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
    assert_ne!(w, expected);
}

#[given(expr = "{word} ← {word}\\({float}, {float}, {float}\\)")]
fn assign_point_or_vector(world: &mut TupleWorld, name: String, type_name: String, x: f64, y: f64, z: f64) {
    let tuple = match type_name.as_str() {
        "point" => primitives::point(x, y, z),
        "vector" => primitives::vector(x, y, z),
        "color" => primitives::color(x, y, z),
        _ => panic!("Unknown constructor: {}", type_name),
    };
    world.tuples.insert(name, tuple);
}

// we cant use {word} because that will match -{word} as well
#[then(regex = r"^(\w+) = tuple\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn check_tuple_equality(world: &mut TupleWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let tuple = world.tuples.get(&name).expect("Tuple not found");
    assert!(primitives::tuple_are_equal(*tuple, primitives::Tuple { x, y, z, w }));
}

#[then(expr = "{word} - {word} = {word}\\({float}, {float}, {float}\\)")]
fn check_subtraction(world: &mut TupleWorld, a: String, b: String, type_name: String, x: f64, y: f64, z: f64) {
    let t1 = world.tuples.get(&a).expect("Tuple not found");
    let t2 = world.tuples.get(&b).expect("Tuple not found");
    let result = t1.subtract(*t2);
    let expected = match type_name.as_str() {
        "point" => primitives::point(x, y, z),
        "vector" => primitives::vector(x, y, z),
        _ => panic!("Unknown type"),
    };
    assert!(primitives::tuple_are_equal(result, expected));
}

#[then(regex = r"^-(\w+) = tuple\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn check_negation(world: &mut TupleWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = t.negate();
    let expected = primitives::Tuple { x, y, z, w };
    assert!(primitives::tuple_are_equal(result, expected));
}

#[then(expr = "{word} * {float} = tuple\\({float}, {float}, {float}, {float}\\)")]
fn check_scalar_multiplication(world: &mut TupleWorld, name: String, scalar: f64, x: f64, y: f64, z: f64, w: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = t.scalar_mult(scalar);
    let expected = primitives::Tuple { x, y, z, w };
    assert!(primitives::tuple_are_equal(result, expected));
}

#[then(expr = "{word} \\/ {float} = tuple\\({float}, {float}, {float}, {float}\\)")]
fn check_scalar_division(world: &mut TupleWorld, name: String, scalar: f64, x: f64, y: f64, z: f64, w: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = t.scalar_mult(1.0 / scalar);
    let expected = primitives::Tuple { x, y, z, w };
    assert!(primitives::tuple_are_equal(result, expected));
}

#[then(expr = "magnitude\\({word}\\) = {float}")]
fn check_magnitude(world: &mut TupleWorld, name: String, expected: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    assert!(primitives::float_are_equal(t.magnitude(), expected));
}

#[then(expr = "magnitude\\({word}\\) = √{float}")]
fn check_magnitude_sqrt(world: &mut TupleWorld, name: String, radicand: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    assert!(primitives::float_are_equal(t.magnitude(), radicand.sqrt()));
}

#[then(expr = "normalize\\({word}\\) = {word}\\({float}, {float}, {float}\\)")]
#[then(expr = "normalize\\({word}\\) = approximately {word}\\({float}, {float}, {float}\\)")]
fn check_normalization(world: &mut TupleWorld, name: String, type_name: String, x: f64, y: f64, z: f64) {
    let t = world.tuples.get(&name).expect("Tuple not found");
    let result = t.normalize();
    let expected = match type_name.as_str() {
        "point" => primitives::point(x, y, z),
        "vector" => primitives::vector(x, y, z),
        _ => panic!("Unknown type"),
    };
    assert!(primitives::tuple_are_equal(result, expected));
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
    assert!(primitives::float_are_equal(t1.dot(*t2), expected));
}

#[then(expr = "cross\\({word}, {word}\\) = vector\\({float}, {float}, {float}\\)")]
fn check_cross_product(world: &mut TupleWorld, a: String, b: String, x: f64, y: f64, z: f64) {
    let t1 = world.tuples.get(&a).expect("Tuple not found");
    let t2 = world.tuples.get(&b).expect("Tuple not found");
    let result = t1.cross(*t2);
    let expected = primitives::vector(x, y, z);
    assert!(primitives::tuple_are_equal(result, expected));
}

#[then(expr = "{word} * {word} = color\\({float}, {float}, {float}\\)")]
fn check_color_multiplication(world: &mut TupleWorld, a: String, b: String, r: f64, g: f64, bl: f64) {
    let t1 = world.tuples.get(&a).expect("Tuple not found");
    let t2 = world.tuples.get(&b).expect("Tuple not found");
    let result = t1.hadamard_product(*t2);
    let expected = primitives::color(r, g, bl);
    assert!(primitives::tuple_are_equal(result, expected));
}

fn main() {
    futures::executor::block_on(TupleWorld::run(
        "tests/features/tuples.feature")
    );
}