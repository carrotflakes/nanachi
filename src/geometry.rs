use crate::point::Point;

pub fn intersect_line_and_line(p1: Point, p2: Point, p3: Point, p4: Point) -> Point {
  let det = (p1.0 - p2.0) * (p4.1 - p3.1) - (p4.0 - p3.0) * (p1.1 - p2.1);
  let t = ((p4.1 - p3.1) * (p4.0 - p2.0) + (p3.0 - p4.0) * (p4.1 - p2.1)) / det;
  let x = t * p1.0 + (1.0 - t) * p2.0;
  let y = t * p1.1 + (1.0 - t) * p2.1;
  Point(x, y)
}

pub fn point_is_right_side_of_line(p1: Point, p2: Point) -> bool {
    p1.0 * p2.1 < p1.1 * p2.0
}
