use approx::assert_relative_eq;
use cucumber::{World, given, when, then};
use std::collections::HashMap;
use nalgebra::Vector3;
use ray_tracer_tdd::light::Light;

#[derive(Debug, Default, World)]
pub struct LightsWorld {
    pub colors: HashMap<String, Vector3<f64>>,
    pub points: HashMap<String, Vector3<f64>>,
    pub lights: HashMap<String, Light>,
}

#[given(expr = "{word} ← color\\({float}, {float}, {float}\\)")]
fn given_color(world: &mut LightsWorld, name: String, r: f64, g: f64, b: f64) {
    world.colors.insert(name, Vector3::new(r, g, b));
}

#[given(expr = "{word} ← point\\({float}, {float}, {float}\\)")]
fn given_point(world: &mut LightsWorld, name: String, x: f64, y: f64, z: f64) {
    world.points.insert(name, Vector3::new(x, y, z));
}

#[when(expr = "{word} ← point_light\\({word}, {word}\\)")]
fn when_point_light(world: &mut LightsWorld, light_name: String, pos_name: String, intensity_name: String) {
    let pos = *world.points.get(&pos_name).expect("position not found");
    let intensity = *world.colors.get(&intensity_name).expect("intensity color not found");
    let light = Light::new(pos, intensity);
    world.lights.insert(light_name, light);
}

#[then(expr = "{word}.position = {word}")]
fn then_position_equals(world: &mut LightsWorld, light_name: String, pos_name: String) {
    let light = world.lights.get(&light_name).expect("light not found");
    let pos = world.points.get(&pos_name).expect("position not found");
    assert_relative_eq!(light.position, pos);
}

#[then(expr = "{word}.intensity = {word}")]
fn then_intensity_equals(world: &mut LightsWorld, light_name: String, intensity_name: String) {
    let light = world.lights.get(&light_name).expect("light not found");
    let intensity = world.colors.get(&intensity_name).expect("intensity color not found");
    assert_relative_eq!(light.intensity, intensity);
}

fn main() {
    futures::executor::block_on(LightsWorld::run(
        "tests/features/lights.feature"
    ));
}
