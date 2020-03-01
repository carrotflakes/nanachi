#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl std::ops::Add for Point {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Point(self.0 + rhs.0, self.1 + rhs.1)
  }
}

impl std::ops::Mul<f64> for Point {
  type Output = Self;
  fn mul(self, rhs: f64) -> Self {
    Point(self.0 * rhs, self.1 * rhs)
  }
}


pub struct Bezier {
  pub points: Vec<Point>,
  pub close: bool
}

impl Bezier {
  pub fn get_point(&self, pos: f64) -> Point {
    let i = pos.floor() as usize * 3;
    let ps = &self.points[i..i+4];
    let v = pos.fract();
    let iv = 1.0 - v;
    (ps[0] + ps[1] * v) * iv + (ps[3] + ps[2] * iv) * v
  }

  pub fn as_lines_points(&self, division: usize) -> Vec<Point> {
    assert!(1 <= division);
    let mut vec = Vec::new();
    for i in 0..self.anchor_num() - 1 {
      vec.push(self.points[i]);
      for j in 1..division {
        vec.push(self.get_point(i as f64 + j as f64 / division as f64));
      }
    }
    vec.push(*self.points.last().unwrap());
    vec
  }

  pub fn anchor_num(&self) -> usize {
    self.points.len() / 3 + 1
  }
}
