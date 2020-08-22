use crate::point::Point;
use std::f64::consts::FRAC_PI_2;

#[derive(Debug, Clone)]
pub struct Line(pub Point, pub Point);

#[derive(Debug, Clone)]
pub struct Arc {
    pub center: Point,
    pub radius: f64,
    pub angle1: f64,
    pub angle2: f64,
}

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub center: Point,
    pub radius_x: f64,
    pub radius_y: f64,
    pub rotation: f64,
    pub angle1: f64,
    pub angle2: f64,
}

impl Ellipse {
    // pub fn normalize(&self) -> Ellipse {
    //     let rotation = self.rotation.rem_euclid(PI);
    //     if FRAC_PI_2 < rotation {
    //         Ellipse {
    //             ..self.clone(),
    //             rotation,
    //         }
    //     } else {
    //         Ellipse {
    //             ..self.clone(),
    //             rotation: rotation % FRAC_PI_2,
    //             radius_x: self.radius_y
    //         }
    //     }
    // }

    pub fn bound(&self) -> (f64, f64, f64, f64) {
        let ux = self.radius_x * self.rotation.cos();
        let uy = self.radius_x * self.rotation.sin();
        let vx = self.radius_y * (self.rotation + FRAC_PI_2).cos();
        let vy = self.radius_y * (self.rotation + FRAC_PI_2).sin();
        let dx = ux.hypot(vx);
        let dy = uy.hypot(vy);
        (self.center.0 - dx, self.center.0 + dx, self.center.1 - dy, self.center.1 + dy)
    }

    pub fn pos(&self, angle: f64) -> Point {
        self.center + Point(self.radius_x * angle.cos(), self.radius_y * angle.sin()).rotate(self.rotation)
    }

    pub fn angle_offset(&self) -> f64 {
        -((self.rotation - FRAC_PI_2).tan() * self.radius_y / self.radius_x).atan()
    }
}

#[derive(Debug, Clone)]
pub struct Quad {
    pub start: Point,
    pub end: Point,
    pub control1: Point,
}

impl Quad {
    pub fn bound(&self) -> (f64, f64, f64, f64) {
        let x = (self.end.1 - self.control1.1) / (self.start.1 + self.end.1 - 2.0 * self.control1.1);
        let y = (self.end.0 - self.control1.0) / (self.start.0 + self.end.0 - 2.0 * self.control1.0);
        let (x_min, x_max) = if 0.0 <= y && y <= 1.0 {
            (
                self.start.0.min(self.end.0).min(self.pos(y).0),
                self.start.0.max(self.end.0).max(self.pos(y).0),
            )
        } else {
            (
                self.start.0.min(self.end.0),
                self.start.0.max(self.end.0),
            )
        };
        let (y_min, y_max) = if 0.0 <= x && x <= 1.0 {
            (
                self.start.1.min(self.end.1).min(self.pos(x).1),
                self.start.1.max(self.end.1).max(self.pos(x).1),
            )
        } else {
            (
                self.start.1.min(self.end.1),
                self.start.1.max(self.end.1),
            )
        };
        (x_min, x_max, y_min, y_max)
    }

    pub fn pos(&self, t: f64) -> Point {
        let it = 1.0 - t;
        self.start * it.powi(2) + self.control1 * t * it * 2.0 + self.end * t.powi(2)
    }

    pub fn separate(&self, t: f64) -> (Quad, Quad) {
        debug_assert!(0.0 <= t && t <= 1.0);
        let middle = self.pos(t);
        (
            Quad {
                start: self.start,
                end: middle,
                control1: (self.control1 - self.start) * t + self.start,
            },
            Quad {
                start: middle,
                end: self.end,
                control1: (self.control1 - self.end) * (1.0 - t) + self.end,
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct Cubic {
    pub start: Point,
    pub end: Point,
    pub control1: Point,
    pub control2: Point,
}

#[test]
fn test() {
    let q = Quad {start: Point(0.0, 0.0), end: Point(10.0, 0.0), control1: Point(10.0, 10.0)};
    dbg!(q.separate(0.5));
    // assert_eq!(Quad {start: Point(0.0, 0.0), end: Point(10.0, 0.0), control1: Point(-2.0, 10.0)}.bound(), (0.0, 0.0, 0.0, 0.0));
    // assert_eq!(Quad {start: Point(0.0, 0.0), end: Point(10.0, 0.0), control1: Point(5.0, 5.0)}.pos(0.5), Point(0.0, 0.0));
}
