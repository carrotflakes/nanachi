use crate::point::Point;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub enum PathAnchor {
    Point(Point),
    Arc {
        center: Point,
        radius: f64,
        angle1: f64,
        angle2: f64,
    },
}

impl PathAnchor {
    pub fn flip(&self) -> PathAnchor {
        match self {
            PathAnchor::Point(_) => self.clone(),
            PathAnchor::Arc {
                center,
                radius,
                angle1,
                angle2,
            } => PathAnchor::Arc {
                center: *center,
                radius: *radius,
                angle1: *angle2,
                angle2: *angle1,
            },
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
    Arc {
        center: Point,
        radius: f64,
        angle1: f64,
        angle2: f64,
    },
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
                    PathAnchor::Arc {
                        center,
                        radius,
                        angle1,
                        angle2,
                    },
                ) => {
                    let (sin, cos) = angle1.sin_cos();
                    edges.push(PathEdge::Line(
                        *p,
                        *center + Point(cos * radius, -sin * radius),
                    ));
                    edges.push(PathEdge::Arc {
                        center: *center,
                        radius: *radius,
                        angle1: *angle1,
                        angle2: *angle2,
                    });
                }
                (
                    PathAnchor::Arc {
                        center,
                        radius,
                        angle1: _,
                        angle2,
                    },
                    PathAnchor::Point(p),
                ) => {
                    let (sin, cos) = angle2.sin_cos();
                    edges.push(PathEdge::Line(
                        *center + Point(cos * radius, -sin * radius),
                        *p,
                    ));
                }
                (
                    PathAnchor::Arc {
                        center: c1,
                        radius: r1,
                        angle1: _,
                        angle2: a12,
                    },
                    PathAnchor::Arc {
                        center: c2,
                        radius: r2,
                        angle1: a21,
                        angle2: a22,
                    },
                ) => {
                    let (sin1, cos1) = a12.sin_cos();
                    let (sin2, cos2) = a21.sin_cos();
                    edges.push(PathEdge::Line(
                        *c1 + Point(cos1 * r1, -sin1 * r1),
                        *c2 + Point(cos2 * r2, -sin2 * r2),
                    ));
                    edges.push(PathEdge::Arc {
                        center: *c2,
                        radius: *r2,
                        angle1: *a21,
                        angle2: *a22,
                    });
                }
            }
        }
        edges
    }

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
                path_anchor_left_point(&self.anchors[1]),
            ));
            left_anchors.extend(cap(
                width,
                &self.anchors[self.anchors.len() - 1],
                path_anchor_right_point(&self.anchors[self.anchors.len() - 2]),
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
    let left = path_anchor_right_point(a2);
    let right = path_anchor_left_point(a3);
    match a1 {
        PathAnchor::Point(p) => {
            let a1 = (left.1 - p.1).atan2(p.0 - left.0);
            let a2 = (right.1 - p.1).atan2(p.0 - right.0);
            let a = (a2 + a1) / 2.0;
            let aa = ((a2 - a1 + PI) / 2.0).abs();
            let r = width / aa.cos();
            let dp = Point(a.cos() * r, -a.sin() * r);
            (PathAnchor::Point(*p + dp), PathAnchor::Point(*p - dp))
        }
        PathAnchor::Arc {
            center,
            radius,
            angle1,
            angle2,
        } => {
            (
                PathAnchor::Arc {
                    center: *center,
                    radius: radius + width,
                    angle1: *angle1,
                    angle2: *angle2, // fixme
                },
                (PathAnchor::Arc {
                    center: *center,
                    radius: radius - width,
                    angle1: *angle2,
                    angle2: *angle1,
                }),
            )
        }
    }
}

fn cap(width: f64, a: &PathAnchor, p: Point) -> Vec<PathAnchor> {
    match a {
        PathAnchor::Point(p1) => {
            let a = ((p.1 - p1.1).atan2(p1.0 - p.0) + PI * 1.5) % (PI * 2.0);
            vec![PathAnchor::Arc {
                center: *p1,
                radius: width,
                angle1: a,
                angle2: a + PI,
            }]
        }
        PathAnchor::Arc {
            center,
            radius,
            angle1,
            angle2,
        } => {
            vec![
                PathAnchor::Arc {
                    center: *center,
                    radius: radius + width,
                    angle1: *angle1,
                    angle2: *angle2, // fixme
                },
                PathAnchor::Arc {
                    center: *center + Point(angle2.cos() * radius, -angle2.sin() * radius),
                    radius: width,
                    angle1: *angle2,
                    angle2: angle2 + PI,
                },
                PathAnchor::Arc {
                    center: *center,
                    radius: radius - width,
                    angle1: *angle2,
                    angle2: *angle1,
                },
            ]
        }
    }
}

fn path_anchor_right_point(a: &PathAnchor) -> Point {
    match a {
        PathAnchor::Point(p) => *p,
        PathAnchor::Arc {
            center,
            radius,
            angle1: _,
            angle2,
        } => *center + Point(angle2.cos() * radius, -angle2.sin() * radius),
    }
}

fn path_anchor_left_point(a: &PathAnchor) -> Point {
    match a {
        PathAnchor::Point(p) => *p,
        PathAnchor::Arc {
            center,
            radius,
            angle1,
            angle2: _,
        } => *center + Point(angle1.cos() * radius, -angle1.sin() * radius),
    }
}
