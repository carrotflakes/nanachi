use crate::point::Point;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AugmentedMatrix(pub [f64; 6]);

impl AugmentedMatrix {
    pub fn new() -> AugmentedMatrix {
        AugmentedMatrix([1.0, 0.0, 0.0, 0.0, 1.0, 0.0])
    }

    pub fn translate(&self, x: f64, y: f64) -> AugmentedMatrix {
        let s = &self.0;
        AugmentedMatrix([s[0], s[1], s[2] + x, s[3], s[4], s[5] + y])
    }

    pub fn scale(&self, x: f64, y: f64) -> AugmentedMatrix {
        let s = &self.0;
        AugmentedMatrix([s[0] * x, s[1] * x, s[2] * x, s[3] * y, s[4] * y, s[5] * y])
    }

    pub fn rotate(&self, rad: f64) -> AugmentedMatrix {
        let s = &self.0;
        let (sin, cos) = rad.sin_cos();
        AugmentedMatrix([
            s[0] * cos - s[3] * sin,
            s[1] * cos - s[4] * sin,
            s[2] * cos - s[5] * sin,
            s[0] * sin + s[3] * cos,
            s[1] * sin + s[4] * cos,
            s[2] * sin + s[5] * cos,
        ])
    }

    pub fn skew_y(&self, dy: f64) -> AugmentedMatrix {
        let s = &self.0;
        AugmentedMatrix([
            s[0],
            s[1],
            s[2],
            s[3] + s[0] * dy,
            s[4] + s[1] * dy,
            s[5] + s[2] * dy,
        ])
    }

    pub fn skew_x(&self, dx: f64) -> AugmentedMatrix {
        let s = &self.0;
        AugmentedMatrix([
            s[0] + s[3] * dx,
            s[1] + s[4] * dx,
            s[2] + s[5] * dx,
            s[3],
            s[4],
            s[5],
        ])
    }

    pub fn apply<P: From<Point> + Into<Point>>(&self, p: P) -> P {
        let p: Point = p.into();
        let s = &self.0;
        Point(
            p.0 * s[0] + p.1 * s[1] + s[2],
            p.0 * s[3] + p.1 * s[4] + s[5],
        )
        .into()
    }

    pub fn inverse(&self) -> AugmentedMatrix {
        let s = &self.0;
        let a = 1.0 / (s[0] * s[4] - s[1] * s[3]);
        AugmentedMatrix([
            a * s[4],
            -a * s[1],
            a * (s[1] * s[5] - s[2] * s[4]),
            -a * s[3],
            a * s[0],
            -a * (s[0] * s[5] - s[2] * s[3]),
        ])
    }

    pub fn then(&self, rhs: &AugmentedMatrix) -> AugmentedMatrix {
        let s = &self.0;
        let t = &rhs.0;
        AugmentedMatrix([
            s[0] * t[0] + s[3] * t[1],
            s[1] * t[0] + s[4] * t[1],
            s[2] * t[0] + s[5] * t[1] + t[2],
            s[0] * t[3] + s[3] * t[4],
            s[1] * t[3] + s[4] * t[4],
            s[2] * t[3] + s[5] * t[4] + t[5],
        ])
    }
}

#[test]
fn test() {
    let am = AugmentedMatrix::new().translate(1.0, 2.0).rotate(1.0).scale(0.5, 0.6);
    assert!((Point(3.0, 4.0) - am.inverse().apply(am.apply(Point(3.0, 4.0)))).norm() < 0.00001);

    assert_eq!(
        am.rotate(0.1).then(&AugmentedMatrix::new().translate(-0.5, -0.6)),
        am.rotate(0.1).translate(-0.5, -0.6));
    assert_eq!(
        am.rotate(0.1).then(&AugmentedMatrix::new().scale(-0.5, -0.6)),
        am.rotate(0.1).scale(-0.5, -0.6));
    assert_eq!(
        am.rotate(0.1).then(&AugmentedMatrix::new().rotate(0.3)),
        am.rotate(0.1).rotate(0.3));
    assert_eq!(
        am.rotate(0.1).then(&AugmentedMatrix::new().scale(0.5, 0.6).translate(-0.5, -0.6).rotate(0.3)),
        am.rotate(0.1).scale(0.5, 0.6).translate(-0.5, -0.6).rotate(0.3));
}
