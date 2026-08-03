use nalgebra::{Vector3, Vector4};
use ray_tracer_tdd::{
    canvas::Canvas, light::Light, material::Material, matrix::Matrix, phong_shader::PhongShader,
    ray::Ray, sphere::Sphere,
};
use std::env;

fn save_image(canvas: Canvas){
    canvas
        .save_to_file("output.ppm")
        .expect("Failed to save PPM file");
    canvas
        .save_to_png("output.png")
        .expect("Failed to save PNG file");
}

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

    save_image(canvas);
}

struct RenderConfig {
    pub pixel_count: usize,
    pub canvas_size: f64,
    pub canvas_center_z: f64,
    pub ray_origin: Vector3<f64>,
    pub spheres: Vec<std::rc::Rc<Sphere>>,
}

fn render_pixels<F>(config: &RenderConfig, shader: F) -> Canvas
where
    F: Fn(&Ray, &ray_tracer_tdd::intersection::Intersection) -> Vector4<f64>,
{
    let canvas_half = config.canvas_size / 2.0;
    let pixel_size = config.canvas_size / config.pixel_count as f64;
    let color_black = Vector4::new(0.0, 0.0, 0.0, 0.0);
    let mut canvas = Canvas::new(config.pixel_count, config.pixel_count);

    for y in 0..config.pixel_count {
        for x in 0..config.pixel_count {
            let canvas_ray_intersection_x = x as f64 * pixel_size - canvas_half;
            let canvas_ray_intersection_y = -(y as f64) * pixel_size + canvas_half;
            let target_point = Vector3::new(
                canvas_ray_intersection_x,
                canvas_ray_intersection_y,
                config.canvas_center_z,
            );
            let ray_direction = (target_point - config.ray_origin).normalize();
            let ray = Ray::new(config.ray_origin, ray_direction);

            let mut all_intersections = ray_tracer_tdd::intersection::Intersections::new();
            for sphere in &config.spheres {
                if let Some(xs) = ray.intersect_sphere(sphere.clone()) {
                    for inter in xs.all() {
                        all_intersections.add(inter.clone());
                    }
                }
            }

            let color = match all_intersections.hit() {
                Some(hit) => shader(&ray, hit),
                None => color_black,
            };

            canvas.write_pixel(x, y, color);
        }
    }

    canvas
}

// Ray traces a sphere and shows its outline
fn run_chapter5_final() {
    let sphere_center = Vector3::new(0.0, 0.0, 5.0);
    let sphere_scale = 4.0;
    let sphere_translation = Matrix::identity().translate(sphere_center);
    let sphere_scaling = Matrix::identity().scale(Vector3::new(
        sphere_scale,
        sphere_scale,
        sphere_scale,
    ));
    let sphere = std::rc::Rc::new(Sphere::with_transform(
        "s1",
        sphere_translation * sphere_scaling,
    ));

    let config = RenderConfig {
        pixel_count: 500,
        canvas_size: 10.0,
        canvas_center_z: 2.0,
        ray_origin: Vector3::new(0.0, 0.0, 0.0),
        spheres: vec![sphere],
    };

    let color_red = Vector4::new(1.0, 0.0, 0.0, 0.0);
    let canvas = render_pixels(&config, |_ray, _hit| color_red);
    save_image(canvas);
}

// simple sphere phong model
fn run_chapter6_final() {
    // Front magenta horizontally-deformed sphere
    let mut sphere_front = Sphere::with_transform("s1", Matrix::identity().scale(Vector3::new(1.0, 0.5, 1.0)));
    sphere_front.material = Material::new();
    sphere_front.material.color = Vector3::new(1.0, 0.2, 1.0);
    let sphere_front_rc = std::rc::Rc::new(sphere_front);

    // Back blue sphere (placed further along Z, larger, matte/less shiny)
    let back_transform = Matrix::identity()
        .translate(Vector3::new(0.0, 0.0, 10.0))
        .scale(Vector3::new(3.5, 3.5, 3.5));
    let mut sphere_back = Sphere::with_transform("s2", back_transform);
    sphere_back.material = Material::new();
    sphere_back.material.color = Vector3::new(0.2, 0.4, 1.0); // Blue
    sphere_back.material.specular = 0.1; // Less shiny
    sphere_back.material.shininess = 20.0;
    let sphere_back_rc = std::rc::Rc::new(sphere_back);

    let light_position = Vector3::new(-10.0, 10.0, -10.0);
    let light_color = Vector3::new(1.0, 1.0, 1.0);
    let light = Light::new(light_position, light_color);

    let config = RenderConfig {
        pixel_count: 500,
        canvas_size: 10.0,
        canvas_center_z: 10.0,
        ray_origin: Vector3::new(0.0, 0.0, -5.0),
        spheres: vec![sphere_front_rc, sphere_back_rc],
    };

    let canvas = render_pixels(&config, |ray, hit| {
        let point = Ray::position(*ray, hit.t);
        let normal = hit.object.normal_at(point);
        let eye = -ray.direction();
        let lit_color = PhongShader::lighting(
            &hit.object.material,
            &light,
            point,
            eye,
            normal,
        );
        Vector4::new(lit_color.x, lit_color.y, lit_color.z, 0.0)
    });

    save_image(canvas);
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
        "6" => run_chapter6_final(),
        other => {
            eprintln!("Unknown chapter number: '{other}'");
            eprintln!("Available chapters: 2, 5");
            std::process::exit(1);
        }
    }
}
