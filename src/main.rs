use ray_tracer_tdd::canvas::Canvas;
use ray_tracer_tdd::primitives;

fn main() {
    let mut canvas = Canvas::new(50, 50);

    for y in 0..50 {
        for x in 0..50 {
            let r = x as f64 / 50.0;
            let g = y as f64 / 50.0;
            let b = 0.2;
            let color = primitives::color(r, g, b);
            canvas.write_pixel(x, y, color);
        }
    }

    canvas.save_to_file("output.ppm").expect("Failed to save PPM file");
}
