//! [`Point`] represents x, y coordinates.

/// Auxiliary struct representing 2D coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub [f64; 2]);

impl Point {
    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn from_angle(angle: f64) -> Point {
        Point([angle.cos(), angle.sin()])
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y()
    }

    pub fn lerp(self, rhs: Self, v: f64) -> Self {
        self * (1.0 - v) + rhs * v
    }

    pub fn norm(self) -> f64 {
        self.x().hypot(self.y())
    }

    pub fn atan2(self) -> f64 {
        self.y().atan2(self.x())
    }

    pub fn rotate(self, angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Point([
            self.x() * cos - self.y() * sin,
            self.x() * sin + self.y() * cos,
        ])
    }

    pub fn unit(self) -> Point {
        self / self.norm()
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point([self.x() + rhs.x(), self.y() + rhs.y()])
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point([self.x() - rhs.x(), self.y() - rhs.y()])
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Point([self.x() * rhs, self.y() * rhs])
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Point([self.x() / rhs, self.y() / rhs])
    }
}

impl<T: Into<f64>> From<(T, T)> for Point {
    fn from(tuple: (T, T)) -> Point {
        Point([tuple.0.into(), tuple.1.into()])
    }
}

impl From<Point> for (f64, f64) {
    fn from(point: Point) -> (f64, f64) {
        (point.x(), point.y())
    }
}
