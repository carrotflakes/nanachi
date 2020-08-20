use crate::geometry;
use crate::models::{Arc, Ellipse};
use crate::path2::PathEdge;
use crate::point::Point;
use crate::bezier_area::QuadPart;
use crate::position_color::PositionColor;
use image::{ImageBuffer, Pixel};
use std::f64::consts::{FRAC_PI_2, PI};

#[derive(Debug, Clone)]
pub struct SkewEllipse {
    center: Point,
    radius_x: f64,
    radius_y: f64,
    dx_dy: f64,
    half_width: f64,
}

#[derive(Debug, Clone)]
enum Elm {
    Line(Point, Point),
    LeftArc {
        arc: Arc,
    },
    RightArc {
        arc: Arc,
    },
    LeftEllipse(SkewEllipse),
    RightEllipse(SkewEllipse),
    Quad(QuadPart, f64, f64),
}

#[derive(Debug, Clone)]
struct ElmContainer {
    bound: (f64, f64),
    elm: Elm,
    signum: f64,
}

fn path_edges_to_elms(es: &Vec<PathEdge>) -> Vec<ElmContainer> {
    let mut elms = Vec::new();
    for e in es {
        match e {
            PathEdge::Line(p1, p2) => {
                elms.push(ElmContainer {
                    bound: (p1.1.min(p2.1), p1.1.max(p2.1)),
                    elm: Elm::Line(*p1, *p2), signum: (p1.1 - p2.1).signum()
                });
            }
            PathEdge::Arc(arc) => {
                fn left_arc(arc: &Arc, upper: f64, lower: f64) -> ElmContainer {
                    ElmContainer {
                        bound: (arc.center.1 + upper * arc.radius, arc.center.1 + lower * arc.radius),
                        elm: Elm::LeftArc{arc: arc.clone()}, signum: (arc.angle1 - arc.angle2).signum()
                    }
                }
                fn right_arc(arc: &Arc, upper: f64, lower: f64) -> ElmContainer {
                    ElmContainer {
                        bound: (arc.center.1 + upper * arc.radius, arc.center.1 + lower * arc.radius),
                        elm: Elm::RightArc{arc: arc.clone()}, signum: (arc.angle2 - arc.angle1).signum()
                    }
                }
                let (a1, a2) = angle_norm(arc.angle1, arc.angle2);
                match (((a1 / FRAC_PI_2) as usize + 1) / 2, ((a2 / FRAC_PI_2) as usize + 1) / 2) {
                    (0, 0) | (2, 2) => {
                        elms.push(right_arc(arc, -a2.sin(), -a1.sin()));
                    }
                    (0, 1) | (2, 3) => {
                        elms.push(left_arc(arc, -1.0, -a2.sin()));
                        elms.push(right_arc(arc, -1.0, -a1.sin()));
                    }
                    (0, 2) | (2, 4) => {
                        elms.push(left_arc(arc, -1.0, 1.0));
                        elms.push(right_arc(arc, -1.0, -a1.sin()));
                        elms.push(right_arc(arc, -a2.sin(), 1.0));
                    }
                    (1, 1) => {
                        elms.push(left_arc(arc, -a1.sin(), -a2.sin()));
                    }
                    (1, 2) => {
                        elms.push(left_arc(arc, -a1.sin(), 1.0));
                        elms.push(right_arc(arc, -a2.sin(), 1.0));
                    }
                    (1, 3) => {
                        elms.push(left_arc(arc, -1.0, -a2.sin()));
                        elms.push(left_arc(arc, -a1.sin(), 1.0));
                        elms.push(right_arc(arc, -1.0, 1.0));
                    }
                    _ => unreachable!(),
                }
            }
            PathEdge::Ellipse(ellipse) => {
                fn left_ellipse(ellipse: &Ellipse, upper: f64, lower: f64) -> ElmContainer {
                    ElmContainer {
                        bound: (upper, lower),
                        elm: Elm::LeftEllipse(ellipse.clone().into()),
                        signum: (ellipse.angle1 - ellipse.angle2).signum()
                    }
                }
                fn right_ellipse(ellipse: &Ellipse, upper: f64, lower: f64) -> ElmContainer {
                    ElmContainer {
                        bound: (upper, lower),
                        elm: Elm::RightEllipse(ellipse.clone().into()),
                        signum: (ellipse.angle2 - ellipse.angle1).signum()
                    }
                }
                let bound = ellipse.bound();
                let (a1, a2) = angle_norm(ellipse.angle1, ellipse.angle2);
                // let aa = (ellipse.rotation.tan() * ellipse.radius_x / ellipse.radius_y).atan();
                let aa = ellipse.angle_offset() - FRAC_PI_2;
                dbg!(&aa);
                dbg!((((a1 - aa) / FRAC_PI_2), ((a2 - aa) / FRAC_PI_2)));
                match ((((a1 - aa) / FRAC_PI_2) as usize + 1) / 2, (((a2 - aa) / FRAC_PI_2) as usize + 1) / 2) {
                    (0, 0) | (2, 2) => {
                        elms.push(right_ellipse(ellipse, ellipse.pos(-a2).1, ellipse.pos(-a1).1));
                    }
                    (0, 1) | (2, 3) => {
                        elms.push(left_ellipse(ellipse, bound.2, ellipse.pos(-a2).1));
                        elms.push(right_ellipse(ellipse, bound.2, ellipse.pos(-a1).1));
                    }
                    (0, 2) | (2, 4) => {
                        elms.push(left_ellipse(ellipse, bound.2, bound.3));
                        elms.push(right_ellipse(ellipse, bound.2, ellipse.pos(-a1).1));
                        elms.push(right_ellipse(ellipse, ellipse.pos(-a2).1, bound.3));
                    }
                    (1, 1) => {
                        elms.push(left_ellipse(ellipse, ellipse.pos(-a1).1, ellipse.pos(-a2).1));
                    }
                    (1, 2) => {
                        elms.push(left_ellipse(ellipse, ellipse.pos(-a1).1, bound.3));
                        elms.push(right_ellipse(ellipse, ellipse.pos(-a2).1, bound.3));
                    }
                    (1, 3) => {
                        elms.push(left_ellipse(ellipse, bound.2, ellipse.pos(-a2).1));
                        elms.push(left_ellipse(ellipse, ellipse.pos(-a1).1, bound.3));
                        elms.push(right_ellipse(ellipse, bound.2, bound.3));
                    }
                    _ => unreachable!(),
                }
            }
            PathEdge::Quad(quad) => {
                elms.extend(crate::bezier_area::separate_quad(quad).into_iter().map(|q| {
                    dbg!(&q);
                    //dbg!(crate::bezier_area::QuadPart::from_quad(&q).area(10.0, 30.0, 10.0));
                    let bound = q.bound();
                    dbg!(bound);
                    ElmContainer {
                        bound: (bound.2, bound.3),
                        elm: Elm::Quad(crate::bezier_area::QuadPart::from_quad(&q), bound.0, bound.1),
                        signum: (q.start.1 - q.end.1).signum(),
                    }
                }));
            }
        }
    }
    elms.into_iter().filter(|e| e.bound.0 < e.bound.1).collect()
}

