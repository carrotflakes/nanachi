use crate::models::Line;
use crate::path::{Path, PathItem};
use crate::point::Point;
use crate::fill_rule::FillRule;

#[derive(Clone)]
pub struct Rasterize {
    width: u32,
    height: u32,
    buffer: Vec<f64>,
}

impl Rasterize {
    pub fn new(width: u32, height: u32) -> Rasterize {
        Rasterize {
            width,
            height,
            buffer: vec![0.0; (width * height) as usize],
        }
    }

    pub fn clear(&mut self) {
        for i in 0..(self.width * self.height) as usize {
            self.buffer[i] = 0.0;
        }
    }

    pub fn rasterize<I: Iterator<Item = PathItem>, F: FnMut(u32, u32, f64), FR: FillRule>(&mut self, pis: I, fill_rule: FR, writer: &mut F, write_transparent_src: bool) {
        self.clear();
        for pi in pis {
            let Line(a, b)  = match pi {
                PathItem::Line(l) => l,
                PathItem::CloseAndJump | PathItem::Jump => continue,
                _ => panic!("Un-line passed to draw_fill_only_lines"),
            };
            if a.1 == b.1 {
                continue;
            }
            let (a, b, signum) = if a.1 < b.1 {
                (a, b, -1.0)
            } else {
                (b, a, 1.0)
            };
            let upper = a.1;
            let lower = b.1;
            if lower < 0.0 || self.height as f64 <= upper {
                continue;
            }
            if a.0 == b.0 {
                if 0.0 <= upper {
                    if lower <= upper.ceil() {
                        f2(&mut self.buffer, self.width, signum, upper, lower, a.0);
                        continue;
                    }
                    f2(&mut self.buffer, self.width, signum, upper, upper.ceil(), a.0);
                }
                if lower < self.height as f64 {
                    f2(&mut self.buffer, self.width, signum, lower.floor(), lower, a.0);
                }
                for y in (upper.ceil() as i32).max(0)..(lower.floor() as i32).min(self.height as i32) {
                    f2(&mut self.buffer, self.width, signum, y as f64, (y + 1) as f64, a.0);
                }
            } else {
                let int = Intersection::new(a, b);
                if 0.0 <= upper {
                    if lower <= upper.ceil() {
                        f(&mut self.buffer, self.width, &int, signum, upper, lower);
                        continue;
                    }
                    f(&mut self.buffer, self.width, &int, signum, upper, upper.ceil());
                }
                if lower < self.height as f64 {
                    f(&mut self.buffer, self.width, &int, signum, lower.floor(), lower);
                }
                for y in (upper.ceil() as i32).max(0)..(lower.floor() as i32).min(self.height as i32) {
                    f(&mut self.buffer, self.width, &int, signum, y as f64, (y + 1) as f64);
                }
            }
        }
        if write_transparent_src {
            for y in 0..self.height {
                let mut acc = 0.0;
                for x in 0..self.width {
                    acc += self.buffer[(y * self.width + x) as usize];
                    writer(x, y, fill_rule.apply(acc));
                }
            }
        } else  {
            for y in 0..self.height {
                let mut acc = 0.0;
                for x in 0..self.width {
                    acc += self.buffer[(y * self.width + x) as usize];
                    let v = fill_rule.apply(acc);
                    if v != 0.0 {
                        writer(x, y, v);
                    }
                }
            }
        }
    }
}

#[inline]
fn f(buf: &mut Vec<f64>, width: u32, int: &Intersection, signum: f64, upper: f64, lower: f64) {
    let offset = upper.floor() as usize * width as usize;
    let mut acc = 0.0;
    let mut v = 0.0;
    let mut write = |x: i32, a: f64| {
        buf[offset + x.max(0) as usize] += (a - acc - v) * signum;
        v = a - acc;
        acc = a;
    };
    let upper_x = int.intersect_h(upper);
    let lower_x = int.intersect_h(lower);
    if upper_x < lower_x {
        for xi in (upper_x.floor() as i32).max(0)..(lower_x.floor() as i32).min(width as i32) {
            let x = (xi + 1) as f64;
            let y = int.intersect_v(x);
            write(xi, (x - upper_x) * (y - upper) * 0.5);
        }
        let xi = lower_x.floor() as i32;
        if xi < width as i32 {
            let a= ((xi + 1) as f64 - (upper_x + lower_x) * 0.5) * (lower - upper);
            write(xi, a);
            if xi + 1 < width as i32 {
                write(xi + 1, a + (lower - upper));
            }
        }
    } else {
        for xi in (lower_x.floor() as i32).max(0)..(upper_x.floor() as i32).min(width as i32) {
            let x = (xi + 1) as f64;
            let y = int.intersect_v(x);
            write(xi, (x - lower_x) * (lower - y) * 0.5);
        }
        let xi = upper_x.floor() as i32;
        if xi < width as i32 {
            let a= ((xi + 1) as f64 - (upper_x + lower_x) * 0.5) * (lower - upper);
            write(xi, a);
            if xi + 1 < width as i32 {
                write(xi + 1, a + (lower - upper));
            }
        }
    }
}

#[inline]
fn f2(buf: &mut Vec<f64>, width: u32, signum: f64, upper: f64, lower: f64, x: f64) {
    let offset = upper.floor() as usize * width as usize;
    let a = (1.0 - x.fract()) * (lower - upper);
    let x = x.floor() as i32;
    if x < 0 {
        buf[offset] += signum;
    } else if x < width as i32 {
        buf[offset + x as usize] += a * signum;
        if x + 1 < width as i32 {
            buf[offset + x as usize + 1] += (lower - upper - a) * signum;
        }
    }
}

struct Intersection(f64, f64, f64, f64);

impl Intersection {
    #[inline]
    fn new(a: Point, b: Point) -> Intersection {
        debug_assert_ne!(a.0, b.0);
        debug_assert_ne!(a.1, b.1);
        Intersection(a.1, (b.0 - a.0) / (b.1 - a.1), a.0, (b.1 - a.1) / (b.0 - a.0))
    }

    #[inline]
    fn intersect_h(&self, y: f64) -> f64 {
        (y - self.0).mul_add(self.1, self.2)
    }

    #[inline]
    fn intersect_v(&self, x: f64) -> f64 {
        (x - self.2).mul_add(self.3, self.0)
    }
}
