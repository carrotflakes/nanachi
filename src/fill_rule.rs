//! [`FillRule`] controls the area of path filling.
//! Major [`FillRule`] is [`NonZero`] and [`EvenOdd`].

pub trait FillRule: Copy {
    fn apply(&self, value: f64) -> f64;
}

#[derive(Clone, Copy, Default)]
pub struct NonZero;

impl FillRule for NonZero {
    fn apply(&self, value: f64) -> f64 {
        value.abs().min(1.0)
    }
}

#[derive(Clone, Copy, Default)]
pub struct EvenOdd;

impl FillRule for EvenOdd {
    fn apply(&self, value: f64) -> f64 {
        1.0 - (value.rem_euclid(2.0) - 1.0).abs()
    }
}

#[derive(Clone, Copy, Default)]
pub struct InverseNonZero;

impl FillRule for InverseNonZero {
    fn apply(&self, value: f64) -> f64 {
        1.0 - value.abs().min(1.0)
    }
}

#[derive(Clone, Copy, Default)]
pub struct InverseEvenOdd;

impl FillRule for InverseEvenOdd {
    fn apply(&self, value: f64) -> f64 {
        (value.rem_euclid(2.0) - 1.0).abs()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Abs;

impl FillRule for Abs {
    fn apply(&self, value: f64) -> f64 {
        value.abs()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Raw;

impl FillRule for Raw {
    fn apply(&self, value: f64) -> f64 {
        value
    }
}
