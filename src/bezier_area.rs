use crate::models::{Quad, Cubic};

#[derive(Debug, Clone)]
pub struct QuadEq(f64, f64, f64);

impl QuadEq {
    pub fn from_abc(a: f64, b: f64, c: f64) -> QuadEq {
        QuadEq(a + b - 2.0 * c, 2.0 * (c - a), a) // TODO a + b - 2.0 * c が 0 のとき?
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

    pub fn y2xd(&self, y: f64) -> f64 {
        if self.0 == 0.0 {
            1.0 / self.1
        } else {
            1.0 / (4.0 * self.0 * (y - self.2) + self.1.powi(2)).sqrt()
        }
    }

    pub fn y2x2d(&self, y: f64) -> f64 {
        if self.0 == 0.0 {
            1.0 / self.1
        } else {
            -1.0 / (4.0 * self.0 * (y - self.2) + self.1.powi(2)).sqrt()
        }
    }

    pub fn x2y(&self, x: f64) -> f64 {
        (self.0 * x + self.1) * x + self.2
    }

    pub fn x2yd(&self, x: f64) -> f64 {
        2.0 * self.0 * x + self.1
    }

    pub fn x2yi(&self, x: f64) -> f64 {
        ((self.0 / 3.0 * x + self.1 / 2.0) * x + self.2) * x
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
        let Quad {start, end, control1} = quad;
        //dbg!((start.0 < end.0, start.1 < end.1));
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
            return (lower - upper) * right; // daijoubu?
        }
        let y = self.qey.x2y(right_t);
        // dbg!(upper_t, lower_t, right_t, y);
        if self.rising {
            let upper_t = self.qey.y2x2(upper);
            let lower_t = self.qey.y2x2(lower);

            if y <= upper {
                integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, upper_t)
            } else if lower <= y {
                (lower - upper) * right
            } else {
                //let a = (lower - y) * (self.qex.x2y(lower_t) + right) / 2.0;
                let b = integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, right_t);
                //dbg!(upper, right, a, b, (a - b).abs());
                b + (y - upper) * right
            }
        } else {
            let upper_t = self.qey.y2x(upper);
            let lower_t = self.qey.y2x(lower);
            if y <= upper {
                (lower - upper) * right
            } else if lower <= y {
                integral(&self.qex, &self.qey, lower_t) - integral(&self.qex, &self.qey, upper_t)
            } else {
                //let a = (y - upper) * (self.qex.x2y(upper_t) + right) / 2.0;
                let b = integral(&self.qex, &self.qey, right_t) - integral(&self.qex, &self.qey, upper_t);
                //dbg!(upper, right, a, b, (a - b).abs());
                b + (lower - y) * right
            }
        }
    }
}

fn integral(x: &QuadEq, y: &QuadEq, t: f64) -> f64 {
    let a = x.0 * y.0 / 2.0;
    let b = (2.0 * x.1 * y.0 + x.0 * y.1) / 3.0;
    let c = (2.0 * x.2 * y.0 + x.1 * y.1) / 2.0;
    let d = x.2 * y.1;
    (((a * t + b) * t + c) * t + d) * t
}

pub fn separate_quad(quad: &Quad) -> Vec<Quad> {//dbg!(quad);
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
    //if n == 0 {panic!()}
    let Quad {start, end, control1} = quad;
    let qex = QuadEq::from_abc(start.0, end.0, control1.0);
    let qey = QuadEq::from_abc(start.1, end.1, control1.1);
    //dbg!((qex.top_x(), qey.top_x()));
    if control1.1 < start.1.min(end.1) || start.1.max(end.1) < control1.1 {
        //dbg!(qey.top_x());
        let (q1, q2) = quad.separate(qey.top_x());
        let mut v = separate_quad(&clip_y(q1));
        v.extend(separate_quad(&clip_y(q2)));
        v
    } else if control1.0 < start.0.min(end.0) || start.0.max(end.0) < control1.0 {
        //dbg!(qex.top_x());
        let (q1, q2) = quad.separate(qex.top_x());
        let mut v = separate_quad(&clip_x(q1));
        v.extend(separate_quad(&clip_y(q2)));
        v
    } else {
        vec![quad.clone()]
    }
}

#[test]
fn test() {
    use crate::point::Point;

    // let q = Quad{
    //     start: Point(0.0, 0.0),
    //     end: Point(0.0, 10.0),
    //     control1: Point(5.0, 6.0),
    // };
    let qex = QuadEq(2.0, -2.0, 3.0);
    assert_eq!(qex.top_x(), 0.5);
    assert_eq!(qex.x2y(qex.top_x()), 2.5);
    assert_eq!(qex.y2x(qex.x2y(qex.top_x())), 0.5);

    // dbg!(separate_quad(&Quad{
    //     start: Point(-3.0, -3.0),
    //     end: Point(7.0, -2.0),
    //     control1: Point(6.0, 2.0),
    // }, 5));
    dbg!(separate_quad(&Quad{
        start: Point(-3.0, -3.0),
        end: Point(7.0, -2.0),
        control1: Point(10.0, -6.0),
    }));

    let q = Quad {
        start: Point(250.0, 10.0),
        end: Point(270.0, 30.0),
        control1: Point(270.0, 10.0),
    };
    let qp = QuadPart::from_quad(&q);
    // let y = 30.0;
    // dbg!(qp.qey.y2x(y));
    // dbg!(qp.qey.x2y(qp.qey.y2x(y)));
    // dbg!(qp.qex.x2y(qp.qey.y2x(y)));
    // let y = 29.0;
    // dbg!(qp.qey.y2x(y));
    // dbg!(qp.qey.x2y(qp.qey.y2x(y)));
    // dbg!(qp.qex.x2y(qp.qey.y2x(y)));
    // let y = 28.0;
    // dbg!(qp.qey.y2x(y));
    // dbg!(qp.qey.x2y(qp.qey.y2x(y)));
    // dbg!(qp.qex.x2y(qp.qey.y2x(y)));
    dbg!(qp.area(10., 30.0, 269.));
    //dbg!(qp.qex.integral(1.0));
}
