use crate::point::Point;
use std::f64::consts::{FRAC_PI_2, PI};

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
}

#[derive(Debug, Clone)]
pub enum PathAnchor {
    Point(Point),
    Arc(Arc),
    Ellipse(Ellipse),
}

impl PathAnchor {
    pub fn flip(&self) -> PathAnchor {
        match self {
            PathAnchor::Point(_) => self.clone(),
            PathAnchor::Arc(arc) => PathAnchor::Arc(Arc {
                center: arc.center,
                radius: arc.radius,
                angle1: arc.angle2,
                angle2: arc.angle1,
            }),
            PathAnchor::Ellipse(ellipse) => PathAnchor::Ellipse(Ellipse {
                center: ellipse.center,
                radius_x: ellipse.radius_x,
                radius_y: ellipse.radius_y,
                rotation: ellipse.rotation,
                angle1: ellipse.angle2,
                angle2: ellipse.angle1,
            }),
        }
    }

    fn right_point(&self) -> Point {
        match self {
            PathAnchor::Point(p) => *p,
            PathAnchor::Arc(arc) => {
                arc.center
                    + Point(
                        arc.angle2.cos() * arc.radius,
                        -arc.angle2.sin() * arc.radius,
                    )
            }
            PathAnchor::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle1.min(ellipse.angle2).cos() * ellipse.radius_x;
                let y = -ellipse.angle1.min(ellipse.angle2).sin() * ellipse.radius_y;
                ellipse.center + Point(x * cos - y * sin, x * sin + y * cos)
            }
        }
    }

    fn left_point(&self) -> Point {
        match self {
            PathAnchor::Point(p) => *p,
            PathAnchor::Arc(arc) => {
                arc.center
                    + Point(
                        arc.angle1.cos() * arc.radius,
                        -arc.angle1.sin() * arc.radius,
                    )
            }
            PathAnchor::Ellipse(ellipse) => {
                let (sin, cos) = ellipse.rotation.sin_cos();
                let x = ellipse.angle1.max(ellipse.angle2).cos() * ellipse.radius_x;
                let y = -ellipse.angle1.max(ellipse.angle2).sin() * ellipse.radius_y;
                ellipse.center + Point(x * cos - y * sin, x * sin + y * cos)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    pub anchors: Vec<PathAnchor>,
    pub close: bool,
}

#[derive(Debug, Clone)]
pub enum PathEdge {
    Line(Point, Point),
    Arc(Arc),
    Ellipse(Ellipse),
}

impl Path {
    pub fn new(anchors: Vec<PathAnchor>, close: bool) -> Path {
        Path { anchors, close }
    }

    pub fn edges(&self) -> Vec<PathEdge> {
        let mut edges = Vec::new();
        let mut last_point = self.anchors.last().unwrap().right_point();
        for i in 0..self.anchors.len() - if self.close { 0 } else { 1 } {
            let point = self.anchors[i].left_point();
            if last_point != point {
                edges.push(PathEdge::Line(last_point, point));
            }
            match &self.anchors[i] {
                PathAnchor::Point(_) => {}
                PathAnchor::Arc(arc) => {
                    edges.push(PathEdge::Arc(arc.clone()));
                }
                PathAnchor::Ellipse(ellipse) => {
                    edges.push(PathEdge::Ellipse(ellipse.clone()));
                }
            }
            last_point = self.anchors[i].right_point();
        }
        edges
    }

    // path の輪郭を Vec<path> として取得
    pub fn edge_path(&self, width: f64) -> Vec<Path> {
        let mut left_anchors = Vec::new();
        let mut right_anchors = Vec::new();
        if self.close {
            for i in 0..self.anchors.len() {
                let (la, ra) = edge_path_(
                    width,
                    &self.anchors[i],
                    &self.anchors[(i + self.anchors.len() - 1).rem_euclid(self.anchors.len())],
                    &self.anchors[(i + 1).rem_euclid(self.anchors.len())],
                );
                left_anchors.push(la);
                right_anchors.push(ra);
            }
            right_anchors.reverse();
            vec![
                Path {
                    anchors: left_anchors,
                    close: true,
                },
                Path {
                    anchors: right_anchors,
                    close: true,
                },
            ]
        } else {
            for i in 1..self.anchors.len() - 1 {
                let (la, ra) = edge_path_(
                    width,
                    &self.anchors[i],
                    &self.anchors[(i + self.anchors.len() - 1).rem_euclid(self.anchors.len())],
                    &self.anchors[(i + 1).rem_euclid(self.anchors.len())],
                );
                left_anchors.push(la);
                right_anchors.push(ra);
            }
            right_anchors.reverse();
            right_anchors.extend(cap(
                width,
                &self.anchors[0].flip(),
                self.anchors[1].left_point(),
            ));
            left_anchors.extend(cap(
                width,
                &self.anchors[self.anchors.len() - 1],
                self.anchors[self.anchors.len() - 2].right_point(),
            ));
            left_anchors.extend(right_anchors);
            vec![Path {
                anchors: left_anchors,
                close: true,
            }]
        }
    }
}

fn edge_path_(
    width: f64,
    a1: &PathAnchor,
    a2: &PathAnchor,
    a3: &PathAnchor,
) -> (PathAnchor, PathAnchor) {
    let left = a2.right_point();
    let right = a3.left_point();
    match a1 {
        PathAnchor::Point(p) => {
            let a1 = (left.1 - p.1).atan2(p.0 - left.0);
            let a2 = (right.1 - p.1).atan2(p.0 - right.0);
            let a = (a2 + a1) / 2.0;
            let aa = ((a2 - a1 + PI) / 2.0).abs();
            let r = width / aa.cos();
            let dp = Point(a.cos() * r, -a.sin() * r);
            if FRAC_PI_2 < aa % PI {
                (
                    PathAnchor::Point(*p + dp),
                    PathAnchor::Arc(Arc {
                        center: *p,
                        radius: width,
                        angle1: a2 - FRAC_PI_2,
                        angle2: a1 + FRAC_PI_2,
                    }),
                )
            } else {
                (
                    PathAnchor::Arc(Arc {
                        center: *p,
                        radius: width,
                        angle1: a1 - FRAC_PI_2,
                        angle2: a2 + FRAC_PI_2,
                    }),
                    PathAnchor::Point(*p - dp),
                )
            }
        }
        PathAnchor::Arc(arc) => {
            (
                PathAnchor::Arc(Arc{
                    center: arc.center,
                    radius: arc.radius + width,
                    angle1: arc.angle1,
                    angle2: arc.angle2, // fixme
                }),
                (PathAnchor::Arc(Arc {
                    center: arc.center,
                    radius: arc.radius - width,
                    angle1: arc.angle2,
                    angle2: arc.angle1,
                })),
            )
        }
        PathAnchor::Ellipse(ellipse) => {
            (
                PathAnchor::Ellipse(Ellipse {
                    center: ellipse.center,
                    radius_x: ellipse.radius_x + width,
                    radius_y: ellipse.radius_y + width,
                    rotation: ellipse.rotation,
                    angle1: ellipse.angle1,
                    angle2: ellipse.angle2,
                }),
                (PathAnchor::Ellipse(Ellipse {
                    center: ellipse.center,
                    radius_x: ellipse.radius_x - width,
                    radius_y: ellipse.radius_y - width,
                    rotation: ellipse.rotation,
                    angle1: ellipse.angle2,
                    angle2: ellipse.angle1,
                })),
            )
        }
    }
}

fn cap(width: f64, a: &PathAnchor, p: Point) -> Vec<PathAnchor> {
    match a {
        PathAnchor::Point(p1) => {
            let a = ((p.1 - p1.1).atan2(p1.0 - p.0) + PI * 1.5) % (PI * 2.0);
            vec![PathAnchor::Arc(Arc {
                center: *p1,
                radius: width,
                angle1: a,
                angle2: a + PI,
            })]
        }
        PathAnchor::Arc(Arc {
            center,
            radius,
            angle1,
            angle2,
        }) => {
            vec![
                PathAnchor::Arc(Arc {
                    center: *center,
                    radius: radius + width,
                    angle1: *angle1,
                    angle2: *angle2, // fixme
                }),
                PathAnchor::Arc(Arc {
                    center: *center + Point(angle2.cos() * radius, -angle2.sin() * radius),
                    radius: width,
                    angle1: *angle2,
                    angle2: angle2 + PI,
                }),
                PathAnchor::Arc(Arc {
                    center: *center,
                    radius: radius - width,
                    angle1: *angle2,
                    angle2: *angle1,
                }),
            ]
        }
        PathAnchor::Ellipse(Ellipse {
            center,
            radius_x,
            radius_y,
            rotation,
            angle1,
            angle2,
        }) => {
            vec![
                PathAnchor::Ellipse(Ellipse {
                    center: *center,
                    radius_x: radius_x + width,
                    radius_y: radius_y + width,
                    rotation: *rotation,
                    angle1: *angle1,
                    angle2: *angle2, // fixme
                }),
                //TODO
                // PathAnchor::Ellipse(Ellipse {
                //     center: *center + Point(angle2.cos() * radius, -angle2.sin() * radius),
                //     radius: width,
                //     angle1: *angle2,
                //     angle2: angle2 + PI,
                // }), 
                PathAnchor::Ellipse(Ellipse {
                    center: *center,
                    radius_x: radius_x - width,
                    radius_y: radius_y - width,
                    rotation: *rotation,
                    angle1: *angle2,
                    angle2: *angle1,
                }),
            ]
        }
    }
}
