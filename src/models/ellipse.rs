use crate::point::Point;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[derive(Debug, Clone)]
pub struct Ellipse {
    pub center: Point,
    pub radius_x: f32,
    pub radius_y: f32,
    pub rotation: f32,
    pub angle1: f32,
    pub angle2: f32,
}

impl Ellipse {
    pub fn normalize(&self) -> Ellipse {
        // rotationを0degから180degに正規化
        let mut rotation = self.rotation.rem_euclid(TAU);
        let mut angle1 = self.angle1;
        let mut angle2 = self.angle2;
        if PI < rotation {
            angle1 -= PI;
            angle2 -= PI;
            rotation -= PI;
        }
        if angle1 < angle2 {
            angle2 = angle2 + angle1.rem_euclid(TAU) - angle1;
            angle1 = angle1.rem_euclid(TAU);
        } else {
            angle1 = angle1 + angle2.rem_euclid(TAU) - angle2;
            angle2 = angle2.rem_euclid(TAU);
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
        let mut rotation = self.rotation.rem_euclid(TAU);
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
            angle2 = angle2 + angle1.rem_euclid(TAU) - angle1;
            angle1 = angle1.rem_euclid(TAU);
        } else {
            angle1 = angle1 + angle2.rem_euclid(TAU) - angle2;
            angle2 = angle2.rem_euclid(TAU);
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

    pub fn bound(&self) -> (f32, f32, f32, f32) {
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

    pub fn pos(&self, angle: f32) -> Point {
        self.center
            + Point::from((self.radius_x * angle.cos(), self.radius_y * angle.sin()))
                .rotate(self.rotation)
    }

    pub fn angle_offset(&self) -> f32 {
        -((self.rotation - FRAC_PI_2).tan() * self.radius_y / self.radius_x).atan()
    }

    pub fn angle_norm(&self) -> (f32, f32) {
        let (a1, a2) = if self.angle1 < self.angle2 {
            (self.angle1, self.angle2)
        } else {
            (self.angle2, self.angle1)
        };
        let a = a1.rem_euclid(TAU);
        (a, if a2 - a < 0.0 { a2 + TAU } else { a2 })
    }
}
