use crate::point::Point;
use std::f64::consts::{FRAC_PI_2, PI};

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
        (
            self.center.x() - dx,
            self.center.x() + dx,
            self.center.y() - dy,
            self.center.y() + dy,
        )
    }

    pub fn pos(&self, angle: f64) -> Point {
        self.center
            + Point::from((self.radius_x * angle.cos(), self.radius_y * angle.sin()))
                .rotate(self.rotation)
    }

    pub fn angle_offset(&self) -> f64 {
        -((self.rotation - FRAC_PI_2).tan() * self.radius_y / self.radius_x).atan()
    }

    pub fn angle_norm(&self) -> (f64, f64) {
        let (a1, a2) = if self.angle1 < self.angle2 {
            (self.angle1, self.angle2)
        } else {
            (self.angle2, self.angle1)
        };
        let a = a1.rem_euclid(PI * 2.0);
        (a, if a2 - a < 0.0 { a2 + PI * 2.0 } else { a2 })
    }
}
