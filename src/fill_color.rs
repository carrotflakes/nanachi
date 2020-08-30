pub trait FillColor<C> {
    fn fill_color(&self, x: f64, y: f64) -> C;
}

#[derive(Debug, Clone)]
pub struct Constant<C: Clone>(C);

impl<C: Clone> Constant<C> {
    pub fn new(c: C) -> Self {
        Constant(c)
    }
}

impl<C: Clone> FillColor<C> for Constant<C> {
    fn fill_color(&self, _: f64, _: f64) -> C {
        self.0.clone()
    }
}

#[derive(Debug, Clone)]
pub struct BlockCheck<C: Clone>(C, C, f64);

impl<C: Clone> BlockCheck<C> {
    pub fn new(c1: C, c2: C, size: f64) -> Self {
        BlockCheck(c1, c2, size)
    }
}

impl<C: Clone> FillColor<C> for BlockCheck<C> {
    fn fill_color(&self, x: f64, y: f64) -> C {
        if ((x / self.2) as isize + (y / self.2) as isize) % 2 == 0 {
            self.0.clone()
        } else {
            self.1.clone()
        }
    }
}

use image::Pixel;

pub type GradientPoint<X> = (f64, X);

fn gradient<X>(points: &Vec<GradientPoint<X>>, p: f64) -> X
where
X: Pixel<Subpixel = u8> + 'static {
    if p <= points[0].0 {
        return points[0].1;
    }
    for i in 0..points.len() - 1 {
        let right = &points[i + 1];
        if p <= right.0 {
            let left = &points[i];
            return blend_pixel(left.1, right.1, (p - left.0) / (right.0 - left.0));
        }
    }
    points.last().unwrap().1
}

#[derive(Debug, Clone)]
pub struct LinearGradient<X>
where
X: Pixel<Subpixel = u8> + 'static {
    start: (f64, f64),
    sin: f64,
    cos: f64,
    d: f64,
    points: Vec<GradientPoint<X>>,
}

impl<X> LinearGradient<X>
where
X: Pixel<Subpixel = u8> + 'static {
    pub fn new(start: (f64, f64), end: (f64, f64), points: Vec<GradientPoint<X>>) -> LinearGradient<X> {
        assert!(!points.is_empty());
        let d = (end.0 - start.0).hypot(end.1 - start.1);
        LinearGradient {
            start,
            sin: (end.1 - start.1) / d,
            cos: (end.0 - start.0) / d,
            d,
            points,
        }
    }
}

impl<X> FillColor<X> for LinearGradient<X>
where
X: Pixel<Subpixel = u8> + 'static {
    fn fill_color(&self, x: f64, y: f64) -> X {
        let p = ((x - self.start.0) * self.cos + (y - self.start.1) * self.sin) / self.d;
        let p = p.min(1.0).max(0.0);
        gradient(&self.points, p)
    }
}

pub fn blend_pixel<X>(p1: X, p2: X, r: f64) -> X
where
    X: Pixel<Subpixel = u8> + 'static,
{
    p1.map2(&p2, |a, b| (a as f64 * (1.0 - r) + b as f64 * r).round() as u8)
}
