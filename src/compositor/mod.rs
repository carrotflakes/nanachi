//! Collection of composition types.

pub mod basic_f32;
pub mod basic_premultiplied_f32;
#[cfg(feature = "image-crate")]
pub mod image_rgb_rgba;

/// Compositor attributes
pub trait CompositorAttr {
    /// If true, the compositor requires updating destinations even alpha is zero.
    fn keep_dst_on_transparent_src(&self) -> bool;
}

/// Compositor composites two pixels with alpha value.
pub trait Compositor<T>: CompositorAttr {
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

/// Dynamically composition type.
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

impl CompositorAttr for Clear {fn keep_dst_on_transparent_src(&self) -> bool {false}}
impl CompositorAttr for Src {fn keep_dst_on_transparent_src(&self) -> bool {false}}
impl CompositorAttr for Dst {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for SrcOver {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for SrcIn {fn keep_dst_on_transparent_src(&self) -> bool {false}}
impl CompositorAttr for SrcOut {fn keep_dst_on_transparent_src(&self) -> bool {false}}
impl CompositorAttr for SrcAtop {fn keep_dst_on_transparent_src(&self) -> bool {false}}
impl CompositorAttr for DstOver {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for DstIn {fn keep_dst_on_transparent_src(&self) -> bool {false}}
impl CompositorAttr for DstOut {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for DstAtop {fn keep_dst_on_transparent_src(&self) -> bool {false}}
impl CompositorAttr for Xor {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Add {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Darken {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Lighten {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Multiply {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Screen {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Overlay {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for HardLight {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Dodge {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Burn {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for SoftLight {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Difference {fn keep_dst_on_transparent_src(&self) -> bool {true}}
impl CompositorAttr for Exclusion {fn keep_dst_on_transparent_src(&self) -> bool {true}}

impl CompositorAttr for Basic {
    fn keep_dst_on_transparent_src(&self) -> bool {
        use Basic::*;
        match self {
            Clear => Clear.keep_dst_on_transparent_src(),
            Src => Src.keep_dst_on_transparent_src(),
            Dst => Dst.keep_dst_on_transparent_src(),
            SrcOver => SrcOver.keep_dst_on_transparent_src(),
            SrcIn => SrcIn.keep_dst_on_transparent_src(),
            SrcOut => SrcOut.keep_dst_on_transparent_src(),
            SrcAtop => SrcAtop.keep_dst_on_transparent_src(),
            DstOver => DstOver.keep_dst_on_transparent_src(),
            DstIn => DstIn.keep_dst_on_transparent_src(),
            DstOut => DstOut.keep_dst_on_transparent_src(),
            DstAtop => DstAtop.keep_dst_on_transparent_src(),
            Xor => Xor.keep_dst_on_transparent_src(),
            Add => Add.keep_dst_on_transparent_src(),
            Darken => Darken.keep_dst_on_transparent_src(),
            Lighten => Lighten.keep_dst_on_transparent_src(),
            Multiply => Multiply.keep_dst_on_transparent_src(),
            Screen => Screen.keep_dst_on_transparent_src(),
            Overlay => Overlay.keep_dst_on_transparent_src(),
            HardLight => HardLight.keep_dst_on_transparent_src(),
            Dodge => Dodge.keep_dst_on_transparent_src(),
            Burn => Burn.keep_dst_on_transparent_src(),
            SoftLight => SoftLight.keep_dst_on_transparent_src(),
            Difference => Difference.keep_dst_on_transparent_src(),
            Exclusion => Exclusion.keep_dst_on_transparent_src(),
        }
    }
}

/// For Measuring performance.
pub mod perf {
    use crate::pixel::Rgba;
    use super::*;

    pub struct Perf;

    impl CompositorAttr for Perf {
        fn keep_dst_on_transparent_src(&self) -> bool {
            false
        }
    }

    #[cfg(feature = "image-crate")]
    impl Compositor<image::Rgba<u8>> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &image::Rgba<u8>, b: &image::Rgba<u8>, alpha: f64) -> image::Rgba<u8> {
            image::Rgba([a.0[0], b.0[0], alpha as u8, 255])
        }
    }

    #[cfg(feature = "image-crate")]
    impl Compositor<image::Rgb<u8>> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &image::Rgb<u8>, b: &image::Rgb<u8>, alpha: f64) -> image::Rgb<u8> {
            image::Rgb([a.0[0], b.0[0], alpha as u8])
        }
    }

    impl Compositor<Rgba> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &Rgba, b: &Rgba, alpha: f64) -> Rgba {
            Rgba([a.0[0], b.0[0], alpha as f32, 1.0])
        }
    }
}
