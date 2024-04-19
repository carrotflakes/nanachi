use crate::point::Point;
use std::f32::consts::TAU;

#[derive(Debug, Clone)]
pub struct Arc {
    pub center: Point,
    pub radius: f32,
    pub angle1: f32,
    pub angle2: f32,
}

impl Arc {
    pub fn from_points(center: Point, start: Point, end: Point) -> Arc {
        let angle1 = (start - center).atan2().rem_euclid(TAU);
        let mut angle2 = (end - center).atan2().rem_euclid(TAU);
        if angle1 > angle2 {
            angle2 += TAU
        }
        Arc {
            center,
            radius: (start - center).norm(),
            angle1,
            angle2,
        }
    }

    pub fn angle_norm(&self) -> [f32; 2] {
        let (a1, a2) = if self.angle1 < self.angle2 {
            (self.angle1, self.angle2)
        } else {
            (self.angle2, self.angle1)
        };
        let a = a1.rem_euclid(TAU);
        [a, if a2 - a < 0.0 { a2 + TAU } else { a2 }]
    }
}
