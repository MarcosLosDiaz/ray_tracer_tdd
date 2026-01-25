use nalgebra::{SMatrix, SVector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub data: SMatrix<f64, 4, 4>,
}

impl Matrix {
    pub fn new(data: SMatrix<f64, 4, 4>) -> Self {
        Self { data }
    }

    /// Returns the multiplication of the current matrix by a translation matrix with x, y, z values.
    ///
    /// Translation Matrix:
    /// | 1 0 0 x |
    /// | 0 1 0 y |
    /// | 0 0 1 z |
    /// | 0 0 0 1 |
    pub fn translate(&self, translation: SVector<f64, 3>) -> Self {
        let x = translation.x;
        let y = translation.y;
        let z = translation.z;

        let translation_matrix = SMatrix::<f64, 4, 4>::new(
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        );

        Self {
            data: self.data * translation_matrix,
        }
    }

    /// Returns the multiplication of the current matrix by a scaling matrix with x, y, z values.
    ///
    /// Scaling Matrix:
    /// | x 0 0 0 |
    /// | 0 y 0 0 |
    /// | 0 0 z 0 |
    /// | 0 0 0 1 |
    pub fn scale(&self, scaling: SVector<f64, 3>) -> Self {
        let x = scaling.x;
        let y = scaling.y;
        let z = scaling.z;

        let scaling_matrix = SMatrix::<f64, 4, 4>::new(
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        Self {
            data: self.data * scaling_matrix,
        }
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

        let rotation_matrix = SMatrix::<f64, 4, 4>::new(
            1.0, 0.0,   0.0,    0.0,
            0.0, cos_r, -sin_r, 0.0,
            0.0, sin_r, cos_r,  0.0,
            0.0, 0.0,   0.0,    1.0,
        );

        Self {
            data: self.data * rotation_matrix,
        }
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

        let rotation_matrix = SMatrix::<f64, 4, 4>::new(
            cos_r,  0.0, sin_r, 0.0,
            0.0,    1.0, 0.0,   0.0,
            -sin_r, 0.0, cos_r, 0.0,
            0.0,    0.0, 0.0,   1.0,
        );

        Self {
            data: self.data * rotation_matrix,
        }
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

        let rotation_matrix = SMatrix::<f64, 4, 4>::new(
            cos_r, -sin_r, 0.0, 0.0,
            sin_r, cos_r,  0.0, 0.0,
            0.0,   0.0,    1.0, 0.0,
            0.0,   0.0,    0.0, 1.0,
        );

        Self {
            data: self.data * rotation_matrix,
        }
    }

    /// Returns the multiplication of the current matrix by a shearing matrix.
    ///
    /// Shearing Matrix:
    /// | 1  xy  xz  0 |
    /// | yx  1  yz  0 |
    /// | zx  zy  1  0 |
    /// | 0   0   0  1 |
    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let shearing_matrix = SMatrix::<f64, 4, 4>::new(
            1.0, xy,  xz,  0.0,
            yx,  1.0, yz,  0.0,
            zx,  zy,  1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        );

        Self {
            data: self.data * shearing_matrix,
        }
    }
}