use crate::point::Point;
use std::f64::consts::{FRAC_PI_2, PI};

#[derive(Debug, Clone)]
pub struct Line(pub Point, pub Point);

#[derive(Debug, Clone)]
pub struct Arc {
    pub center: Point,
    pub radius: f64,
    pub angle1: f64,
    pub angle2: f64,
}

impl Arc {
    pub fn angle_norm(&self) -> (f64, f64) {
        let (a1, a2) = if self.angle1 < self.angle2 { (self.angle1, self.angle2) } else { (self.angle2, self.angle1) };
        let a = a1.rem_euclid(PI * 2.0);
        (a, if a2 - a < 0.0 { a2 + PI * 2.0 } else { a2 })
    }
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
    pub fn normalize(&self) -> Ellipse {
        // rotationを0degから180degに正規化
        let mut rotation = self.rotation.rem_euclid(PI * 2.0);
        let mut angle1 = self.angle1;
        let mut angle2 = self.angle2;
        if PI < rotation {
            angle1 -= PI;
            angle2 -= PI;
            rotation -= PI;
        }
        if angle1 < angle2 {
            angle2 = angle2 + angle1.rem_euclid(PI * 2.0) - angle1;
            angle1 = angle1.rem_euclid(PI * 2.0);
        } else {
            angle1 = angle1 + angle2.rem_euclid(PI * 2.0) - angle2;
            angle2 = angle2.rem_euclid(PI * 2.0);
        }
        Ellipse {
            center: self.center,
            radius_x: self.radius_x,
            radius_y: self.radius_y,
            rotation,
            angle1,
            angle2,
        }
    }

    pub fn normalize_half_pi(&self) -> Ellipse {
        // rotationを0degから90degに正規化
        let mut rotation = self.rotation.rem_euclid(PI * 2.0);
        let mut radius_x = self.radius_x;
        let mut radius_y = self.radius_y;
        let mut angle1 = self.angle1;
        let mut angle2 = self.angle2;
        if PI < rotation {
            angle1 -= PI;
            angle2 -= PI;
            rotation -= PI;
        }
        if FRAC_PI_2 < rotation {
            rotation -= FRAC_PI_2;
            radius_x = self.radius_y;
            radius_y = self.radius_x;
            angle1 += FRAC_PI_2; // TODO
            angle2 += FRAC_PI_2; // TODO
        }
        if angle1 < angle2 {
            angle2 = angle2 + angle1.rem_euclid(PI * 2.0) - angle1;
            angle1 = angle1.rem_euclid(PI * 2.0);
        } else {
            angle1 = angle1 + angle2.rem_euclid(PI * 2.0) - angle2;
            angle2 = angle2.rem_euclid(PI * 2.0);
        }
        Ellipse {
            center: self.center,
            radius_x,
            radius_y,
            rotation,
            angle1,
            angle2,
        }
    }

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

    pub fn angle_norm(&self) -> (f64, f64) {
        let (a1, a2) = if self.angle1 < self.angle2 { (self.angle1, self.angle2) } else { (self.angle2, self.angle1) };
        let a = a1.rem_euclid(PI * 2.0);
        (a, if a2 - a < 0.0 { a2 + PI * 2.0 } else { a2 })
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

    pub fn closest_t_to_control(&self) -> f64 {
        let v0 = self.control1 - self.start;
        let v1 = self.end - self.control1;

        let a = (v1 - v0).dot(&(v1 - v0));
        let b = 3.0 * (v1.dot(&v0) - v0.dot(&v0));
        let c = 3.0 * v0.dot(&v0) - v1.dot(&v0);
        let d = -1.0 * v0.dot(&v0);

        let p = -b / (3.0 * a);
        let q = p.powi(3) + (b * c - 3.0 * a * d) / (6.0 * a.powi(2));
        let r = c / (3.0 * a);

        let s = (q.powi(2) + (r - p.powi(2)).powi(3)).sqrt();
        (q + s).cbrt() + (q - s).cbrt() + p
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
