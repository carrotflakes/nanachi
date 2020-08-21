use crate::models::{Quad, Cubic};
use crate::point::Point;

// pub fn y2x(quad: &Quad, y: f64) -> f64 {
//     let Quad {
//         start,
//         end,
//         control1
//     } = quad;
//     let (a, b, c) = (start.1 + end.1 - 2.0 * control1.1, 2.0 * (control1.1 - end.1), end.1);
//     let v = if a == 0.0 {
//         (y - c) / b
//     } else {
//         let d = (-4.0 * a * (y - c) + b.powi(2)).sqrt();
//         let r = (-b + d) / (2.0 * a);
//         if 0.0 <= r && r <= 1.0 {
//             r
//         } else {
//             (-b - d) / (2.0 * a)
//         }
//     };
//     (start.0 + end.0 - 2.0 * control1.0) * v.powi(2) + 2.0 * (control1.0 - end.0) * v + end.0
// }

// pub fn x2y(quad: &Quad, x: f64) -> f64 {
//     let Quad {
//         start,
//         end,
//         control1
//     } = quad;
//     let (a, b, c) = (start.0 + end.0 - 2.0 * control1.0, 2.0 * (control1.0 - end.0), end.0);
//     let v = if a == 0.0 {
//         (x - c) / b
//     } else {
//         let d = (-4.0 * a * (x - c) + b.powi(2)).sqrt();
//         let r = (-b + d) / (2.0 * a);
//         if 0.0 <= r && r <= 1.0 {
//             r
//         } else {
//             (-b - d) / (2.0 * a)
//         }
//     };
//     (start.1 + end.1 - 2.0 * control1.1) * v.powi(2) + 2.0 * (control1.1 - end.1) * v + end.1
// }

// pub fn x2v1(b: f64, c: f64, x: f64) -> f64 {
//     (c - x) / b
// }
// pub fn x2v2(a: f64, b: f64, c: f64, x: f64) -> f64 {
//     (-b + (-4.0 * a * (c - x) + b.powi(2)).sqrt()) / (2.0 * a)
// }
// pub fn x2v3(a: f64, b: f64, c: f64, x: f64) -> f64 {
//     (-b - (-4.0 * a * (c - x) + b.powi(2)).sqrt()) / (2.0 * a)
// }

// pub fn x2t(p: (f64, f64, f64), x: f64) -> f64 {
//     (-p.1 + (-4.0 * p.0 * (p.2 - x) + p.1.powi(2)).sqrt()) / (2.0 * p.0)
// }

// pub fn zero_y(quad: &Quad) -> f64 {
//     (quad.end.0 - quad.control1.0) / (quad.start.0 + quad.end.0 - 2.0 * quad.control1.0)
// }

// pub fn zero_x(quad: &Quad) -> f64 {
//     (quad.end.1 - quad.control1.1) / (quad.start.1 + quad.end.1 - 2.0 * quad.control1.1)
// }

// pub fn t2x(p: (f64, f64, f64), t: f64) -> f64 {
//     ((p.0 + p.1 - 2.0 * p.2) * t + 2.0 * (p.2 - p.0)) * t + p.0
// }

// pub fn integral(p: (f64, f64, f64), t: f64) -> f64 {
//     ((((p.0 + p.1 - 2.0 * p.2) / 3.0 * t + (p.2 - p.0)) * t) + p.0) * t
// }

// pub struct QuadArea {
//     top_left: Option<()>,
//     top_right: Option<()>,
//     bottom_left: Option<()>,
//     bottom_right: Option<()>,
//     sep_x: f64,
//     sep_y: f64,
// }

// impl QuadArea {
//     pub fn area(&self, upper: f64, lower: f64, right: f64) -> f64 {
//         if let Some(()) = self.top_left {

//         }
//     }
// }

// pub fn quad_area(quad: &Quad, range: (f64, f64), upper: f64, lower: f64, right: f64) -> f64 {
//     let upper_x = y2x(quad, upper);
//     let lower_x = y2x(quad, lower);
//     integral(quad.start.1, quad.end.1, quad.control1.1, lower_x) - integral(quad.start.1, quad.end.1, quad.control1.1, upper_x)
//     // if (!upper_x.is_nan(), !lower_x.is_nan()) {
//     //     // (true, true) => {
            
//     //     // }
//     //     (false, false) => {
            
//     //     }
//     // }
// }

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

    pub fn x2y(&self, x: f64) -> f64 {
        (self.0 * x + self.1) * x + self.2
    }

    pub fn integral(&self, x: f64) -> f64 {
        (((self.0 / 3.0 * x + self.1 / 2.0) * x) + self.2) * x
    }

    pub fn top_x(&self) -> f64 {
        -0.5 * self.1 / self.0
    }
}//(a * x + b) * x + c
// (-b + sqrt(-4 * a * (c - y) + b^2)) / (2 * a)

pub fn zero_x_t(a: f64, b: f64) -> f64 {
    -0.5 * b / a
}

