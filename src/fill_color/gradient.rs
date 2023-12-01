use crate::fill_color::FillColor;
use crate::pixel::Pixel;

type GradientPoint<P> = (f64, P);

fn gradient<P: Pixel>(points: &Vec<GradientPoint<P>>, p: f64) -> P {
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
    start: (f64, f64),
    sin: f64,
    cos: f64,
    d: f64,
    points: Vec<GradientPoint<P>>,
}

impl<P: Pixel> LinearGradient<P> {
    pub fn new(
        start: (f64, f64),
        end: (f64, f64),
        points: Vec<GradientPoint<P>>,
    ) -> LinearGradient<P> {
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

impl<P: Pixel> FillColor<P> for LinearGradient<P> {
    fn fill_color(&self, x: f64, y: f64) -> P {
        let p = ((x - self.start.0) * self.cos + (y - self.start.1) * self.sin) / self.d;
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
    start: (f64, f64),
    radius: f64,
    points: Vec<GradientPoint<P>>,
}

impl<P: Pixel> RadialGradient<P> {
    pub fn new(start: (f64, f64), radius: f64, points: Vec<GradientPoint<P>>) -> RadialGradient<P> {
        assert!(!points.is_empty());
        RadialGradient {
            start,
            radius,
            points,
        }
    }
}

impl<P: Pixel> FillColor<P> for RadialGradient<P> {
    fn fill_color(&self, x: f64, y: f64) -> P {
        let p = (x - self.start.0).hypot(y - self.start.1) / self.radius;
        gradient(&self.points, p)
    }
}

/// Conic gradient.
#[derive(Debug, Clone)]
pub struct ConicGradient<P: Pixel> {
    origin: (f64, f64),
    start_angle: f64,
    points: Vec<GradientPoint<P>>,
}

impl<P: Pixel> ConicGradient<P> {
    pub fn new(
        origin: (f64, f64),
        start_angle: f64,
        points: Vec<GradientPoint<P>>,
    ) -> ConicGradient<P> {
        assert!(!points.is_empty());
        ConicGradient {
            origin,
            start_angle: (-start_angle).rem_euclid(std::f64::consts::TAU) + std::f64::consts::PI,
            points,
        }
    }
}

impl<P: Pixel> FillColor<P> for ConicGradient<P> {
    fn fill_color(&self, x: f64, y: f64) -> P {
        let p = ((self.origin.1 - y).atan2(self.origin.0 - x) + self.start_angle)
            / std::f64::consts::TAU
            % 1.0;
        gradient(&self.points, p)
    }
}
