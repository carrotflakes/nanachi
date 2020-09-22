#[cfg(feature = "image-crate")]
pub mod basic_f32;
#[cfg(feature = "image-crate")]
pub mod image_rgb_rgba;

pub trait Compositor<T> {
    fn composite(&self, dst: &T, src: &T, alpha: f64) -> T;
}

pub struct Clear;
pub struct Src;
pub struct Dst;
pub struct SrcOver;
pub struct SrcIn;
pub struct SrcOut;
pub struct SrcAtop;
pub struct DstOver;
pub struct DstIn;
pub struct DstOut;
pub struct DstAtop;
pub struct Xor;
pub struct Add;
pub struct Darken;
pub struct Lighten;
pub struct Multiply;
pub struct Screen;
pub struct Overlay;
pub struct HardLight;
pub struct Dodge;
pub struct Burn;
pub struct SoftLight;
pub struct Difference;
pub struct Exclusion;

pub enum Basic {
    Clear,
    Src,
    Dst,
    SrcOver,
    SrcIn,
    SrcOut,
    SrcAtop,
    DstOver,
    DstIn,
    DstOut,
    DstAtop,
    Xor,
    Add,
    Darken,
    Lighten,
    Multiply,
    Screen,
    Overlay,
    HardLight,
    Dodge,
    Burn,
    SoftLight,
    Difference,
    Exclusion,
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

    impl Compositor<Rgba<f32>> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &Rgba<f32>, b: &Rgba<f32>, alpha: f64) -> Rgba<f32> {
            Rgba([a.0[0], b.0[0], alpha as f32, 1.0])
        }
    }
}