pub fn draw_fill<F: FnMut(u32, u32, f64)>(
    width: u32,
    height: u32,
    edges: &Vec<PathEdge>,
    writer: &mut F,
) {
    let ecs = path_edges_to_elms(edges);
    for y in 0..height as i32 {
        let mut acc = 0.0;
        for x in 0..width as i32 {
            let a = ecs.iter().map(|e|
                e.area(y as f64, (y + 1) as f64, (x + 1) as f64)
            ).sum();
            let v = a - acc;
            acc = a;
            writer(x as u32, y as u32, v);
        }
    }
}

impl ElmContainer {
    fn area(&self, upper: f64, lower: f64, x: f64) -> f64 {
        let upper = upper.max(self.bound.0);
        let lower = lower.min(self.bound.1);
        if lower <= upper {
            return 0.0;
        }
        self.elm.area(upper, lower, x) * self.signum
    }
}

impl Elm {
    fn area(&self, upper: f64, lower: f64, x: f64) -> f64 {
        match self {
            Elm::Line(p1, p2) => {
                segment_area(*p1, *p2, upper, lower, x)
            }
            Elm::LeftArc { ref arc } => {
                let h = lower - upper;
                if x < arc.center.0 - arc.radius {
                    x * h
                } else {
                    let x = x.min(arc.center.0);
                    x * h - circle_area(arc.center, arc.radius, upper, lower, x)
                }
            }
            Elm::RightArc { ref arc } => {
                let h = lower - upper;
                if x < arc.center.0 {
                    x * h
                } else if x < arc.center.0 + arc.radius {
                    arc.center.0 * h + circle_area(arc.center, arc.radius, upper, lower, x) - circle_area(arc.center, arc.radius, upper, lower, arc.center.0)
                } else {
                    arc.center.0 * h + circle_area(arc.center, arc.radius, upper, lower, arc.center.0)
                }
            }
            Elm::LeftEllipse(se) => {
                let h = lower - upper;
                if x < se.center.0 - se.half_width {
                    x * h
                } else {
                    let upper_right = x - (upper - se.center.1) * se.dx_dy;
                    let lower_right = x - (lower - se.center.1) * se.dx_dy;
                    //(lower_right + upper_right) / 2.0
                    x * h - skewed_half_circle_area(se.center, se.radius_x, se.radius_y, upper, lower, upper_right, lower_right)
                }
            }
            Elm::RightEllipse(se) => {
                let h = lower - upper;
                if x < se.center.0 - se.half_width {
                    x * h
                } else {
                    let upper_right = x - (upper - se.center.1) * se.dx_dy;
                    let lower_right = x - (lower - se.center.1) * se.dx_dy;
                    let upper_x = se.center.0 + (upper - se.center.1) * se.dx_dy;
                    let lower_x = se.center.0 + (lower - se.center.1) * se.dx_dy;
                    match (x < upper_x, x < lower_x) {
                        (true, true) => x * h,
                        (false, false) => (upper_x + lower_x) / 2.0 * h + right_skewed_half_circle_area(se.center, se.radius_x, se.radius_y, upper, lower, upper_right, lower_right),
                        (true, false) => (x - (x - lower_x).powi(2) / (upper_x - lower_x) / 2.0) * h + right_skewed_half_circle_area(se.center, se.radius_x, se.radius_y, upper, lower, upper_right, lower_right),
                        (false, true) => (x - (x - upper_x).powi(2) / (lower_x - upper_x) / 2.0) * h + right_skewed_half_circle_area(se.center, se.radius_x, se.radius_y, upper, lower, upper_right, lower_right),
                        //(_, _) => (upper_x + lower_x) / 2.0 * h + right_skewed_half_circle_area(se.center, se.radius_x, se.radius_y, upper, lower, upper_right, lower_right),
                    }
                    //((x.min(upper_x) + x.min(lower_x)) / 2.0) * h // FIXME
                    //+ right_skewed_half_circle_area(se.center, se.radius_x, se.radius_y, upper, lower, upper_right, lower_right)
                }
            }
            Elm::Quad(q, x1, x2) => {
                if x <= *x1 {
                    (lower - upper) * x
                } else {
                    q.area(upper, lower, x.min(*x2))
                }
            }
        }
    }
}

