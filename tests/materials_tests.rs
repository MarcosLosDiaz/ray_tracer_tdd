use approx::assert_relative_eq;
use cucumber::{World, given, when, then};
use std::collections::HashMap;
use nalgebra::Vector3;
use ray_tracer_tdd::material::Material;
use ray_tracer_tdd::light::Light;
use ray_tracer_tdd::phong_shader::PhongShader;

#[derive(Debug, Default, World)]
pub struct MaterialsWorld {
    pub materials: HashMap<String, Material>,
    pub vectors: HashMap<String, Vector3<f64>>,
    pub points: HashMap<String, Vector3<f64>>,
    pub colors: HashMap<String, Vector3<f64>>,
    pub lights: HashMap<String, Light>,
}

mod common;
use common::math_parser::parse_expr;

#[given(expr = "{word} ← material\\(\\)" )]
fn given_material(world: &mut MaterialsWorld, name: String) {
    world.materials.insert(name, Material::new());
}

#[given(expr = "{word} ← point\\({float}, {float}, {float}\\)")]
fn given_point(world: &mut MaterialsWorld, name: String, x: f64, y: f64, z: f64) {
    world.points.insert(name, Vector3::new(x, y, z));
}

#[given(expr = "{word} ← vector\\({word}, {word}, {word}\\)")]
fn given_vector_str(world: &mut MaterialsWorld, name: String, x: String, y: String, z: String) {
    let v = Vector3::new(parse_expr(&x), parse_expr(&y), parse_expr(&z));
    world.vectors.insert(name, v);
}

#[given(expr = "{word} ← point_light\\(point\\({float}, {float}, {float}\\), color\\({float}, {float}, {float}\\)\\)")]
fn given_point_light_inline(world: &mut MaterialsWorld, name: String, px: f64, py: f64, pz: f64, cr: f64, cg: f64, cb: f64) {
    let pos = Vector3::new(px, py, pz);
    let intensity = Vector3::new(cr, cg, cb);
    world.lights.insert(name, Light::new(pos, intensity));
}

#[when(expr = "{word} ← lighting\\({word}, {word}, {word}, {word}, {word}\\)")]
fn when_lighting(world: &mut MaterialsWorld, res_name: String, m_name: String, light_name: String, pos_name: String, eye_name: String, norm_name: String) {
    let m = world.materials.get(&m_name).expect("material not found");
    let light = world.lights.get(&light_name).expect("light not found");
    let pos = *world.points.get(&pos_name).expect("position not found");
    let eyev = *world.vectors.get(&eye_name).expect("eyev not found");
    let normalv = *world.vectors.get(&norm_name).expect("normalv not found");

    let result = PhongShader::lighting(m, light, pos, eyev, normalv);
    world.colors.insert(res_name, result);
}

#[then(expr = "{word}.color = color\\({float}, {float}, {float}\\)")]
fn then_color_equals(world: &mut MaterialsWorld, name: String, r: f64, g: f64, b: f64) {
    let m = world.materials.get(&name).expect("material not found");
    assert_relative_eq!(m.color, Vector3::new(r, g, b));
}

#[then(expr = "{word}.ambient = {float}")]
fn then_ambient_equals(world: &mut MaterialsWorld, name: String, expected: f64) {
    let m = world.materials.get(&name).expect("material not found");
    assert_relative_eq!(m.ambient, expected);
}

#[then(expr = "{word}.diffuse = {float}")]
fn then_diffuse_equals(world: &mut MaterialsWorld, name: String, expected: f64) {
    let m = world.materials.get(&name).expect("material not found");
    assert_relative_eq!(m.diffuse, expected);
}

#[then(expr = "{word}.specular = {float}")]
fn then_specular_equals(world: &mut MaterialsWorld, name: String, expected: f64) {
    let m = world.materials.get(&name).expect("material not found");
    assert_relative_eq!(m.specular, expected);
}

#[then(expr = "{word}.shininess = {float}")]
fn then_shininess_equals(world: &mut MaterialsWorld, name: String, expected: f64) {
    let m = world.materials.get(&name).expect("material not found");
    assert_relative_eq!(m.shininess, expected);
}

// Note: Rust proc-macro attributes evaluate before macro expansion or const evaluation,
// so string constants (e.g. FLOAT_REGEX) or concat!() macros cannot be used directly in #[then(regex = ...)].
#[then(regex = r"^(\w+) = color\((-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?), (-?\d+(?:\.\d+)?)\)$")]
fn then_result_color_equals(world: &mut MaterialsWorld, name: String, r: f64, g: f64, b: f64) {
    let c = world.colors.get(&name).expect("color result not found");
    assert_relative_eq!(c.x, r, epsilon = 1e-4);
    assert_relative_eq!(c.y, g, epsilon = 1e-4);
    assert_relative_eq!(c.z, b, epsilon = 1e-4);
}

fn main() {
    futures::executor::block_on(MaterialsWorld::run(
        "tests/features/materials.feature"
    ));
}

