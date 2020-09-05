use crate::point::Point;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Arc {
    pub center: Point,
    pub radius: f64,
    pub angle1: f64,
    pub angle2: f64,
}

impl Arc {
    pub fn from_points(center: Point, start: Point, end: Point) -> Arc {
        let angle1 = (start - center).atan2().rem_euclid(PI * 2.0);
        let mut angle2 = (end - center).atan2().rem_euclid(PI * 2.0);
        if angle1 > angle2 {
            angle2 += PI * 2.0
        }
        Arc {
            center,
            radius: (start - center).norm(),
            angle1,
            angle2,
        }
    }

    pub fn angle_norm(&self) -> (f64, f64) {
        let (a1, a2) = if self.angle1 < self.angle2 { (self.angle1, self.angle2) } else { (self.angle2, self.angle1) };
        let a = a1.rem_euclid(PI * 2.0);
        (a, if a2 - a < 0.0 { a2 + PI * 2.0 } else { a2 })
    }
}