#[derive(Debug, Clone)]
pub struct QuadPart {
    qex: QuadEq,
    qey: QuadEq,
    rising: bool,
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
        let y = self.qey.x2y(right_t);
        // dbg!(upper_t);
        // dbg!(lower_t);
        // dbg!(right_t);
        // dbg!(y);
        if self.rising {
            let upper_t = self.qey.y2x2(upper);
            let lower_t = self.qey.y2x2(lower);
            // if self.qex.x2y(lower_t) < right {
            //     (lower - upper) * right
            // } else {
            //     (self.qex.integral(lower_t) - self.qex.integral(right_t)) * (lower - y) / (lower_t - right_t) // approximate
            //     + (y - upper) * right
            // }
            
            if y <= upper {
                (lower - upper) * (self.qex.x2y(upper_t) + self.qex.x2y(lower_t)) / 2.0
            } else if lower <= y {
                (lower - upper) * right
            } else {
                if !((self.qex.x2y(lower_t) - right).abs() < 40.0) {
                    dbg!((self.qex.x2y(upper_t), self.qex.x2y(lower_t), right));
                    panic!();
                }
                (lower - y) * (self.qex.x2y(lower_t) + right) / 2.0
                + (y - upper) * right
            }
        } else {
            let upper_t = self.qey.y2x(upper);
            let lower_t = self.qey.y2x(lower);
            // dbg!(self.qex.integral(right_t));
            // dbg!(self.qex.integral(upper_t));
            // dbg!(self.qex.integral(right_t) - self.qex.integral(upper_t));
            // dbg!((lower - y) * right);
            //if right <= self.qex.x2y(upper_t) {
            if y <= upper {
                (lower - upper) * right
            //if self.qex.x2y(lower_t) <= right {
            } else if lower <= y {
                (lower - upper) * (self.qex.x2y(upper_t) + self.qex.x2y(lower_t)) / 2.0
            } else {
                //(self.qex.integral(right_t) - self.qex.integral(upper_t)) * (y - upper) / (right_t - upper_t) // approximate
                (y - upper) * (self.qex.x2y(upper_t) + right) / 2.0
                + (lower - y) * right
            }
            //(lower - upper) * right.min(self.qex.x2y(upper_t))
        }
    }
}

pub fn separate_quad(quad: &Quad) -> Vec<Quad> {//dbg!(quad);
    //if n == 0 {panic!()}
    let Quad {start, end, control1} = quad;
    let qex = QuadEq::from_abc(start.0, end.0, control1.0);
    let qey = QuadEq::from_abc(start.1, end.1, control1.1);
    //dbg!((qex.top_x(), qey.top_x()));
    if control1.1 < start.1.min(end.1) {
        //dbg!(qey.top_x());
        let (q1, q2) = quad.separate(qey.top_x());
        let mut v = separate_quad(&q1);
        v.extend(separate_quad(&q2));
        v
    } else if start.1.max(end.1) < control1.1 {
        //dbg!(qey.top_x());
        let (q1, q2) = quad.separate(qey.top_x());
        let mut v = separate_quad(&q1);
        v.extend(separate_quad(&q2));
        v
    } else if control1.0 < start.0.min(end.0) {
        //dbg!(qex.top_x());
        let (q1, q2) = quad.separate(qex.top_x());
        let mut v = separate_quad(&q1);
        v.extend(separate_quad(&q2));
        v
    } else if start.0.max(end.0) < control1.0 {
        //dbg!(qex.top_x());
        let (q1, q2) = quad.separate(qex.top_x());
        let mut v = separate_quad(&q1);
        v.extend(separate_quad(&q2));
        v
    } else {
        vec![quad.clone()]
    }
}

// pub fn quad_area(quad: &Quad, upper: f64, lower: f64, right: f64) -> f64 {
//     let Quad {
//         start,
//         end,
//         control1
//     } = quad;
//     let integral = |x| (start.1 + end.1 - 2.0 * control1.1) / 3.0 * x.powi(3) - (2.0 * control1.1 + 2.0 * end.1) / 2.0 * x.powi(2) + end.1 * x;
//     if control1.1 < start.1.min(end.1) { // Convex upward

//     } else if start.1.max(end.1) < control1.1 { // Convex downward

//     } else {
//         let (a, b, c) = (start.1 + end.1 - 2.0 * control1.1, -2.0 * (control1.1 - end.1), end.1);
//         let f = |y| { // y から x 座標を求める
//             let d = (-4.0 * a * (y - c) + b.powi(2)).sqrt();
//             let r = (-b + d) / 2 * a;
//             let v= if 0.0 <= r && r <= 1.0 {
//                 r
//             } else {
//                 (-b - d) / 2 * a
//             };
//             (start.0 + end.0 - 2.0 * control1.0) * v.powi(2) - 2.0 * (control1.0 - end.0) * v + end.0
//         };
//     }
// }

#[test]
fn test() {
    // let q = Quad{
    //     start: Point(0.0, 0.0),
    //     end: Point(0.0, 10.0),
    //     control1: Point(5.0, 6.0),
    // };
    // dbg!(zero_y(&q));
    // let a = zero_y(&q);
    // dbg!(y2x(&q, a));
    // dbg!((q.pos(a), q.pos(a-0.1), q.pos(a+0.1)));
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
