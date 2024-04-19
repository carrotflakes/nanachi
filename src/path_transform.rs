//! Path transformation.

use crate::matrix::Matrix;
use crate::models::{Cubic, Ellipse, Line, Quad};
use crate::path::{Path, PathItem};
use crate::point::Point;
use std::f32::consts::TAU;

/// Transform a path with the [`Matrix`].
pub fn path_transform(path: &Path, matrix: &Matrix) -> Path {
    let mut pis = Vec::with_capacity(path.0.len());
    for pi in path.0.iter() {
        pis.push(match pi {
            PathItem::Line(line) => {
                PathItem::Line(Line([matrix.apply(line.0[0]), matrix.apply(line.0[1])]))
            }
            PathItem::Arc(arc) => PathItem::Ellipse(transform_ellipse(
                &Ellipse {
                    center: arc.center,
                    radius_x: arc.radius,
                    radius_y: arc.radius,
                    rotation: 0.0,
                    angle1: arc.angle1,
                    angle2: arc.angle2,
                },
                matrix,
            )),
            PathItem::Ellipse(ellipse) => PathItem::Ellipse(transform_ellipse(ellipse, matrix)),
            PathItem::Quad(quad) => PathItem::Quad(Quad {
                start: matrix.apply(quad.start),
                end: matrix.apply(quad.end),
                control1: matrix.apply(quad.control1),
            }),
            PathItem::Cubic(cubic) => PathItem::Cubic(Cubic {
                start: matrix.apply(cubic.start),
                end: matrix.apply(cubic.end),
                control1: matrix.apply(cubic.control1),
                control2: matrix.apply(cubic.control2),
            }),
            PathItem::CloseAndJump => PathItem::CloseAndJump,
            PathItem::Jump => PathItem::Jump,
        });
    }
    if matrix.is_direct() {
        Path::new(pis)
    } else {
        Path::new(pis).flip()
    }
}

fn transform_ellipse(ellipse: &Ellipse, matrix: &Matrix) -> Ellipse {
    let matrix = Matrix::new()
        .scale(ellipse.radius_x, ellipse.radius_y)
        .rotate(ellipse.rotation)
        .translate(ellipse.center.x(), ellipse.center.y())
        .then(&matrix);
    let center = Point::from((matrix.0[2], matrix.0[5]));
    // dbg!(am);
    let k = (matrix.0[1].atan2(matrix.0[4]) + matrix.0[3].atan2(matrix.0[0])).tan();
    let w = Point::from((matrix.0[0], matrix.0[3]))
        .rotate(matrix.0[1].atan2(matrix.0[4]))
        .x();
    let h = matrix.0[1].hypot(matrix.0[4]);
    let signum = w.signum() * h.signum();
    let (w, h) = (w.abs(), h.abs());
    // dbg!(k, w, h);
    if !k.is_normal() || k == 0.0 {
        return Ellipse {
            center,
            radius_x: w,
            radius_y: h,
            rotation: matrix.0[3].atan2(matrix.0[0]),
            angle1: ellipse.angle1,
            angle2: ellipse.angle2,
        };
    }
    let rotation = 1.0 / 2.0 * (2.0 * k / (1.0 - k.powi(2) - (h / w).powi(2))).atan();
    let radius_x = w * (1.0 - k / rotation.tan()).sqrt();
    let radius_y = w * (1.0 + k * rotation.tan()).sqrt();
    // dbg!(rotation, radius_x, radius_y);
    let rotation = rotation + matrix.0[4].atan2(matrix.0[1]);
    let matrix2 = Matrix::new()
        .scale(radius_x, radius_y)
        .rotate(rotation)
        .translate(center.x(), center.y())
        .inverse();
    let mut angle1 = matrix2
        .apply(matrix.apply(Point::from_angle(ellipse.angle1)))
        .atan2();
    let mut angle2 = matrix2
        .apply(matrix.apply(Point::from_angle(ellipse.angle2)))
        .atan2();
    // if signum < 0.0 {
    //     std::mem::swap(&mut angle1, &mut angle2);
    // }
    if (ellipse.angle1 < ellipse.angle2) ^ (signum < 0.0) && angle1 >= angle2 {
        angle2 += TAU;
    }
    if (ellipse.angle1 > ellipse.angle2) ^ (signum < 0.0) && angle1 <= angle2 {
        angle1 += TAU;
    }
    Ellipse {
        center,
        radius_x,
        radius_y,
        rotation,
        angle1,
        angle2,
    }
}

#[test]
fn test() {
    let am = Matrix::new().scale(3.0, 2.0).skew_y(0.5).rotate(0.1);
    // let k = (am.0[1] + am.0[2]) / (am.0[4] + am.0[5]) + (am.0[3] + am.0[5]) / (am.0[0] + am.0[2]);
    let k = (std::f32::consts::PI / 2.0 - am.0[4].atan2(am.0[1]) + am.0[3].atan2(am.0[0])).tan();
    let p = am
        .apply(Point::from((1.0, 0.0)))
        .rotate(am.0[1].atan2(am.0[4]))
        .0;
    let q = am.0[1].hypot(am.0[4]);
    dbg!(k, p, q);

    let k = 0.5f32;
    let p = 3.0f32;
    let q = 2.0;
    let rotation = 1.0 / 2.0 * (2.0 * k / (1.0 - k.powi(2) - (q / p).powi(2))).atan();
    let scale_x = p * (1.0 + k * rotation.tan()).sqrt();
    let scale_y = p * (1.0 - k / rotation.tan()).sqrt();
    dbg!(rotation, scale_x, scale_y);
}
