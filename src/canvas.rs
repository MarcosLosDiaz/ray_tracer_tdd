use nalgebra::Vector4;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Box<[Vector4<f64>]>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let black = Vector4::new(0.0, 0.0, 0.0, 0.0);
        let pixels = vec![black; width * height].into_boxed_slice();
        Canvas { width, height, pixels }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Vector4<f64>) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);

        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let index = y * self.width + x;
                let pixel = self.pixels[index];
                
                let r = (pixel.x * 255.0).round().clamp(0.0, 255.0) as u8;
                let g = (pixel.y * 255.0).round().clamp(0.0, 255.0) as u8;
                let b = (pixel.z * 255.0).round().clamp(0.0, 255.0) as u8;

                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(&format!("{} {} {}", r, g, b));
            }
            ppm.push_str(&line);
            ppm.push('\n');
        }

        ppm
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        std::fs::write(filename, self.to_ppm())
    }
}