fn half_circle_area(center: Point, radius: f64, upper: f64, lower: f64, right: f64) -> f64 {
    let x = (center.0 - right) / radius;
    (
        ((1.0 - ((upper - center.1) / radius).powi(2)).sqrt() - x).max(0.0) +
        ((1.0 - (((upper * 2.0 + lower) / 3.0 - center.1) / radius).powi(2)).sqrt() - x).max(0.0) +
        ((1.0 - (((upper + lower * 2.0) / 3.0 - center.1) / radius).powi(2)).sqrt() - x).max(0.0) +
        ((1.0 - ((lower - center.1) / radius).powi(2)).sqrt() - x).max(0.0)
    ) / 4.0 * radius * (lower - upper)
}

fn skewed_half_circle_area(center: Point, radius_x: f64, radius_y: f64, upper: f64, lower: f64, upper_right: f64, lower_right: f64) -> f64 {
    radius_y * radius_x * skewed_half_unit_circle_area(
        ((upper - center.1) / radius_y).max(-1.0),
        ((lower - center.1) / radius_y).min(1.0),
        (upper_right - center.0) / radius_x,
        (lower_right - center.0) / radius_x,
    )
}

fn skewed_half_unit_circle_area(upper: f64, lower: f64, upper_right: f64, lower_right: f64) -> f64 {
    let upper_x = -(1.0 - upper.powi(2)).sqrt();
    let lower_x = -(1.0 - lower.powi(2)).sqrt();
    match (upper_x < upper_right, lower_x < lower_right) {
        (true, true) => {
            let d = (lower_x * upper - lower * upper_x).abs() / (lower - upper).hypot(lower_x - upper_x);
            let s1 = d.acos() - (1.0 - d * d).sqrt() * d;
            (s1 + ((upper_right - upper_x) + (lower_right - lower_x)) * (lower - upper) / 2.0).max(0.0)
        }
        (false, false) => {
            let d = (lower_right * upper - lower * upper_right).abs() / (lower - upper).hypot(lower_right - upper_right);
            if d < 1.0 {
                d.acos() - (1.0 - d * d).sqrt() * d
            } else {
                0.0
            };0.
        }
        (true, false) => geometry::circle_2segment_area(
            Point(upper_right, upper),
            Point(lower_right, lower), // p1とp2逆じゃないの？
            Point(upper_x, upper),
        ),
        (false, true) => geometry::circle_2segment_area(
            Point(lower_right, lower),
            Point(lower_x, lower), // p1とp2逆じゃないの？
            Point(upper_right, upper),
        ),
    }
}

