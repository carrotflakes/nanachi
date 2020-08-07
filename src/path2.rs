use crate::point::Point;

#[derive(Debug, Clone)]
pub enum PathAnchor {
    Point(Point),
    Arc{
        center: Point,
        radius: f64,
        angle1: f64,
        angle2: f64,
    },
}

#[derive(Debug, Clone)]
pub struct Path {
    pub anchors: Vec<PathAnchor>,
    pub close: bool,
}

#[derive(Debug, Clone)]
pub enum PathEdge {
    Line(Point, Point),
    Arc{
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
        for i in 0..self.anchors.len() - if self.close {1} else {0} {
            match (&self.anchors[i], &self.anchors[(i + 1) % self.anchors.len()]) {
                (PathAnchor::Point(p1), PathAnchor::Point(p2)) => {
                    edges.push(PathEdge::Line(*p1, *p2));
                }
                (PathAnchor::Point(p), PathAnchor::Arc { center, radius, angle1, angle2 }) => {
                    let (sin, cos) = angle1.sin_cos();
                    edges.push(PathEdge::Line(*p, *center + Point(cos * radius, -sin * radius)));
                    edges.push(PathEdge::Arc {center: *center, radius: *radius, angle1: *angle1, angle2: *angle2});
                }
                (PathAnchor::Arc { center, radius, angle1: _, angle2 }, PathAnchor::Point(p)) => {
                    let (sin, cos) = angle2.sin_cos();
                    edges.push(PathEdge::Line(*center + Point(cos * radius, -sin * radius), *p));
                }
                (PathAnchor::Arc { center: c1, radius: r1, angle1: _, angle2: a12 }, PathAnchor::Arc { center: c2, radius: r2, angle1: a21, angle2: a22 }) => {
                    let (sin1, cos1) = a12.sin_cos();
                    let (sin2, cos2) = a21.sin_cos();
                    edges.push(PathEdge::Line(*c1 + Point(cos1 * r1, -sin1 * r1), *c2 + Point(cos2 * r2, -sin2 * r2)));
                    edges.push(PathEdge::Arc {center: *c2, radius: *r2, angle1: *a21, angle2: *a22});
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
                let (la, ra) = edge_path_(width, &self.anchors[i], &self.anchors[(i + self.anchors.len() - 1).rem_euclid(self.anchors.len())], &self.anchors[(i+1).rem_euclid(self.anchors.len())]);
                left_anchors.push(la);
                right_anchors.push(ra);
            }
            vec![Path{
                anchors: left_anchors, close: true
            },
            Path{
                anchors: right_anchors, close: true
            }]
        } else {
            for i in 1..self.anchors.len() - 1 {
                let (la, ra) = edge_path_(width, &self.anchors[i], &self.anchors[(i + self.anchors.len() - 1).rem_euclid(self.anchors.len())], &self.anchors[(i+1).rem_euclid(self.anchors.len())]);
                left_anchors.push(la);
                right_anchors.push(ra);

            }
            left_anchors.extend(right_anchors);
            vec![Path{
                anchors: left_anchors, close: true
            }]
        }
    }
}

fn edge_path_(width: f64, a1: &PathAnchor, a2: &PathAnchor, a3: &PathAnchor) -> (PathAnchor, PathAnchor) {
    let left = match a2 {
        PathAnchor::Point(p) => *p,
        PathAnchor::Arc { center, radius, angle1: _, angle2 } => *center + Point(angle2.cos() * radius, -angle2.sin() * radius),
    };
    let right = match a3 {
        PathAnchor::Point(p) => *p,
        PathAnchor::Arc { center, radius, angle1, angle2: _ } => *center + Point(angle1.cos() * radius, -angle1.sin() * radius),
    };
    match a1 {
        PathAnchor::Point(p) => {
            println!("{:?} {:?} {:?}", left, p, right);
            let a1 = (left.1 - p.1).atan2(p.0 - left.0);
            let a2 = (right.1 - p.1).atan2(p.0 - right.0);
            let a = (a2 + a1) / 2.0;
            let aa = ((a2 - a1 + std::f64::consts::PI) / 2.0).abs();//(a2 - a1).rem_euclid(std::f64::consts::PI * 2.0);
            let r = width / aa.cos();
            println!("!a:{:?} a.cos:{:?} a1:{} a2:{} a:{} aa:{} r:{}", a, a.cos(), a1, a2, a, aa, r);
            let dp = Point(a.cos() * r, -a.sin() * r);
            (PathAnchor::Point(*p + dp), PathAnchor::Point(*p - dp))
        }
        PathAnchor::Arc { center, radius, angle1, angle2 } => {
            (PathAnchor::Arc{
                center: *center, radius: radius + width, angle1: *angle1, angle2: *angle2 // fixme
            }, (PathAnchor::Arc{
                center: *center, radius: radius - width, angle1: *angle1, angle2: *angle2
            }))
        }
    }
}
