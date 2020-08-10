use crate::geometry;
use crate::path2::{PathEdge, Arc};
use crate::point::Point;
use crate::position_color::PositionColor;
use image::{ImageBuffer, Pixel};
use std::f64::consts::{FRAC_PI_2, PI};

#[derive(Debug, Clone)]
enum Elm {
    Line(Point, Point),
    LeftArc {
        arc: Arc,
    },
    RightArc {
        arc: Arc,
    },
}

#[derive(Debug, Clone)]
struct ElmContainer {
    bound: (f64, f64),
    elm: Elm,
    max: f64,
}

fn path_edges_to_elms(es: &Vec<PathEdge>) -> Vec<ElmContainer> {
    let mut elms = Vec::new();
    for e in es {
        match e {
            PathEdge::Line(p1, p2) => {
                elms.push(ElmContainer {
                    bound: (p1.1.min(p2.1), p1.1.max(p2.1)),
                    elm: Elm::Line(*p1, *p2), max: 0.0
                });
            }
            PathEdge::Arc(arc) => {
                let (a1, a2) = angle_norm(arc.angle1, arc.angle2);
                match (((a1 / FRAC_PI_2) as usize + 1) / 2, ((a2 / FRAC_PI_2) as usize + 1) / 2) {
                    (0, 0) | (2, 2) => {
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - a2.sin() * arc.radius, arc.center.1 - a1.sin() * arc.radius),
                            elm: Elm::RightArc{arc: arc.clone()}, max: 0.0
                        });
                    }
                    (0, 1) | (2, 3) => {
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - arc.radius, arc.center.1 - a2.sin() * arc.radius),
                            elm: Elm::LeftArc{arc: arc.clone()}, max: 0.0
                        });
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - arc.radius, arc.center.1 - a1.sin() * arc.radius),
                            elm: Elm::RightArc{arc: arc.clone()}, max: 0.0
                        });
                    }
                    (0, 2) | (2, 4) => {
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - arc.radius, arc.center.1 + arc.radius),
                            elm: Elm::LeftArc{arc: arc.clone()}, max: 0.0
                        });
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - arc.radius, arc.center.1 - a1.sin() * arc.radius),
                            elm: Elm::RightArc{arc: arc.clone()}, max: 0.0
                        });
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - a2.sin() * arc.radius, arc.center.1 + arc.radius),
                            elm: Elm::RightArc{arc: arc.clone()}, max: 0.0
                        });
                    }
                    (1, 1) => {
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - a1.sin() * arc.radius, arc.center.1 - a2.sin() * arc.radius),
                            elm: Elm::LeftArc{arc: arc.clone()}, max: 0.0
                        });
                    }
                    (1, 2) => {
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - a1.sin() * arc.radius, arc.center.1 + arc.radius),
                            elm: Elm::LeftArc{arc: arc.clone()}, max: 0.0
                        });
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - a2.sin() * arc.radius, arc.center.1 + arc.radius),
                            elm: Elm::RightArc{arc: arc.clone()}, max: 0.0
                        });
                    }
                    (1, 3) => {
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - arc.radius, arc.center.1 - a1.sin() * arc.radius),
                            elm: Elm::LeftArc{arc: arc.clone()}, max: 0.0
                        });
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - a2.sin() * arc.radius, arc.center.1 + arc.radius),
                            elm: Elm::LeftArc{arc: arc.clone()}, max: 0.0
                        });
                        elms.push(ElmContainer {
                            bound: (arc.center.1 - arc.radius, arc.center.1 + arc.radius),
                            elm: Elm::RightArc{arc: arc.clone()}, max: 0.0
                        });
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    elms.into_iter().filter(|e| e.bound.0 < e.bound.1).collect()
}

pub fn draw_fill<X, C: PositionColor<X>>(
    img: &mut ImageBuffer<X, Vec<u8>>,
    edges: &Vec<PathEdge>,
    position_color: &C,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    let width = img.width() as usize;
    let mut ecs = path_edges_to_elms(edges);
    for ec in ecs.iter_mut() {
        ec.max = ec.area(img.height() as f64, img.width() as f64);
    }
    let ecslen = ecs.len();
    let ww = width + 2;
    let mut acc = vec![0.0; ecslen * ww];
    for y in 0..img.height() {
        for i in 0..ecslen {
            acc[i * ww + (-(y as i32)).rem_euclid(ww as i32) as usize] = 0.0;
        }
        for x in 0..img.width() as usize {
            let mut r = 0.0;
            for (i, e) in ecs.iter().enumerate() {
                let offset = i * ww;
                let a = e.area((y + 1) as f64, (x + 1) as f64);
                r += a + acc[offset + (x as i32 + 1 - y as i32).rem_euclid(ww as i32) as usize] - acc[offset + (x as i32 + 2 - y as i32).rem_euclid(ww as i32) as usize] - acc[offset + (x as i32 - y as i32).rem_euclid(ww as i32) as usize];
                acc[offset + (x as i32 + 1 - y as i32).rem_euclid(ww as i32) as usize] = a;
                //r += e.area((y + 1) as f64, (x + 1) as f64) - e.area((y + 1) as f64, x as f64) - e.area(y as f64, (x + 1) as f64) + e.area(y as f64, x as f64)
            }
            img_blend_pixel(img, position_color, x as i32, y as i32, r);//.min(1.0).max(0.0)
            //img_blend_pixel(img, position_color, x as i32, y as i32, r.min(1.0).max(0.0));
        }
    }
}

