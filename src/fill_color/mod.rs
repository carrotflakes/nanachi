//! Collection of colorization types.
//!
//! - [`Solid`]
//! - [`LinearGradient`]
//! - [`RadialGradient`]
//! - [`Pattern`]
//! - [`Transform`]

pub mod gradient;
pub mod pattern;
pub mod transform;

pub use gradient::{ConicGradient, LinearGradient, RadialGradient};
pub use pattern::Pattern;
pub use transform::Transform;

/// A trait for Colorization.
/// Generates `Pixel` from (x, y).
pub trait FillColor<C> {
    fn fill_color(&self, x: f32, y: f32) -> C;
}

/// Solid color.
#[derive(Debug, Clone)]
pub struct Solid<C: Clone>(C);

impl<C: Clone> Solid<C> {
    pub fn new(c: C) -> Self {
        Solid(c)
    }
}

impl<C: Clone> FillColor<C> for Solid<C> {
    fn fill_color(&self, _: f32, _: f32) -> C {
        self.0.clone()
    }
}

/// Block check pattern.
#[derive(Debug, Clone)]
pub struct BlockCheck<C: Clone>(C, C, f32);

impl<C: Clone> BlockCheck<C> {
    pub fn new(c1: C, c2: C, size: f32) -> Self {
        BlockCheck(c1, c2, size)
    }
}

impl<C: Clone> FillColor<C> for BlockCheck<C> {
    fn fill_color(&self, x: f32, y: f32) -> C {
        if ((x / self.2) as isize + (y / self.2) as isize) % 2 == 0 {
            self.0.clone()
        } else {
            self.1.clone()
        }
    }
}
