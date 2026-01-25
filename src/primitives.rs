use std::ops::{Add, Div, Mul, Neg, Sub};

// Float comparison constant
const EPSILON: f64 = 0.00001;

// Checks if two floats are equal
pub fn float_are_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

#[derive(Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn subtract(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    pub fn negate(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    pub fn scalar_mult(self, scalar: f64) -> Tuple {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        let mag = self.magnitude();
        Tuple {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    pub fn dot(self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(self, other: Tuple) -> Tuple {
        vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn hadamard_product(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.add(other)
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.subtract(other)
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        self.negate()
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        self.scalar_mult(scalar)
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        self.scalar_mult(1.0 / scalar)
    }
}

// Checks if two tuples are equal
pub fn tuple_are_equal(a: Tuple, b: Tuple) -> bool {
    float_are_equal(a.x, b.x)
        && float_are_equal(a.y, b.y)
        && float_are_equal(a.z, b.z)
        && float_are_equal(a.w, b.w)
}

// Creates a point with specified coordinates
pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

// Creates a vector with specified coordinates
pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

// Creates a color with specified red, green, and blue components
pub fn color(r: f64, g: f64, b: f64) -> Tuple {
    Tuple { x: r, y: g, z: b, w: 0.0 }
}
