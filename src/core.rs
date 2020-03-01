use image::{ImageBuffer, Rgb};

pub fn intersection_(ax: f64, ay: f64, bx: f64, by: f64, hy: f64) -> Option<f64> {
  if ay != by && ((hy < ay) ^ (hy < by)) {
      let r = (hy - ay) / (by - ay);
      Some(ax * (1.0 - r) + bx * r)
  } else {
      None
  }
}

pub fn transform(p: &(f64, f64), translation: (f64, f64), rotation: f64, scale: (f64, f64)) -> (f64, f64) {
  let (x, y) = p;
  let (sin, cos) = rotation.sin_cos();
  let (x, y) = (x * cos - y * sin, x * sin + y * cos);
  let (x, y) = (x * scale.0, y * scale.1);
  let (x, y) = (x + translation.0, y + translation.1);
  (x, y)
}

pub fn draw_line(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, mut p1: (f64, f64), mut p2: (f64, f64), pixel: Rgb<u8>) {
  if (p1.0 - p2.0).abs() < (p1.1 - p2.1).abs() {
      if p1.1 > p2.1 {
          std::mem::swap(&mut p1, &mut p2);
      }
      for y in p1.1.max(0.0) as u32..=p2.1.max(0.0).min(img.height() as f64 - 1.0) as u32 {
          let x = (p2.0 - p1.0) * ((y as f64 - p1.1) / (p2.1 - p1.1)) + p1.0;
          if 0.0 <= x && x < img.width() as f64 {
              img.put_pixel(x as u32, y, pixel);
          }
      }
  } else {
      if p1.0 > p2.0 {
          std::mem::swap(&mut p1, &mut p2);
      }
      for x in p1.0.max(0.0) as u32..=p2.0.max(0.0).min(img.width() as f64 - 1.0) as u32 {
          let y = (p2.1 - p1.1) * ((x as f64 - p1.0) / (p2.0 - p1.0)) + p1.1;
          if 0.0 <= y && y < img.height() as f64 {
              img.put_pixel(x, y as u32, pixel);
          }
      }
  }
}

pub fn draw_path(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, ps: &[(f64, f64)], pixel: Rgb<u8>) {
  for y in 0..img.height() {
      for x in 0..img.width() {
          for pair in ps.windows(2) {
              let d = distance_between_line_segment_and_point(&pair[0], &pair[1], &(x as f64, y as f64));
              if d < 5.0 {
                  img.put_pixel(x, y, pixel);
              } else if d < 6.0 {
                  img.put_pixel(x, y, blend_rgb(*img.get_pixel(x, y), pixel, 6.0 - d));
              }
          }
      }
  }
}

pub fn distance_between_line_and_point(p1: (f64, f64), p2: (f64, f64), p0: (f64, f64)) -> f64 {
  ((p2.1 - p1.1) * p0.0 - (p2.0 - p1.0) * p0.1 + p2.0 * p1.1 - p2.1 * p1.0).abs() / (p2.1 - p1.1).hypot(p2.0 - p1.0)
}

pub fn distance_between_line_segment_and_point(p1: &(f64, f64), p2: &(f64, f64), p0: &(f64, f64)) -> f64 {
  let a = p2.0 - p1.0;
  let b = p2.1 - p1.1;
  let a2 = a.powi(2);
  let b2 = b.powi(2);
  let r2 = a2 + b2;
  let tt = -(a * (p1.0 - p0.0) + b * (p1.1 - p0.1));
  if tt < 0.0 {
      (p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)
  } else if tt > r2 {
      (p2.0 - p0.0).powi(2) + (p2.1 - p0.1).powi(2)
  } else {
      (a * (p1.1 - p0.1) - b * (p1.0 - p0.0)).powi(2) / r2
  }
}

pub fn draw_fill(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, ps: &[(f64, f64)], pixel: Rgb<u8>) {
  for y in 0..img.height() {
      let mut vec = ps.windows(2).filter_map(|pair| intersection_(pair[0].0, pair[0].1, pair[1].0, pair[1].1, y as f64)).collect::<Vec<f64>>();
      vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
      for i in 0..vec.len() / 2 {
          let s = vec[i * 2].max(0.0) as u32;
          let e = vec[i * 2 + 1].max(0.0).min(img.width() as f64) as u32;
          for x in s..e {
              img.put_pixel(x, y, pixel);
          }
      }
  }
}

pub fn blend_rgb(p1: Rgb<u8>, p2: Rgb<u8>, r: f64) -> Rgb<u8> {
  Rgb([
      (p1[0] as f64 * (1.0 - r) + p2[0] as f64 * r) as u8,
      (p1[1] as f64 * (1.0 - r) + p2[1] as f64 * r) as u8,
      (p1[2] as f64 * (1.0 - r) + p2[2] as f64 * r) as u8
  ])
}
