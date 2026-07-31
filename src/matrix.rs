use nalgebra::{Matrix4, Vector3, Vector4};
use std::ops::Mul;

// PartialEq implements episolon-based element-wise comparison of matrices
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub data: Matrix4<f64>,
}

impl Matrix {
    pub fn new(data: Matrix4<f64>) -> Self {
        Self { data }
    }

    pub fn identity() -> Self {
        Self {
            data: Matrix4::identity(),
        }
    }

    pub fn try_inverse(&self) -> Option<Self> {
        self.data.try_inverse().map(Matrix::new)
    }

    pub fn inverse(&self) -> Self {
        self.try_inverse().expect("Matrix is not invertible")
    }

    /// Returns the multiplication of the current matrix by a translation matrix with x, y, z values.
    ///
    /// Translation Matrix:
    /// | 1 0 0 x |
    /// | 0 1 0 y |
    /// | 0 0 1 z |
    /// | 0 0 0 1 |
    pub fn translate(&self, translation: Vector3<f64>) -> Self {
        let x = translation.x;
        let y = translation.y;
        let z = translation.z;
        let translation_matrix = Self::translation_matrix(x, y, z);

        Self { data: self.data * translation_matrix }
    }

    /// Returns the multiplication of the current matrix by a scaling matrix with x, y, z values.
    ///
    /// Scaling Matrix:
    /// | x 0 0 0 |
    /// | 0 y 0 0 |
    /// | 0 0 z 0 |
    /// | 0 0 0 1 |
    pub fn scale(&self, scaling: Vector3<f64>) -> Self {
        let x = scaling.x;
        let y = scaling.y;
        let z = scaling.z;
        let scaling_matrix = Self::scaling_matrix(x, y, z);
        Self { data: self.data * scaling_matrix }
    }

    /// Returns the multiplication of the current matrix by a rotation matrix around the x-axis.
    ///
    /// Rotation Matrix (right-handed):
    /// | 1    0        0      0 |
    /// | 0  cos(r)  -sin(r)   0 |
    /// | 0  sin(r)   cos(r)   0 |
    /// | 0    0        0      1 |
    pub fn rotate_x(&self, radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();
        let rotation_matrix = Self::rotation_x_matrix(cos_r, sin_r);
        Self { data: self.data * rotation_matrix }
    }

    /// Returns the multiplication of the current matrix by a rotation matrix around the y-axis.
    ///
    /// Rotation Matrix (right-handed):
    /// |  cos(r)   0   sin(r)   0 |
    /// |    0      1     0      0 |
    /// | -sin(r)   0   cos(r)   0 |
    /// |    0      0     0      1 |
    pub fn rotate_y(&self, radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();
        let rotation_matrix = Self::rotation_y_matrix(cos_r, sin_r);
        Self { data: self.data * rotation_matrix }
    }

    /// Returns the multiplication of the current matrix by a rotation matrix around the z-axis.
    ///
    /// Rotation Matrix (right-handed):
    /// | cos(r)  -sin(r)   0   0 |
    /// | sin(r)   cos(r)   0   0 |
    /// |   0        0      1   0 |
    /// |   0        0      0   1 |
    pub fn rotate_z(&self, radians: f64) -> Self {
        let cos_r = radians.cos();
        let sin_r = radians.sin();
        let rotation_matrix = Self::rotation_z_matrix(cos_r, sin_r);
        Self { data: self.data * rotation_matrix }
    }

    /// Returns the multiplication of the current matrix by a shearing matrix.
    ///
    /// Shearing Matrix:
    /// | 1  xy  xz  0 |
    /// | yx  1  yz  0 |
    /// | zx  zy  1  0 |
    /// | 0   0   0  1 |
    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let shearing_matrix = Self::shearing_matrix(xy, xz, yx, yz, zx, zy);
        Self { data: self.data * shearing_matrix }
    }

    // Helper constructors for transformation matrices
    pub fn translation_matrix(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        Matrix4::new(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn scaling_matrix(x: f64, y: f64, z: f64) -> Matrix4<f64> {
        Matrix4::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }

    pub fn rotation_x_matrix(cos_r: f64, sin_r: f64) -> Matrix4<f64>{
        Matrix4::new(
            1.0, 0.0,   0.0,    0.0,
            0.0, cos_r, -sin_r, 0.0,
            0.0, sin_r, cos_r,  0.0,
            0.0, 0.0,   0.0,    1.0,
        )
    }

    pub fn rotation_y_matrix(cos_r: f64, sin_r: f64) -> Matrix4<f64> {
        Matrix4::new(
            cos_r,  0.0, sin_r, 0.0,
            0.0,    1.0, 0.0,   0.0,
            -sin_r, 0.0, cos_r, 0.0,
            0.0,    0.0, 0.0,   1.0,
        )
    }

    pub fn rotation_z_matrix(cos_r: f64, sin_r: f64) -> Matrix4<f64> {
        Matrix4::new(
            cos_r, -sin_r, 0.0, 0.0,
            sin_r, cos_r,  0.0, 0.0,
            0.0,   0.0,    1.0, 0.0,
            0.0,   0.0,    0.0, 1.0,
        )
    }

    pub fn shearing_matrix(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4<f64> {
        Matrix4::new(
            1.0, xy,  xz,  0.0,
            yx,  1.0, yz,  0.0,
            zx,  zy,  1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::new(self.data * rhs.data)
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::new(self.data * rhs.data)
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        Matrix::new(self.data * rhs.data)
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        Matrix::new(self.data * rhs.data)
    }
}

impl Mul<Vector4<f64>> for Matrix {
    type Output = Vector4<f64>;

    fn mul(self, rhs: Vector4<f64>) -> Self::Output {
        self.data * rhs
    }
}

impl Mul<Vector4<f64>> for &Matrix {
    type Output = Vector4<f64>;

    fn mul(self, rhs: Vector4<f64>) -> Self::Output {
        self.data * rhs
    }
}

impl Mul<&Vector4<f64>> for Matrix {
    type Output = Vector4<f64>;

    fn mul(self, rhs: &Vector4<f64>) -> Self::Output {
        self.data * rhs
    }
}

impl Mul<&Vector4<f64>> for &Matrix {
    type Output = Vector4<f64>;

    fn mul(self, rhs: &Vector4<f64>) -> Self::Output {
        self.data * rhs
    }
}