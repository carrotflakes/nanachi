pub trait FillRule {
    fn apply(&self, value: f64) -> f64;
}

#[derive(Clone, Copy)]
pub struct NonZero;

impl FillRule for NonZero {
    fn apply(&self, value: f64) -> f64 {
        value.abs().min(1.0)
    }
}

#[derive(Clone, Copy)]
pub struct EvenOdd;

impl FillRule for EvenOdd {
    fn apply(&self, value: f64) -> f64 {
        1.0 - (value.rem_euclid(2.0) - 1.0).abs()
    }
}

#[derive(Clone, Copy)]
pub struct Abs;

impl FillRule for Abs {
    fn apply(&self, value: f64) -> f64 {
        value.abs()
    }
}

#[derive(Clone, Copy)]
pub struct Raw;

impl FillRule for Raw {
    fn apply(&self, value: f64) -> f64 {
        value
    }
}
