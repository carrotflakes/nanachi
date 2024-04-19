//! `rasterize` method draws a path

use crate::buffer::GenericBuffer;
use crate::fill_rule::FillRule;
use crate::point::Point;

pub type RasterizeBuffer = GenericBuffer<f32>;

impl RasterizeBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        GenericBuffer {
            width,
            height,
            buffer: vec![0.0; (width * height) as usize],
        }
    }

    /// Write the area of the [`Path`].
    pub fn rasterize(
        &mut self,
        segments: impl Iterator<Item = (Point, Point)>,
        fill_rule: impl FillRule,
        writer: &mut impl FnMut(u32, u32, f32),
        write_transparent_src: bool,
    ) {
        let mut bound = [self.width as f32, 0.0f32, self.height as f32, 0.0f32];
        for (a, b) in segments {
            if a.y() == b.y() {
                continue;
            }
            let (a, b, signum) = if a.y() < b.y() {
                (a, b, -1.0)
            } else {
                (b, a, 1.0)
            };
            let upper = a.y();
            let lower = b.y();
            if lower < 0.0 || self.height as f32 <= upper {
                continue;
            }
            bound[0] = bound[0].min(a.x().min(b.x()));
            bound[1] = bound[1].max(a.x().max(b.x()));
            bound[2] = bound[2].min(upper);
            bound[3] = bound[3].max(lower);
            if a.x() == b.x() {
                if 0.0 <= upper {
                    if lower <= upper.ceil() {
                        f2(&mut self.buffer, self.width, signum, upper, lower, a.x());
                        continue;
                    }
                    f2(
                        &mut self.buffer,
                        self.width,
                        signum,
                        upper,
                        upper.ceil(),
                        a.x(),
                    );
                }
                if lower < self.height as f32 {
                    f2(
                        &mut self.buffer,
                        self.width,
                        signum,
                        lower.floor(),
                        lower,
                        a.x(),
                    );
                }
                for y in
                    (upper.ceil() as i32).max(0)..(lower.floor() as i32).min(self.height as i32)
                {
                    f2(
                        &mut self.buffer,
                        self.width,
                        signum,
                        y as f32,
                        (y + 1) as f32,
                        a.x(),
                    );
                }
            } else {
                let int = Intersection::new(a, b);
                if 0.0 <= upper {
                    if lower <= upper.ceil() {
                        f1(&mut self.buffer, self.width, &int, signum, upper, lower);
                        continue;
                    }
                    f1(
                        &mut self.buffer,
                        self.width,
                        &int,
                        signum,
                        upper,
                        upper.ceil(),
                    );
                }
                if lower < self.height as f32 {
                    f1(
                        &mut self.buffer,
                        self.width,
                        &int,
                        signum,
                        lower.floor(),
                        lower,
                    );
                }
                for y in
                    (upper.ceil() as i32).max(0)..(lower.floor() as i32).min(self.height as i32)
                {
                    f1(
                        &mut self.buffer,
                        self.width,
                        &int,
                        signum,
                        y as f32,
                        (y + 1) as f32,
                    );
                }
            }
        }
        self.transfer(fill_rule, writer, write_transparent_src, bound);
    }

    /// Write the area of the [`Path`] without anti-aliasing.
    pub fn rasterize_no_aa(
        &mut self,
        segments: impl Iterator<Item = (Point, Point)>,
        fill_rule: impl FillRule,
        writer: &mut impl FnMut(u32, u32, f32),
        write_transparent_src: bool,
    ) {
        let mut bound = [self.width as f32, 0.0f32, self.height as f32, 0.0f32];
        for (a, b) in segments {
            if a.y() == b.y() {
                continue;
            }
            let (a, b, signum) = if a.y() < b.y() {
                (a, b, -1.0)
            } else {
                (b, a, 1.0)
            };
            let upper = a.y();
            let lower = b.y();
            if lower < 0.0 || self.height as f32 <= upper {
                continue;
            }
            bound[0] = bound[0].min(a.x().min(b.x()));
            bound[1] = bound[1].max(a.x().max(b.x()));
            bound[2] = bound[2].min(upper);
            bound[3] = bound[3].max(lower);
            let int = Intersection::new(a, b);
            let width = self.width as usize;
            for y in (upper.round() as i32).max(0) as usize
                ..(lower.round() as i32).min(self.height as i32) as usize
            {
                let x = int.intersect_h(y as f32 + 0.5).round() as usize;
                if width <= x {
                    continue;
                }
                self.buffer[y * width as usize + x.max(0) as usize] += signum;
            }
        }
        self.transfer(fill_rule, writer, write_transparent_src, bound);
    }

    #[inline]
    pub fn transfer(
        &mut self,
        fill_rule: impl FillRule,
        writer: &mut impl FnMut(u32, u32, f32),
        write_transparent_src: bool,
        bound: [f32; 4],
    ) {
        if write_transparent_src {
            for y in 0..self.height {
                let mut acc = 0.0;
                for x in 0..self.width {
                    let i = (y * self.width + x) as usize;
                    acc += self.buffer[i];
                    self.buffer[i] = 0.0;
                    writer(x, y, fill_rule.apply(acc));
                }
            }
        } else {
            for y in
                bound[2].max(0.0).floor() as u32..bound[3].min(self.height as f32).ceil() as u32
            {
                let mut acc = 0.0;
                for x in bound[0].max(0.0).floor() as u32
                    ..(bound[1] + 1.0).min(self.width as f32).ceil() as u32
                {
                    let i = (y * self.width + x) as usize;
                    acc += self.buffer[i];
                    self.buffer[i] = 0.0;
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
fn f1(buf: &mut Vec<f32>, width: u32, int: &Intersection, signum: f32, upper: f32, lower: f32) {
    let offset = upper.floor() as usize * width as usize;
    let mut acc = 0.0;
    let mut v = 0.0;
    let mut write = |x: i32, a: f32| {
        buf[offset + x as usize] += (a - acc - v) * signum;
        v = a - acc;
        acc = a;
    };
    let upper_x = int.intersect_h(upper);
    let lower_x = int.intersect_h(lower);
    let xi = if upper_x < lower_x {
        for xi in (upper_x.floor() as i32).max(0)..(lower_x.floor() as i32).min(width as i32) {
            let x = (xi + 1) as f32;
            let y = int.intersect_v(x);
            write(xi, (x - upper_x) * (y - upper) * 0.5);
        }
        lower_x.floor() as i32
    } else {
        for xi in (lower_x.floor() as i32).max(0)..(upper_x.floor() as i32).min(width as i32) {
            let x = (xi + 1) as f32;
            let y = int.intersect_v(x);
            write(xi, (x - lower_x) * (lower - y) * 0.5);
        }
        upper_x.floor() as i32
    };
    if xi < 0 {
        write(0, lower - upper);
    } else if xi < width as i32 {
        let a = ((xi + 1) as f32 - (upper_x + lower_x) * 0.5) * (lower - upper);
        write(xi, a);
        if xi + 1 < width as i32 {
            write(xi + 1, a + (lower - upper));
        }
    }
}

#[inline]
fn f2(buf: &mut Vec<f32>, width: u32, signum: f32, upper: f32, lower: f32, x: f32) {
    let offset = upper.floor() as usize * width as usize;
    if x < 0.0 {
        buf[offset] += (lower - upper) * signum;
    } else if x < width as f32 {
        let a = (1.0 - x.fract()) * (lower - upper);
        let x = x.floor() as usize;
        buf[offset + x] += a * signum;
        if x + 1 < width as usize {
            buf[offset + x + 1] += (lower - upper - a) * signum;
        }
    }
}

struct Intersection(f32, f32, f32, f32);

impl Intersection {
    #[inline]
    fn new(a: Point, b: Point) -> Intersection {
        Intersection(
            a.y(),
            (b.x() - a.x()) / (b.y() - a.y()),
            a.x(),
            (b.y() - a.y()) / (b.x() - a.x()),
        )
    }

    #[inline]
    fn intersect_h(&self, y: f32) -> f32 {
        (y - self.0).mul_add(self.1, self.2)
    }

    #[inline]
    fn intersect_v(&self, x: f32) -> f32 {
        (x - self.2).mul_add(self.3, self.0)
    }
}
