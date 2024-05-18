use super::*;
use crate::pixel::Rgba;

macro_rules! def_linear_compositor {
    (
        $name:ident ($aa:ident, $ba:ident => $ca:ident, $ax:ident, $bx:ident)
        {$($rest1:tt)+}
    ) => {
        impl Compositor<Rgba> for $name {
            #[allow(unused_variables)]
            fn composite_with_alpha(&self, a: &Rgba, b: &Rgba, alpha: f32) -> Rgba {
                let $aa = a.0[3];
                let $ba = b.0[3] * alpha;
                $($rest1)+
                Rgba([
                    a.0[0] * $ax + b.0[0] * $bx,
                    a.0[1] * $ax + b.0[1] * $bx,
                    a.0[2] * $ax + b.0[2] * $bx,
                    $ca,
                ])
            }
        }
    };
}

def_linear_compositor! {
    Clear(a, b => c, ax, bx) {
        let c = 0.0;
        let ax = 0.0;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    Src(a, b => c, ax, bx) {
        let c = b;
        let ax = 0.0;
        let bx = b;
    }
}

def_linear_compositor! {
    Dst(a, b => c, ax, bx) {
        let c = a;
        let ax = a;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    SrcOver(a, b => c, ax, bx) {
        let c = a + b - a * b;
        if c == 0.0 {
            return Rgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = (a * (1.0 - b)) / c;
        let bx = b / c;
    }
}

def_linear_compositor! {
    SrcIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = 0.0;
        let bx = 1.0;
    }
}

def_linear_compositor! {
    SrcOut(a, b => c, ax, bx) {
        let c = (1.0 - a) * b;
        let ax = 0.0;
        let bx = 1.0;
    }
}

def_linear_compositor! {
    SrcAtop(a, b => c, ax, bx) {
        let c = a;
        if c == 0.0 {
            return Rgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = 1.0 - b;
        let bx = b;
    }
}

def_linear_compositor! {
    DstOver(a, b => c, ax, bx) {
        let c = a + b - a * b;
        if c == 0.0 {
            return Rgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = a / c;
        let bx = ((1.0 - a) * b) / c;
    }
}

def_linear_compositor! {
    DstIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = 1.0;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    DstOut(a, b => c, ax, bx) {
        let c = a * (1.0 - b);
        let ax = 1.0;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    DstAtop(a, b => c, ax, bx) {
        let c = b;
        if c == 0.0 {
            return Rgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = a;
        let bx = 1.0 - a;
    }
}

def_linear_compositor! {
    Xor(a, b => c, ax, bx) {
        let ax = a * (1.0 - b);
        let bx = (1.0 - a) * b;
        let c = a + b - 2.0 * a * b;
    }
}

def_linear_compositor! {
    Add(a, b => c, ax, bx) {
        let c = (a + b).min(1.0);
        if c == 0.0 {
            return Rgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = a / c;
        let bx = b / c;
    }
}

macro_rules! def_compositor {
    (
        $name:ident
        $e:expr
    ) => {
        impl Compositor<Rgba> for $name {
            #[allow(unused_variables)]
            fn composite_with_alpha(&self, a: &Rgba, b: &Rgba, alpha: f32) -> Rgba {
                let aa = a.0[3];
                let ba = b.0[3] * alpha;

                let ca = aa + ba - aa * ba;
                if ca == 0.0 {
                    return Rgba([0.0, 0.0, 0.0, 0.0]);
                }
                let ax = (aa * (1.0 - ba)) / ca;
                let bx = (ba * (1.0 - aa)) / ca;
                let cx = aa * ba / ca;

                let e = $e;
                Rgba([
                    a.0[0] * ax + b.0[0] * bx + e(a.0[0], b.0[0]) * cx,
                    a.0[1] * ax + b.0[1] * bx + e(a.0[1], b.0[1]) * cx,
                    a.0[2] * ax + b.0[2] * bx + e(a.0[2], b.0[2]) * cx,
                    ca,
                ])
            }
        }
    };
}

def_compositor! {
    Darken
    {|a: f32, b: f32| a.min(b)}
}

def_compositor! {
    Lighten
    {|a: f32, b: f32| a.max(b)}
}

def_compositor! {
    Multiply
    {|a: f32, b: f32| a * b}
}

def_compositor! {
    Screen
    {|a: f32, b: f32| (a + b - a * b)}
}

def_compositor! {
    Overlay
    {|a: f32, b: f32| if a < 0.5 {2.0 * a * b} else {1.0 - 2.0 * (1.0 - a) * (1.0 - b)}}
}

def_compositor! {
    HardLight
    {|a: f32, b: f32| if b < 0.5 {2.0 * a * b} else {1.0 - 2.0 * (1.0 - a) * (1.0 - b)}}
}

def_compositor! {
    Dodge
    {|a: f32, b: f32| if b < 1.0 {(a / (1.0 - b)).min(1.0)} else {1.0}}
}

def_compositor! {
    Burn
    {|a: f32, b: f32| if 0.0 < b {1.0 - ((1.0 - a) / b).min(1.0)} else {0.0}}
}

def_compositor! {
    SoftLight
    {|a: f32, b: f32|
        if b < 0.5 {
            a - (1.0 - 2.0 * b) * a * (1.0 - a)
        } else {
            let g = if a < 0.25 {
                ((16.0 * a - 12.0) * a + 4.0) * a
            } else {
                a.sqrt()
            };
            a + (2.0 * b - 1.0) * (g - a)
        }
    }
}

def_compositor! {
    Difference
    {|a: f32, b: f32| (a - b).abs()}
}

def_compositor! {
    Exclusion
    {|a: f32, b: f32| (a + b - 2.0 * a * b)}
}

impl Compositor<Rgba> for Basic {
    fn composite(&self, dst: &Rgba, src: &Rgba) -> Rgba {
        self.composite_with_alpha(dst, src, 1.0)
    }

    fn composite_with_alpha(&self, a: &Rgba, b: &Rgba, alpha: f32) -> Rgba {
        match self {
            Basic::Clear => Clear.composite_with_alpha(a, b, alpha),
            Basic::Src => Src.composite_with_alpha(a, b, alpha),
            Basic::Dst => Dst.composite_with_alpha(a, b, alpha),
            Basic::SrcOver => SrcOver.composite_with_alpha(a, b, alpha),
            Basic::SrcIn => SrcIn.composite_with_alpha(a, b, alpha),
            Basic::SrcOut => SrcOut.composite_with_alpha(a, b, alpha),
            Basic::SrcAtop => SrcAtop.composite_with_alpha(a, b, alpha),
            Basic::DstOver => DstOver.composite_with_alpha(a, b, alpha),
            Basic::DstIn => DstIn.composite_with_alpha(a, b, alpha),
            Basic::DstOut => DstOut.composite_with_alpha(a, b, alpha),
            Basic::DstAtop => DstAtop.composite_with_alpha(a, b, alpha),
            Basic::Xor => Xor.composite_with_alpha(a, b, alpha),
            Basic::Add => Add.composite_with_alpha(a, b, alpha),
            Basic::Darken => Darken.composite_with_alpha(a, b, alpha),
            Basic::Lighten => Lighten.composite_with_alpha(a, b, alpha),
            Basic::Multiply => Multiply.composite_with_alpha(a, b, alpha),
            Basic::Screen => Screen.composite_with_alpha(a, b, alpha),
            Basic::Overlay => Overlay.composite_with_alpha(a, b, alpha),
            Basic::HardLight => HardLight.composite_with_alpha(a, b, alpha),
            Basic::Dodge => Dodge.composite_with_alpha(a, b, alpha),
            Basic::Burn => Burn.composite_with_alpha(a, b, alpha),
            Basic::SoftLight => SoftLight.composite_with_alpha(a, b, alpha),
            Basic::Difference => Difference.composite_with_alpha(a, b, alpha),
            Basic::Exclusion => Exclusion.composite_with_alpha(a, b, alpha),
        }
    }
}
