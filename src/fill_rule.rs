//! [`FillRule`] controls the area of path filling.

/// FillRule Trait
pub trait FillRule: Copy {
    fn apply(&self, value: f32) -> f32;
    fn is_inverse(&self) -> bool;
}

/// NonZero fills the overlapping part of the path
#[derive(Clone, Copy, Default)]
pub struct NonZero;

impl FillRule for NonZero {
    fn apply(&self, value: f32) -> f32 {
        value.abs().min(1.0)
    }

    fn is_inverse(&self) -> bool {
        false
    }
}

/// EvenOdd fills only the odd overlap of the path
#[derive(Clone, Copy, Default)]
pub struct EvenOdd;

impl FillRule for EvenOdd {
    fn apply(&self, value: f32) -> f32 {
        1.0 - (value.rem_euclid(2.0) - 1.0).abs()
    }

    fn is_inverse(&self) -> bool {
        false
    }
}

/// Negative of [`NonZero`]
#[derive(Clone, Copy, Default)]
pub struct InverseNonZero;

impl FillRule for InverseNonZero {
    fn apply(&self, value: f32) -> f32 {
        1.0 - value.abs().min(1.0)
    }

    fn is_inverse(&self) -> bool {
        true
    }
}

/// Negative of [`EvenOdd`]
#[derive(Clone, Copy, Default)]
pub struct InverseEvenOdd;

impl FillRule for InverseEvenOdd {
    fn apply(&self, value: f32) -> f32 {
        (value.rem_euclid(2.0) - 1.0).abs()
    }

    fn is_inverse(&self) -> bool {
        true
    }
}

#[derive(Clone, Copy, Default)]
pub struct Abs;

impl FillRule for Abs {
    fn apply(&self, value: f32) -> f32 {
        value.abs()
    }

    fn is_inverse(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy, Default)]
pub struct Raw;

impl FillRule for Raw {
    fn apply(&self, value: f32) -> f32 {
        value
    }

    fn is_inverse(&self) -> bool {
        false
    }
}

#[test]
fn test() {
    assert!((NonZero.apply(0.0) - 0.0).abs() < 0.000001);
    assert!((NonZero.apply(0.6) - 0.6).abs() < 0.000001);
    assert!((NonZero.apply(1.0) - 1.0).abs() < 0.000001);
    assert!((NonZero.apply(1.2) - 1.0).abs() < 0.000001);
    assert!((NonZero.apply(-1.0) - 1.0).abs() < 0.000001);
    assert!((NonZero.apply(-1.2) - 1.0).abs() < 0.000001);

    assert!((EvenOdd.apply(0.0) - 0.0).abs() < 0.000001);
    assert!((EvenOdd.apply(0.6) - 0.6).abs() < 0.000001);
    assert!((EvenOdd.apply(1.0) - 1.0).abs() < 0.000001);
    assert!((EvenOdd.apply(1.2) - 0.8).abs() < 0.000001);
    assert!((EvenOdd.apply(-1.0) - 1.0).abs() < 0.000001);
    assert!((EvenOdd.apply(-1.2) - 0.8).abs() < 0.000001);

    assert!((InverseNonZero.apply(0.0) - 1.0).abs() < 0.000001);
    assert!((InverseNonZero.apply(0.6) - 0.4).abs() < 0.000001);
    assert!((InverseNonZero.apply(1.0) - 0.0).abs() < 0.000001);
    assert!((InverseNonZero.apply(1.2) - 0.0).abs() < 0.000001);
    assert!((InverseNonZero.apply(-1.0) - 0.0).abs() < 0.000001);
    assert!((InverseNonZero.apply(-1.2) - 0.0).abs() < 0.000001);

    assert!((InverseEvenOdd.apply(0.0) - 1.0).abs() < 0.000001);
    assert!((InverseEvenOdd.apply(0.6) - 0.4).abs() < 0.000001);
    assert!((InverseEvenOdd.apply(1.0) - 0.0).abs() < 0.000001);
    assert!((InverseEvenOdd.apply(1.2) - 0.2).abs() < 0.000001);
    assert!((InverseEvenOdd.apply(-1.0) - 0.0).abs() < 0.000001);
    assert!((InverseEvenOdd.apply(-1.2) - 0.2).abs() < 0.000001);
}
