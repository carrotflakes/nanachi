#[cfg(feature = "image-crate")]
pub mod basic;
#[cfg(feature = "image-crate")]
pub mod basic_enum;

pub trait Compositor<T> {
    fn composite(&self, dst: &T, src: &T, alpha: f64) -> T;
}

#[cfg(feature = "image-crate")]
pub mod perf {
    use super::*;
    use image::{Rgb, Rgba};

    pub struct Perf;

    impl Compositor<Rgba<u8>> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
            Rgba([a.0[0], b.0[0], alpha as u8, 255])
        }
    }

    impl Compositor<Rgb<u8>> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
            Rgb([a.0[0], b.0[0], alpha as u8])
        }
    }
}
