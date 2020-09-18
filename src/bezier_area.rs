use crate::models::Quad;

#[derive(Debug, Clone)]
pub struct QuadEq(f64, f64, f64);

impl QuadEq {
    pub fn from_abc(a: f64, b: f64, c: f64) -> QuadEq {
        QuadEq(a + b - 2.0 * c, 2.0 * (c - a), a)
    }

    pub fn y2x(&self, y: f64) -> f64 {
        if self.0 == 0.0 {
            (y - self.2) / self.1
        } else {
            (-self.1 + (-4.0 * self.0 * (self.2 - y) + self.1.powi(2)).sqrt()) / (2.0 * self.0)
        }
    }

    pub fn y2x2(&self, y: f64) -> f64 {
        if self.0 == 0.0 {
            (y - self.2) / self.1
        } else {
            (-self.1 - (-4.0 * self.0 * (self.2 - y) + self.1.powi(2)).sqrt()) / (2.0 * self.0)
        }
    }

    pub fn x2y(&self, x: f64) -> f64 {
        (self.0 * x + self.1) * x + self.2
    }

    pub fn top_x(&self) -> f64 {
        -0.5 * self.1 / self.0
    }
}

#[derive(Debug, Clone)]
pub struct QuadPart {
    pub qex: QuadEq,
    pub qey: QuadEq,
    pub rising: bool,
}

impl QuadPart {
    pub fn from_quad(quad: &Quad) -> QuadPart {
        let Quad {
            start,
            end,
            control1,
        } = quad;
        match (start.0 < end.0, start.1 < end.1) {
            (true, true) => QuadPart {
                qex: QuadEq::from_abc(start.0, end.0, control1.0),
                qey: QuadEq::from_abc(start.1, end.1, control1.1),
                rising: false,
            },
            (false, false) => QuadPart {
                qex: QuadEq::from_abc(end.0, start.0, control1.0),
                qey: QuadEq::from_abc(end.1, start.1, control1.1),
                rising: false,
            },
            (true, false) => QuadPart {
                qex: QuadEq::from_abc(start.0, end.0, control1.0),
                qey: QuadEq::from_abc(start.1, end.1, control1.1),
                rising: true,
            },
            (false, true) => QuadPart {
                qex: QuadEq::from_abc(end.0, start.0, control1.0),
                qey: QuadEq::from_abc(end.1, start.1, control1.1),
                rising: true,
            },
        }
    }

    pub fn area(&self, upper: f64, lower: f64, right: f64) -> f64 {
        let right_t = self.qex.y2x(right);
        if right_t.is_nan() {
            return if 0.0 < self.qex.0 {
                (lower - upper) * right
            } else if self.rising {
                let upper_t = self.qey.y2x2(upper);
                let lower_t = self.qey.y2x2(lower);
                integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, upper_t)
            } else {
                let upper_t = self.qey.y2x(upper);
                let lower_t = self.qey.y2x(lower);
                integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, upper_t)
            };
        }
        let y = self.qey.x2y(right_t);
        if self.rising {
            let upper_t = self.qey.y2x2(upper);
            let lower_t = self.qey.y2x2(lower);
            if y <= upper {
                integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, upper_t)
            } else if lower <= y {
                (lower - upper) * right
            } else {
                integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, right_t)
                    + (y - upper) * right
            }
        } else {
            let upper_t = self.qey.y2x(upper);
            let lower_t = self.qey.y2x(lower);
            if y <= upper {
                (lower - upper) * right
            } else if lower <= y {
                integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, upper_t)
            } else {
                integral(&self.qex, &self.qey, right_t) - integral(&self.qex, &self.qey, upper_t)
                    + (lower - y) * right
            }
        }
    }

    pub fn y2x(&self, y: f64) -> f64 {
        self.qex.x2y(if self.rising {
            self.qey.y2x2(y)
        } else {
            self.qey.y2x(y)
        })
    }
}

fn integral(x: &QuadEq, y: &QuadEq, t: f64) -> f64 {
    let a = x.0 * y.0 / 2.0;
    let b = (2.0 * x.1 * y.0 + x.0 * y.1) / 3.0;
    let c = (2.0 * x.2 * y.0 + x.1 * y.1) / 2.0;
    let d = x.2 * y.1;
    (((a * t + b) * t + c) * t + d) * t
}

pub fn separate_quad(quad: &Quad) -> Vec<Quad> {
    let Quad {
        start,
        end,
        control1,
    } = quad;
    let qex = QuadEq::from_abc(start.0, end.0, control1.0);
    let qey = QuadEq::from_abc(start.1, end.1, control1.1);
    if control1.1 < start.1.min(end.1) || start.1.max(end.1) < control1.1 {
        let (q1, q2) = quad.separate(qey.top_x());
        let mut v = separate_quad(&clip_y(q1));
        v.extend(separate_quad(&clip_y(q2)));
        v
    } else if control1.0 < start.0.min(end.0) || start.0.max(end.0) < control1.0 {
        let (q1, q2) = quad.separate(qex.top_x());
        let mut v = separate_quad(&clip_x(q1));
        v.extend(separate_quad(&clip_x(q2)));
        v
    } else {
        vec![quad.clone()]
    }
}

fn clip_x(mut quad: Quad) -> Quad {
    if quad.control1.0 < quad.start.0.min(quad.end.0) {
        quad.control1.0 = quad.start.0.min(quad.end.0);
    } else if quad.start.0.max(quad.end.0) < quad.control1.0 {
        quad.control1.0 = quad.start.0.max(quad.end.0);
    }
    quad
}

fn clip_y(mut quad: Quad) -> Quad {
    if quad.control1.1 < quad.start.1.min(quad.end.1) {
        quad.control1.1 = quad.start.1.min(quad.end.1);
    } else if quad.start.1.max(quad.end.1) < quad.control1.1 {
        quad.control1.1 = quad.start.1.max(quad.end.1);
    }
    quad
}
