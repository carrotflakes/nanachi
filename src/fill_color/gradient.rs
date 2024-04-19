use crate::fill_color::FillColor;
use crate::pixel::Pixel;
use crate::point::Point;

type GradientPoint<P> = (f32, P);

fn gradient<P: Pixel>(points: &Vec<GradientPoint<P>>, p: f32) -> P {
    if p <= points[0].0 {
        return points[0].1.clone();
    }
    for i in 0..points.len() - 1 {
        let right = &points[i + 1];
        if p <= right.0 {
            let left = &points[i];
            return left.1.lerp(&right.1, (p - left.0) / (right.0 - left.0));
        }
    }
    points.last().unwrap().1.clone()
}

/// Linear gradient.
///
/// # Examples
///
/// ```
/// use nanachi::pixel::Rgba;
/// let color = nanachi::fill_color::LinearGradient::new((0.0, 0.0), (100.0, 0.0), vec![
///     (0.0, Rgba([1.0, 0.0, 0.0, 1.0])),
///     (0.5, Rgba([0.0, 1.0, 0.0, 1.0])),
///     (1.0, Rgba([0.0, 0.0, 1.0, 1.0])),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct LinearGradient<P: Pixel> {
    start: Point,
    sin: f32,
    cos: f32,
    d: f32,
    points: Vec<GradientPoint<P>>,
}

impl<P: Pixel> LinearGradient<P> {
    pub fn new<T: Into<Point>>(
        start: T,
        end: T,
        points: Vec<GradientPoint<P>>,
    ) -> LinearGradient<P> {
        let start: Point = start.into();
        let end: Point = end.into();
        assert!(!points.is_empty());
        let d = (end - start).norm();
        LinearGradient {
            start,
            sin: (end.y() - start.y()) / d,
            cos: (end.x() - start.x()) / d,
            d,
            points,
        }
    }
}

impl<P: Pixel> FillColor<P> for LinearGradient<P> {
    fn fill_color(&self, pos: [f32; 2]) -> P {
        let p =
            ((pos[0] - self.start.x()) * self.cos + (pos[1] - self.start.y()) * self.sin) / self.d;
        gradient(&self.points, p)
    }
}

/// Radial gradient.
///
/// # Examples
///
/// ```
/// use nanachi::pixel::Rgba;
/// let color = nanachi::fill_color::RadialGradient::new((50.0, 50.0), 50.0, vec![
///     (0.0, Rgba([1.0, 0.0, 0.0, 1.0])),
///     (0.5, Rgba([0.0, 1.0, 0.0, 1.0])),
///     (1.0, Rgba([0.0, 0.0, 1.0, 1.0])),
/// ]);
/// ```
#[derive(Debug, Clone)]
pub struct RadialGradient<P: Pixel> {
    start: Point,
    radius: f32,
    points: Vec<GradientPoint<P>>,
}

impl<P: Pixel> RadialGradient<P> {
    pub fn new<T: Into<Point>>(
        start: T,
        radius: f32,
        points: Vec<GradientPoint<P>>,
    ) -> RadialGradient<P> {
        assert!(!points.is_empty());
        RadialGradient {
            start: start.into(),
            radius,
            points,
        }
    }
}

impl<P: Pixel> FillColor<P> for RadialGradient<P> {
    fn fill_color(&self, pos: [f32; 2]) -> P {
        let p = (Point::from(pos) - self.start).norm() / self.radius;
        gradient(&self.points, p)
    }
}

/// Conic gradient.
#[derive(Debug, Clone)]
pub struct ConicGradient<P: Pixel> {
    origin: Point,
    start_angle: f32,
    points: Vec<GradientPoint<P>>,
}

impl<P: Pixel> ConicGradient<P> {
    pub fn new<T: Into<Point>>(
        origin: T,
        start_angle: f32,
        points: Vec<GradientPoint<P>>,
    ) -> ConicGradient<P> {
        assert!(!points.is_empty());
        ConicGradient {
            origin: origin.into(),
            start_angle: (-start_angle).rem_euclid(std::f32::consts::TAU) + std::f32::consts::PI,
            points,
        }
    }
}

impl<P: Pixel> FillColor<P> for ConicGradient<P> {
    fn fill_color(&self, pos: [f32; 2]) -> P {
        let p = ((self.origin - Point::from(pos)).atan2() + self.start_angle)
            / std::f32::consts::TAU
            % 1.0;
        gradient(&self.points, p)
    }
}