impl ElmContainer {
    fn area(&self, y: f64, x: f64) -> f64 {
        if y <= self.bound.0 {
            return 0.0;
        }
        let y = y.min(self.bound.1);
        match self.elm {
            Elm::Line(p1, p2) => {
                segment_area(p1, p2, y, x)
            }
            Elm::LeftArc { ref arc } => {
                let h = y - self.bound.0;
                (arc.angle1 - arc.angle2).signum() *
                if x < arc.center.0 - arc.radius {
                    x * h
                } else {
                    let x = x.min(arc.center.0);
                    x * h - circle_area(arc.center, arc.radius, y, x) + circle_area(arc.center, arc.radius, self.bound.0, x)
                }
            }
            Elm::RightArc { ref arc } => {
                let h = y - self.bound.0;
                (arc.angle2 - arc.angle1).signum() *
                if x < arc.center.0 {
                    x * h
                } else if x < arc.center.0 + arc.radius {
                    arc.center.0 * h + circle_area(arc.center, arc.radius, y, x) - circle_area(arc.center, arc.radius, y, arc.center.0) - circle_area(arc.center, arc.radius, self.bound.0, x) + circle_area(arc.center, arc.radius, self.bound.0, arc.center.0)
                } else {
                    arc.center.0 * h + circle_area(arc.center, arc.radius, y, arc.center.0) - circle_area(arc.center, arc.radius, self.bound.0, arc.center.0)
                }
            }
        }
    }
}

fn circle_area(center: Point, radius: f64, lower: f64, right: f64) -> f64 {
    fn f(d: f64) -> f64 {
        d.acos() - (1.0 - d * d).sqrt() * d
    }
    let w = (center.0 - right) / radius;
    let h = (center.1 - lower) / radius;

    radius.powi(2) *
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

fn segment_area(p1: Point, p2: Point, lower: f64, right: f64) -> f64 {
    let y1 = p1.1.min(p2.1);
    let y2 = p1.1.max(p2.1).min(lower);
    if y1 < y2 {
        let x1 = geometry::intersect_line_and_horizon(p1, p2, y1);
        let x2 = geometry::intersect_line_and_horizon(p1, p2, y2);
        let (x1, x2) = if x1 < x2 {(x1, x2)} else {(x2, x1)};
        ((y2 - y1) * if right < x1 {
            right
        } else if right < x2 {
            right - (right - x1).powi(2) / (x2 - x1) / 2.0
        } else {
            (x1 + x2) / 2.0
        }).copysign(p1.1 - p2.1)
    } else {
        0.0
    }
}

fn angle_norm(a1: f64, a2: f64) -> (f64, f64) {
    let (a1, a2) = if a1 < a2 { (a1, a2) } else { (a2, a1) };
    let a = a1.rem_euclid(PI * 2.0);
    (a, (a2 - a).rem_euclid(PI * 2.0) + a)
}

pub fn img_blend_pixel<X, C: PositionColor<X>>(
    buf: &mut ImageBuffer<X, Vec<u8>>,
    position_color: &C,
    x: i32,
    y: i32,
    r: f64,
) where
    X: Pixel<Subpixel = u8> + 'static,
{
    if 0 <= x && x < buf.width() as i32 && 0 <= y && y < buf.height() as i32 {
        let pixel = position_color.position_color((x, y).into());
        let (x, y) = (x as u32, y as u32);
        let pixel = blend_pixel(*buf.get_pixel(x, y), pixel, r);
        buf.put_pixel(x, y, pixel);
    }
}

pub fn blend_pixel<X>(p1: X, p2: X, r: f64) -> X
where
    X: Pixel<Subpixel = u8> + 'static,
{
    p1.map2(&p2, |a, b| (a as f64 * (1.0 - r) + b as f64 * r).round() as u8)
}
