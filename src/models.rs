use crate::point::Point;
use std::f64::consts::FRAC_PI_2;

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
        (self.center.0 - dx, self.center.0 + dy, self.center.1 - dy, self.center.1 + dy)
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

#[derive(Debug, Clone)]
pub struct Cubic {
    pub start: Point,
    pub end: Point,
    pub control1: Point,
    pub control2: Point,
}