fn right_skewed_half_circle_area(center: Point, radius_x: f64, radius_y: f64, upper: f64, lower: f64, upper_right: f64, lower_right: f64) -> f64 {
    radius_y * radius_x * right_skewed_half_unit_circle_area(
        ((upper - center.1) / radius_y).max(-1.0),
        ((lower - center.1) / radius_y).min(1.0),
        (upper_right - center.0) / radius_x,
        (lower_right - center.0) / radius_x,
    )
}


fn right_skewed_half_unit_circle_area(upper: f64, lower: f64, upper_right: f64, lower_right: f64) -> f64 {
    fn f(d: f64) -> f64 {
        d.acos() - (1.0 - d * d).sqrt() * d
    }
    let upper_x = (1.0 - upper.powi(2)).sqrt();
    let lower_x = (1.0 - lower.powi(2)).sqrt();
    match (upper_right < upper_x, lower_right < lower_x) {
        (true, true) => {
            //(upper_right + lower_right).max(0.0) * (lower - upper) / 2.0
            (upper_right + lower_right) * (lower - upper) / 2.0 + match (0. <= upper_right, 0. <= lower_right) {
                (true, true) => 0.0,
                (false, false) => unreachable!(),
                (true, false) => (lower_right).powi(2) / (upper_right - lower_right) / 2.0 * (lower - upper),
                (false, true) => (upper_right).powi(2) / (lower_right - upper_right) / 2.0 * (lower - upper),
            }
        }
        (false, false) => {
            let d = (lower_right * upper - lower * upper_right).abs() / (lower - upper).hypot(lower_right - upper_right);
            if d < 1.0 {
                d.acos() - (1.0 - d * d).sqrt() * d
            } else {
                0.0
            };
            (f(upper) - f(lower)) / 2.0
        }
        (true, false) => {
            (f(upper) - f(lower)) / 2.0 -
            geometry::circle_2segment_area_(
                Point(upper_right, upper),
                Point(upper_x, upper), // p1とp2逆じゃないの？
                Point(lower_right, lower),
            )
        },
        (false, true) => {
            (f(upper) - f(lower)) / 2.0 -
            geometry::circle_2segment_area_(
                Point(lower_right, lower),
                Point(upper_right, upper), // p1とp2逆じゃないの？
                Point(lower_x, lower),
            )
        },
    }
}

fn circle_area(center: Point, radius: f64, upper: f64, lower: f64, right: f64) -> f64 {
    fn f(d: f64) -> f64 {
        d.acos() - (1.0 - d * d).sqrt() * d
    }
    fn g(w: f64, h: f64) -> f64 {
        if w.powi(2) + h.powi(2) >= 1.0 {
            match (0.0 <= w, 0.0 <= h) {
                (true, true) => 0.0,
                (true, false) => f(w),
                (false, true) => f(h),
                (false, false) => -PI + f(w) + f(h),
            }
        } else {
            (-0.5 * PI + f(w) + f(h)) / 2.0 + w * h
        }
    }
    let w = (center.0 - right) / radius;
    let upper = (center.1 - upper) / radius;
    let lower = (center.1 - lower) / radius;

    radius.powi(2) * (g(w, lower) - g(w, upper))
}

fn segment_area(p1: Point, p2: Point, upper: f64, lower: f64, right: f64) -> f64 {
    let y1 = p1.1.min(p2.1).max(upper);
    let y2 = p1.1.max(p2.1).min(lower);
    if y1 < y2 {
        let x1 = geometry::intersect_line_and_horizon(p1, p2, y1);
        let x2 = geometry::intersect_line_and_horizon(p1, p2, y2);
        let (x1, x2) = if x1 < x2 {(x1, x2)} else {(x2, x1)};
        (y2 - y1) * if right < x1 {
            right
        } else if right < x2 {
            right - (right - x1).powi(2) / (x2 - x1) / 2.0
        } else {
            (x1 + x2) / 2.0
        }
    } else {
        0.0
    }
}

