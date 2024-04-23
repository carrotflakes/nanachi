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
    fn composite(&self, dst: &T, src: &T, alpha: f32) -> T;
}

#[derive(Clone)]
pub struct Clear;
#[derive(Clone)]
pub struct Src;
#[derive(Clone)]
pub struct Dst;
#[derive(Clone)]
pub struct SrcOver;
#[derive(Clone)]
pub struct SrcIn;
#[derive(Clone)]
pub struct SrcOut;
#[derive(Clone)]
pub struct SrcAtop;
#[derive(Clone)]
pub struct DstOver;
#[derive(Clone)]
pub struct DstIn;
#[derive(Clone)]
pub struct DstOut;
#[derive(Clone)]
pub struct DstAtop;
#[derive(Clone)]
pub struct Xor;
#[derive(Clone)]
pub struct Add;
#[derive(Clone)]
pub struct Darken;
#[derive(Clone)]
pub struct Lighten;
#[derive(Clone)]
pub struct Multiply;
#[derive(Clone)]
pub struct Screen;
#[derive(Clone)]
pub struct Overlay;
#[derive(Clone)]
pub struct HardLight;
#[derive(Clone)]
pub struct Dodge;
#[derive(Clone)]
pub struct Burn;
#[derive(Clone)]
pub struct SoftLight;
#[derive(Clone)]
pub struct Difference;
#[derive(Clone)]
pub struct Exclusion;

/// Dynamically composition type.
#[derive(Clone)]
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

impl CompositorAttr for Clear {
    fn keep_dst_on_transparent_src(&self) -> bool {
        false
    }
}
impl CompositorAttr for Src {
    fn keep_dst_on_transparent_src(&self) -> bool {
        false
    }
}
impl CompositorAttr for Dst {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for SrcOver {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for SrcIn {
    fn keep_dst_on_transparent_src(&self) -> bool {
        false
    }
}
impl CompositorAttr for SrcOut {
    fn keep_dst_on_transparent_src(&self) -> bool {
        false
    }
}
impl CompositorAttr for SrcAtop {
    fn keep_dst_on_transparent_src(&self) -> bool {
        false
    }
}
impl CompositorAttr for DstOver {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for DstIn {
    fn keep_dst_on_transparent_src(&self) -> bool {
        false
    }
}
impl CompositorAttr for DstOut {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for DstAtop {
    fn keep_dst_on_transparent_src(&self) -> bool {
        false
    }
}
impl CompositorAttr for Xor {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Add {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Darken {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Lighten {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Multiply {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Screen {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Overlay {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for HardLight {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Dodge {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Burn {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for SoftLight {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Difference {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}
impl CompositorAttr for Exclusion {
    fn keep_dst_on_transparent_src(&self) -> bool {
        true
    }
}

impl CompositorAttr for Basic {
    fn keep_dst_on_transparent_src(&self) -> bool {
        match self {
            Basic::Clear => Clear.keep_dst_on_transparent_src(),
            Basic::Src => Src.keep_dst_on_transparent_src(),
            Basic::Dst => Dst.keep_dst_on_transparent_src(),
            Basic::SrcOver => SrcOver.keep_dst_on_transparent_src(),
            Basic::SrcIn => SrcIn.keep_dst_on_transparent_src(),
            Basic::SrcOut => SrcOut.keep_dst_on_transparent_src(),
            Basic::SrcAtop => SrcAtop.keep_dst_on_transparent_src(),
            Basic::DstOver => DstOver.keep_dst_on_transparent_src(),
            Basic::DstIn => DstIn.keep_dst_on_transparent_src(),
            Basic::DstOut => DstOut.keep_dst_on_transparent_src(),
            Basic::DstAtop => DstAtop.keep_dst_on_transparent_src(),
            Basic::Xor => Xor.keep_dst_on_transparent_src(),
            Basic::Add => Add.keep_dst_on_transparent_src(),
            Basic::Darken => Darken.keep_dst_on_transparent_src(),
            Basic::Lighten => Lighten.keep_dst_on_transparent_src(),
            Basic::Multiply => Multiply.keep_dst_on_transparent_src(),
            Basic::Screen => Screen.keep_dst_on_transparent_src(),
            Basic::Overlay => Overlay.keep_dst_on_transparent_src(),
            Basic::HardLight => HardLight.keep_dst_on_transparent_src(),
            Basic::Dodge => Dodge.keep_dst_on_transparent_src(),
            Basic::Burn => Burn.keep_dst_on_transparent_src(),
            Basic::SoftLight => SoftLight.keep_dst_on_transparent_src(),
            Basic::Difference => Difference.keep_dst_on_transparent_src(),
            Basic::Exclusion => Exclusion.keep_dst_on_transparent_src(),
        }
    }
}

/// For Measuring performance.
pub mod perf {
    use super::*;
    use crate::pixel::Rgba;

    #[derive(Clone)]
    pub struct Perf;

    impl CompositorAttr for Perf {
        fn keep_dst_on_transparent_src(&self) -> bool {
            false
        }
    }

    #[cfg(feature = "image-crate")]
    impl Compositor<image::Rgba<u8>> for Perf {
        #[allow(unused_variables)]
        fn composite(
            &self,
            a: &image::Rgba<u8>,
            b: &image::Rgba<u8>,
            alpha: f32,
        ) -> image::Rgba<u8> {
            image::Rgba([a.0[0], b.0[0], alpha as u8, 255])
        }
    }

    #[cfg(feature = "image-crate")]
    impl Compositor<image::Rgb<u8>> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &image::Rgb<u8>, b: &image::Rgb<u8>, alpha: f32) -> image::Rgb<u8> {
            image::Rgb([a.0[0], b.0[0], alpha as u8])
        }
    }

    impl Compositor<Rgba> for Perf {
        #[allow(unused_variables)]
        fn composite(&self, a: &Rgba, b: &Rgba, alpha: f32) -> Rgba {
            Rgba([a.0[0], b.0[0], alpha as f32, 1.0])
        }
    }
}
