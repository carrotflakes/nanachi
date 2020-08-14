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
pub enum PathAnchor {
    Point(Point),
    Arc(Arc),
}

impl PathAnchor {
    pub fn flip(&self) -> PathAnchor {
        match self {
            PathAnchor::Point(_) => self.clone(),
            PathAnchor::Arc(Arc {
                center,
                radius,
                angle1,
                angle2,
            }) => PathAnchor::Arc(Arc{
                center: *center,
                radius: *radius,
                angle1: *angle2,
                angle2: *angle1,
            }),
        }
    }

    fn right_point(&self) -> Point {
        match self {
            PathAnchor::Point(p) => *p,
            PathAnchor::Arc(arc) => {
                arc.center
                    + Point(
                        arc.angle1.min(arc.angle2).cos() * arc.radius,
                        -arc.angle1.min(arc.angle2).sin() * arc.radius,
                    )
            }
        }
    }

    fn left_point(&self) -> Point {
        match self {
            PathAnchor::Point(p) => *p,
            PathAnchor::Arc(arc) => {
                arc.center
                    + Point(
                        arc.angle1.max(arc.angle2).cos() * arc.radius,
                        -arc.angle1.max(arc.angle2).sin() * arc.radius,
                    )
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
}

impl Path {
    pub fn new(anchors: Vec<PathAnchor>, close: bool) -> Path {
        Path { anchors, close }
    }

    pub fn edges(&self) -> Vec<PathEdge> {
        let mut edges = Vec::new();
        for i in 0..self.anchors.len() - if self.close { 0 } else { 1 } {
            match (
                &self.anchors[i],
                &self.anchors[(i + 1) % self.anchors.len()],
            ) {
                (PathAnchor::Point(p1), PathAnchor::Point(p2)) => {
                    edges.push(PathEdge::Line(*p1, *p2));
                }
                (
                    PathAnchor::Point(p),
                    PathAnchor::Arc(arc),
                ) => {
                    let (sin, cos) = arc.angle1.sin_cos();
                    edges.push(PathEdge::Line(
                        *p,
                        arc.center + Point(cos * arc.radius, -sin * arc.radius),
                    ));
                    edges.push(PathEdge::Arc(arc.clone()));
                }
                (
                    PathAnchor::Arc(arc),
                    PathAnchor::Point(p),
                ) => {
                    let (sin, cos) = arc.angle2.sin_cos();
                    edges.push(PathEdge::Line(
                        arc.center + Point(cos * arc.radius, -sin * arc.radius),
                        *p,
                    ));
                }
                (
                    PathAnchor::Arc(arc1),
                    PathAnchor::Arc(arc2),
                ) => {
                    let (sin1, cos1) = arc1.angle2.sin_cos();
                    let (sin2, cos2) = arc2.angle1.sin_cos();
                    edges.push(PathEdge::Line(
                        arc1.center + Point(cos1 * arc1.radius, -sin1 * arc1.radius),
                        arc2.center + Point(cos2 * arc2.radius, -sin2 * arc2.radius),
                    ));
                    edges.push(PathEdge::Arc(arc2.clone()));
                }
            }
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
    }
}
