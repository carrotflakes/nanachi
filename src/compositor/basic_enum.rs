use super::basic;
use super::Compositor;
use image::{Rgb, Rgba};

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

impl Compositor<Rgba<u8>> for Basic {
    fn composite(&self, a: &Rgba<u8>, b: &Rgba<u8>, alpha: f64) -> Rgba<u8> {
        use Basic::*;
        match self {
            Clear => basic::Clear.composite(a, b, alpha),
            Src => basic::Src.composite(a, b, alpha),
            Dst => basic::Dst.composite(a, b, alpha),
            SrcOver => basic::SrcOver.composite(a, b, alpha),
            SrcIn => basic::SrcIn.composite(a, b, alpha),
            SrcOut => basic::SrcOut.composite(a, b, alpha),
            SrcAtop => basic::SrcAtop.composite(a, b, alpha),
            DstOver => basic::DstOver.composite(a, b, alpha),
            DstIn => basic::DstIn.composite(a, b, alpha),
            DstOut => basic::DstOut.composite(a, b, alpha),
            DstAtop => basic::DstAtop.composite(a, b, alpha),
            Xor => basic::Xor.composite(a, b, alpha),
            Add => basic::Add.composite(a, b, alpha),
            Darken => basic::Darken.composite(a, b, alpha),
            Lighten => basic::Lighten.composite(a, b, alpha),
            Multiply => basic::Multiply.composite(a, b, alpha),
            Screen => basic::Screen.composite(a, b, alpha),
            Overlay => basic::Overlay.composite(a, b, alpha),
            HardLight => basic::HardLight.composite(a, b, alpha),
            Dodge => basic::Dodge.composite(a, b, alpha),
            Burn => basic::Burn.composite(a, b, alpha),
            SoftLight => basic::SoftLight.composite(a, b, alpha),
            Difference => basic::Difference.composite(a, b, alpha),
            Exclusion => basic::Exclusion.composite(a, b, alpha),
        }
    }
}

impl Compositor<Rgb<u8>> for Basic {
    fn composite(&self, a: &Rgb<u8>, b: &Rgb<u8>, alpha: f64) -> Rgb<u8> {
        use Basic::*;
        match self {
            Clear => basic::Clear.composite(a, b, alpha),
            Src => basic::Src.composite(a, b, alpha),
            Dst => basic::Dst.composite(a, b, alpha),
            SrcOver => basic::SrcOver.composite(a, b, alpha),
            SrcIn => basic::SrcIn.composite(a, b, alpha),
            SrcOut => basic::SrcOut.composite(a, b, alpha),
            SrcAtop => basic::SrcAtop.composite(a, b, alpha),
            DstOver => basic::DstOver.composite(a, b, alpha),
            DstIn => basic::DstIn.composite(a, b, alpha),
            DstOut => basic::DstOut.composite(a, b, alpha),
            DstAtop => basic::DstAtop.composite(a, b, alpha),
            Xor => basic::Xor.composite(a, b, alpha),
            Add => basic::Add.composite(a, b, alpha),
            Darken => basic::Darken.composite(a, b, alpha),
            Lighten => basic::Lighten.composite(a, b, alpha),
            Multiply => basic::Multiply.composite(a, b, alpha),
            Screen => basic::Screen.composite(a, b, alpha),
            Overlay => basic::Overlay.composite(a, b, alpha),
            HardLight => basic::HardLight.composite(a, b, alpha),
            Dodge => basic::Dodge.composite(a, b, alpha),
            Burn => basic::Burn.composite(a, b, alpha),
            SoftLight => basic::SoftLight.composite(a, b, alpha),
            Difference => basic::Difference.composite(a, b, alpha),
            Exclusion => basic::Exclusion.composite(a, b, alpha),
        }
    }
}