fn ellipse_area(center: Point, radius_x: f64, radius_y: f64, rotation: f64, upper: f64, lower: f64, right: f64) -> f64 {
    let x = right - center.0;
    let y1 = upper - center.1;
    let y2 = lower - center.1;
    let (sin, cos) = (-rotation).sin_cos();
    let (x1, y1, x2, y2, x3, y3, x4, y4, x5, y5) = (
        (x * cos - y1 * sin) / radius_x,
        (x * sin + y1 * cos) / radius_y,
        (x * cos - y2 * sin) / radius_x,
        (x * sin + y2 * cos) / radius_y,
        ((x - 100.0) * cos - y1 * sin) / radius_x,
        ((x - 100.0) * sin + y1 * cos) / radius_y,
        ((x - 100.0) * cos - y2 * sin) / radius_x,
        ((x - 100.0) * sin + y2 * cos) / radius_y,
        (x * cos - (y1 - 100.0) * sin) / radius_x,
        (x * sin + (y1 - 100.0) * cos) / radius_y,
    );
    (
        geometry::circle_2segment_area_(Point(x2, -y2), Point(x5, -y5), Point(x4, -y4)) -
        geometry::circle_2segment_area_(Point(x1, -y1), Point(x5, -y5), Point(x3, -y3))
    ) * radius_x * radius_y
}

#[test]
fn test() {
    assert_eq!(
        skewed_half_unit_circle_area(-1.0, -0.9, -0.9, -0.9),
        0.0,
    );
    assert_eq!(
        skewed_half_unit_circle_area(-1.0, 1.0, 0.0, 0.0),
        FRAC_PI_2,
    );
    assert_eq!(
        skewed_half_unit_circle_area(-1.0, 0.0, 0.0, 0.0),
        FRAC_PI_2 / 2.0,
    );
    assert_eq!(
        skewed_half_unit_circle_area(0.0, 1.0, 0.0, 0.0),
        FRAC_PI_2 / 2.0,
    );
    assert_eq!(
        ellipse_area(Point(10.0, 10.0), 3.0, 3.0, 0.0, 9.0, 10.0, 10.0),
        circle_area(Point(10.0, 10.0), 3.0, 9.0, 10.0, 10.0)
    );
    assert_eq!(
        ellipse_area(Point(10.0, 10.0), 3.0, 3.0, 0.0, 9.0, 9.5, 10.0) +
        ellipse_area(Point(10.0, 10.0), 3.0, 3.0, 0.0, 9.5, 10.0, 10.0),
        circle_area(Point(10.0, 10.0), 3.0, 9.0, 10.0, 10.0)
    );
}

fn angle_norm(a1: f64, a2: f64) -> (f64, f64) {
    let (a1, a2) = if a1 < a2 { (a1, a2) } else { (a2, a1) };
    let a = a1.rem_euclid(PI * 2.0);
    (a, if a2 - a < 0.0 { a2 + PI * 2.0 } else { a2 })
}

impl Into<SkewEllipse> for Ellipse {
    fn into(self) -> SkewEllipse {
        // let radius_x = (self.radius_x * self.rotation.cos()).hypot(self.radius_y * (self.rotation + FRAC_PI_2).cos());
        // let rr = (self.radius_y / self.radius_x * (-self.rotation).tan()).atan() + FRAC_PI_2;
        // let radius_y = self.radius_y * rr.sin() * self.rotation.cos() + self.radius_x * rr.cos() * self.rotation.sin();
        let radius_y = (self.radius_x * self.rotation.sin()).hypot(self.radius_y * (self.rotation + FRAC_PI_2).sin());
        let rr = (self.radius_x / self.radius_y * (-self.rotation).tan()).atan();
        let radius_x = self.radius_x * rr.cos() * self.rotation.cos() - self.radius_y * rr.sin() * self.rotation.sin();
        let half_width = (self.radius_x * self.rotation.cos()).hypot(self.radius_y * (self.rotation + FRAC_PI_2).cos());
        
let aa = -(self.radius_y / self.radius_x * (self.rotation + FRAC_PI_2).tan()).atan();
let x = aa.cos() * self.radius_x;
let y = aa.sin() * self.radius_y;
let (sin, cos) = self.rotation.sin_cos();
        let dx_dy = (x * cos - y * sin) / (x * sin + y * cos);
        SkewEllipse{
            center: self.center,
            radius_x,
            radius_y,
            dx_dy,
            half_width,
        }
    }
}
