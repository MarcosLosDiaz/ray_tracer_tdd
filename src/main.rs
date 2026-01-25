use ray_tracer_tdd::canvas::Canvas;
use nalgebra::Vector4;

fn main() {
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

    canvas.save_to_file("output.ppm").expect("Failed to save PPM file");
}
