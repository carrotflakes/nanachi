//! Affine transformation

use crate::point::Point;

/// Matrix for affine transformation.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix(pub [f64; 6]);

impl Matrix {
    /// Create a Matrix that no transform.
    pub fn new() -> Matrix {
        Matrix([1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    }

    /// Create new Matrix translated with specified position from myself.
    pub fn translate(&self, x: f64, y: f64) -> Matrix {
        let s = &self.0;
        Matrix([s[0], s[1], s[2] + x, s[3], s[4], s[5] + y])
    }

    /// Create new Matrix scaled with specified size from myself.
    pub fn scale(&self, x: f64, y: f64) -> Matrix {
        let s = &self.0;
        Matrix([s[0] * x, s[1] * x, s[2] * x, s[3] * y, s[4] * y, s[5] * y])
    }

    /// Create new Matrix rotated with specified angle from myself.
    pub fn rotate(&self, rad: f64) -> Matrix {
        let s = &self.0;
        let (sin, cos) = rad.sin_cos();
        Matrix([
            s[0] * cos - s[3] * sin,
            s[1] * cos - s[4] * sin,
            s[2] * cos - s[5] * sin,
            s[0] * sin + s[3] * cos,
            s[1] * sin + s[4] * cos,
            s[2] * sin + s[5] * cos,
        ])
    }

    /// Create new Matrix skewed with specified y-axis amount from myself.
    pub fn skew_y(&self, dy: f64) -> Matrix {
        let s = &self.0;
        Matrix([
            s[0],
            s[1],
            s[2],
            s[3] + s[0] * dy,
            s[4] + s[1] * dy,
            s[5] + s[2] * dy,
        ])
    }

    /// Create new Matrix skewed with specified x-axis amount from myself.
    pub fn skew_x(&self, dx: f64) -> Matrix {
        let s = &self.0;
        Matrix([
            s[0] + s[3] * dx,
            s[1] + s[4] * dx,
            s[2] + s[5] * dx,
            s[3],
            s[4],
            s[5],
        ])
    }

    /// Transform the [`Point`].
    pub fn apply<P: From<Point> + Into<Point>>(&self, p: P) -> P {
        let p: Point = p.into();
        let s = &self.0;
        Point(
            p.0 * s[0] + p.1 * s[1] + s[2],
            p.0 * s[3] + p.1 * s[4] + s[5],
        )
        .into()
    }

    /// Inverse the matrix
    /// Ideally, `matrix.inverse().inverse() == matrix`.
    pub fn inverse(&self) -> Matrix {
        let s = &self.0;
        let a = 1.0 / (s[0] * s[4] - s[1] * s[3]);
        Matrix([
            a * s[4],
            -a * s[1],
            a * (s[1] * s[5] - s[2] * s[4]),
            -a * s[3],
            a * s[0],
            -a * (s[0] * s[5] - s[2] * s[3]),
        ])
    }

    /// Return the multiplication of the two matrices.
    pub fn then(&self, rhs: &Matrix) -> Matrix {
        let s = &self.0;
        let t = &rhs.0;
        Matrix([
            s[0] * t[0] + s[3] * t[1],
            s[1] * t[0] + s[4] * t[1],
            s[2] * t[0] + s[5] * t[1] + t[2],
            s[0] * t[3] + s[3] * t[4],
            s[1] * t[3] + s[4] * t[4],
            s[2] * t[3] + s[5] * t[4] + t[5],
        ])
    }

    /// Return whether it is unit matrix.
    pub fn is_unit(&self) -> bool {
        self == &Default::default()
    }

    /// Return whether it is directly or indirectly.
    /// An indirect matrix makes path flip.
    pub fn is_direct(&self) -> bool {
        self.0[1] * self.0[3] <= self.0[0] * self.0[4]
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::new()
    }
}

#[test]
fn test() {
    let am = Matrix::new()
        .translate(1.0, 2.0)
        .rotate(1.0)
        .scale(0.5, 0.6);
    assert!((Point(3.0, 4.0) - am.inverse().apply(am.apply(Point(3.0, 4.0)))).norm() < 0.00001);

    assert_eq!(
        am.rotate(0.1).then(&Matrix::new().translate(-0.5, -0.6)),
        am.rotate(0.1).translate(-0.5, -0.6)
    );
    assert_eq!(
        am.rotate(0.1).then(&Matrix::new().scale(-0.5, -0.6)),
        am.rotate(0.1).scale(-0.5, -0.6)
    );
    assert_eq!(
        am.rotate(0.1).then(&Matrix::new().rotate(0.3)),
        am.rotate(0.1).rotate(0.3)
    );
    assert!(
        am.rotate(0.1)
            .then(
                &Matrix::new()
                    .scale(0.5, 0.6)
                    .translate(-0.5, -0.6)
                    .rotate(0.3)
            )
            .0
            .iter()
            .zip(
                am.rotate(0.1)
                    .scale(0.5, 0.6)
                    .translate(-0.5, -0.6)
                    .rotate(0.3)
                    .0
                    .iter()
            )
            .map(|(a, b)| (a - b).abs())
            .sum::<f64>()
            < 0.0001
    );
    assert_eq!(Matrix::new().apply((0.0f64, 0.0f64)), (0.0, 0.0));
}
