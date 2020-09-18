use crate::fill_color::FillColor;
use image::Pixel;

type GradientPoint<X> = (f64, X);

fn gradient<X>(points: &Vec<GradientPoint<X>>, p: f64) -> X
where
    X: Pixel<Subpixel = u8> + 'static,
{
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
    X: Pixel<Subpixel = u8> + 'static,
{
    start: (f64, f64),
    sin: f64,
    cos: f64,
    d: f64,
    points: Vec<GradientPoint<X>>,
}

impl<X> LinearGradient<X>
where
    X: Pixel<Subpixel = u8> + 'static,
{
    pub fn new(
        start: (f64, f64),
        end: (f64, f64),
        points: Vec<GradientPoint<X>>,
    ) -> LinearGradient<X> {
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
    X: Pixel<Subpixel = u8> + 'static,
{
    fn fill_color(&self, x: f64, y: f64) -> X {
        let p = ((x - self.start.0) * self.cos + (y - self.start.1) * self.sin) / self.d;
        gradient(&self.points, p)
    }
}

#[derive(Debug, Clone)]
pub struct RadialGradient<X>
where
    X: Pixel<Subpixel = u8> + 'static,
{
    start: (f64, f64),
    radius: f64,
    points: Vec<GradientPoint<X>>,
}

impl<X> RadialGradient<X>
where
    X: Pixel<Subpixel = u8> + 'static,
{
    pub fn new(start: (f64, f64), radius: f64, points: Vec<GradientPoint<X>>) -> RadialGradient<X> {
        assert!(!points.is_empty());
        RadialGradient {
            start,
            radius,
            points,
        }
    }
}

impl<X> FillColor<X> for RadialGradient<X>
where
    X: Pixel<Subpixel = u8> + 'static,
{
    fn fill_color(&self, x: f64, y: f64) -> X {
        let p = (x - self.start.0).hypot(y - self.start.1) / self.radius;
        gradient(&self.points, p)
    }
}

pub fn blend_pixel<X>(p1: X, p2: X, r: f64) -> X
where
    X: Pixel<Subpixel = u8> + 'static,
{
    p1.map2(&p2, |a, b| {
        (a as f64 * (1.0 - r) + b as f64 * r).round() as u8
    })
}
