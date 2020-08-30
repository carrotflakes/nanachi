#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn from_angle(angle: f64) -> Point {
        Point(angle.cos(), angle.sin())
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }

    pub fn lerp(self, rhs: Self, v: f64) -> Self {
        self * (1.0 - v) + rhs * v
    }

    pub fn norm(self) -> f64 {
        self.0.hypot(self.1)
    }

    pub fn atan2(self) -> f64 {
        self.1.atan2(self.0)
    }

    pub fn rotate(self, angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Point(
            self.0 * cos - self.1 * sin,
            self.0 * sin + self.1 * cos,
        )
    }

    pub fn unit(self) -> Point {
        self / self.0.hypot(self.1)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Point(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Point(self.0 / rhs, self.1 / rhs)
    }
}

impl<T: Into<f64>> From<(T, T)> for Point {
    fn from(tuple: (T, T)) -> Point {
        Point(tuple.0.into(), tuple.1.into())
    }
}

impl<T: From<f64>> Into<(T, T)> for Point {
    fn into(self) -> (T, T) {
        (self.0.into(), self.1.into())
    }
}
