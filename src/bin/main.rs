use nalgebra::{Vector3, Vector4};
use ray_tracer_tdd::{canvas::Canvas, matrix::Matrix, ray::Ray, sphere::Sphere};
use std::env;

fn run_chapter2_final() {
    let mut canvas = Canvas::new(50, 50);

    for y in 0..50 {
        for x in 0..50 {
            let r = x as f64 / 50.0;
            let g = y as f64 / 50.0;
            let b = 0.2;
            let color = Vector4::new(r, g, b, 0.0);
            canvas.write_pixel(x, y, color);
        }
    }

    canvas
        .save_to_file("output.ppm")
        .expect("Failed to save PPM file");
}

// Ray traces a spehere and shows it's outline
fn run_chapter5_final() {
    let pixel_count = 500;
    let canvas_size = 10.0;
    let canvas_center_z = 2.0;

    let canvas_half = canvas_size / 2.0;
    let pixel_size = canvas_size / pixel_count as f64;

    let color_red = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let color_black = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let mut canvas = Canvas::new(pixel_count, pixel_count);

    let sphere_center = Vector3::new(0.0, 0.0, 5.0);
    let sphere_scale = 4.0;
    let sphere_translation = Matrix::identity().translate(sphere_center);
    let sphere_scaling = Matrix::identity().scale(Vector3::new(
        sphere_scale,
        sphere_scale,
        sphere_scale,
    ));
    // Multiply translation * scaling to scale first, then position at sphere_center
    let sphere = std::rc::Rc::new(Sphere::with_transform(
        "s1",
        sphere_translation * sphere_scaling,
    ));

    for y in 0..pixel_count {
        for x in 0..pixel_count {
            // ray origin at world origin
            let canvas_ray_intersection_x = x as f64 * pixel_size - canvas_half;
            let canvas_ray_intersection_y = -(y as f64) * pixel_size + canvas_half;
            let ray = Ray::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(
                    canvas_ray_intersection_x,
                    canvas_ray_intersection_y,
                    canvas_center_z,
                ),
            );

            let xs = ray.intersect_sphere(sphere.clone());
            let color = match xs.as_ref().and_then(|intersections| intersections.hit()) {
                Some(_hit) => color_red,
                None => color_black,
            };

            canvas.write_pixel(x, y, color);
        }
    }

    canvas
        .save_to_file("output.ppm")
        .expect("Failed to save PPM file");
    canvas
        .save_to_png("output.png")
        .expect("Failed to save PNG file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --bin main -- <chapter_number>");
        eprintln!("Available chapters: 2, 5");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "2" => run_chapter2_final(),
        "5" => run_chapter5_final(),
        other => {
            eprintln!("Unknown chapter number: '{other}'");
            eprintln!("Available chapters: 2, 5");
            std::process::exit(1);
        }
    }
}
