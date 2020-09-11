use crate::point::Point;
use crate::models::Line;
use crate::path3::{Path, PathItem};
use lyon_geom::{
    euclid::{Angle, default::Point2D},
    Arc, QuadraticBezierSegment
};

pub fn path_flatten(path: &Path, tolerance: f64) -> Path {
    let mut pis = Vec::new();
    for pi in path.0.iter() {
        match pi {
            PathItem::Line(line) => {
                pis.push(PathItem::Line(line.clone()));
            }
            PathItem::Arc(arc) => {
                let arc = Arc {
                    center: point_to_point2d(&arc.center),
                    radii: (arc.radius, arc.radius).into(),
                    start_angle: Angle::radians(arc.angle1),
                    sweep_angle: Angle::radians(arc.angle2 - arc.angle1),
                    x_rotation: Angle::radians(0.0),
                };
                let vec: Vec<_> = if arc.sweep_angle.radians.is_sign_positive() {
                    vec![arc.from()].into_iter().chain(arc.flattened(tolerance)).map(|x| x.to_tuple().into()).collect()
                } else {
                    let mut vec: Vec<_> = vec![arc.to()].into_iter().chain(arc.flip().flattened(tolerance)).map(|x| x.to_tuple().into()).collect();
                    vec.reverse();
                    vec
                };
                let mut p = vec[0];
                for q in vec {
                    pis.push(PathItem::Line(Line(p, q)));
                    p = q;
                }
            }
            PathItem::Ellipse(ellipse) => {
                let arc = Arc {
                    center: point_to_point2d(&ellipse.center),
                    radii: (ellipse.radius_x, ellipse.radius_y).into(),
                    start_angle: Angle::radians(ellipse.angle1),
                    sweep_angle: Angle::radians(ellipse.angle2 - ellipse.angle1),
                    x_rotation: Angle::radians(ellipse.rotation),
                };
                let vec: Vec<_> = if arc.sweep_angle.radians.is_sign_positive() {
                    vec![arc.from()].into_iter().chain(arc.flattened(tolerance)).map(|x| x.to_tuple().into()).collect()
                } else {
                    let mut vec: Vec<_> = vec![arc.to()].into_iter().chain(arc.flip().flattened(tolerance)).map(|x| x.to_tuple().into()).collect();
                    vec.reverse();
                    vec
                };
                let mut p = vec[0];
                for q in vec {
                    pis.push(PathItem::Line(Line(p, q)));
                    p = q;
                }
            }
            PathItem::Quad(quad) => {
                let it = QuadraticBezierSegment{
                    from: point_to_point2d(&quad.start),
                    ctrl: point_to_point2d(&quad.control1),
                    to: point_to_point2d(&quad.end),
                }.flattened(tolerance).map(|x| x.to_tuple().into());
                let mut p = quad.start;
                for q in it {
                    pis.push(PathItem::Line(Line(p, q)));
                    p = q;
                }
            }
        }
    }
    Path(pis)
}

fn point_to_point2d(p: &Point) -> Point2D<f64> {
    Point2D::new(p.0, p.1)
}

#[test]
fn test() {
    let a= Arc{
        center: Point2D::new(0.0f64, 0.0),
        radii: (1.0f64, 1.0).into(),
        start_angle: Angle::radians(std::f64::consts::PI),
        sweep_angle: Angle::radians(-1.0),
        x_rotation: Angle::radians(0.0),
    };
    dbg!(a.sample(1.0));
}
