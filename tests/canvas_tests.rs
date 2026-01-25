use cucumber::{World, given, when, then};
use ray_tracer_tdd::canvas::Canvas;
use std::collections::HashMap;
use nalgebra::Vector4;

#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvas: Option<Canvas>,
    ppm: String,
    colors: HashMap<String, Vector4<f64>>,
}

#[given(expr = "c ← canvas\\({int}, {int}\\)")]
fn given_canvas(world: &mut CanvasWorld, width: usize, height: usize) {
    world.canvas = Some(Canvas::new(width, height));
}

#[given(expr = "{word} ← color\\({float}, {float}, {float}\\)")]
fn given_color(world: &mut CanvasWorld, name: String, r: f64, g: f64, b: f64) {
    world.colors.insert(name, Vector4::new(r, g, b, 0.0));
}

#[when(expr = "ppm ← canvas_to_ppm\\(c\\)")]
fn canvas_to_ppm(world: &mut CanvasWorld) {
    if let Some(c) = &world.canvas {
        world.ppm = c.to_ppm();
    }
}

#[when(expr = "write_pixel\\(c, {int}, {int}, {word}\\)")]
fn write_pixel(world: &mut CanvasWorld, x: usize, y: usize, color_name: String) {
    if let Some(c) = &mut world.canvas {
        let color = *world.colors.get(&color_name).expect("Color not found");
        c.write_pixel(x, y, color);
    }
}

#[then(expr = "lines {int}-{int} of ppm are")]
fn check_ppm_header(world: &mut CanvasWorld, start: usize, end: usize, step: &cucumber::gherkin::Step) {
    let expected = step.docstring.as_ref().expect("Docstring expected").trim();
    let lines: Vec<&str> = world.ppm.lines().skip(start - 1).take(end - start + 1).collect();
    let actual = lines.join("\n");
    assert_eq!(actual, expected);
}

#[then(expr = "ppm ends with a newline character")]
fn check_ppm_ends_with_newline(world: &mut CanvasWorld) {
    assert!(world.ppm.ends_with('\n'));
}

fn main() {
    futures::executor::block_on(CanvasWorld::run("tests/features/canvas.feature"));
}