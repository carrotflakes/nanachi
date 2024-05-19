use super::*;
use crate::pixel::Rgba;

macro_rules! def_linear_compositor {
    (
        $name:ident ($aa:ident, $ba:ident => $ca:ident, $ax:ident, $bx:ident)
        {$($rest1:tt)+}
    ) => {
        impl Compositor<Rgba> for $name {
            type F1 = fn(&Rgba, &Rgba, f32) -> Rgba;
            type F2 = fn(&Rgba, &Rgba) -> Rgba;

            #[allow(unused_variables)]
            fn composite_with_alpha(&self) -> Self::F1 {
                |a, b, alpha| {
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

            #[allow(unused_variables)]
            fn composite(&self) -> Self::F2 {
                |a, b| {
                    let $aa = a.0[3];
                    let $ba = b.0[3];
                    $($rest1)+
                    Rgba([
                        a.0[0] * $ax + b.0[0] * $bx,
                        a.0[1] * $ax + b.0[1] * $bx,
                        a.0[2] * $ax + b.0[2] * $bx,
                        $ca,
                    ])
                }
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
            type F1 = fn(&Rgba, &Rgba, f32) -> Rgba;
            type F2 = fn(&Rgba, &Rgba) -> Rgba;

            #[allow(unused_variables)]
            fn composite_with_alpha(&self) -> Self::F1 {
                |a, b, alpha| {
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

            #[allow(unused_variables)]
            fn composite(&self) -> Self::F2 {
                |a, b| {
                    let aa = a.0[3];
                    let ba = b.0[3];

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
    type F1 = fn(&Rgba, &Rgba, f32) -> Rgba;
    type F2 = fn(&Rgba, &Rgba) -> Rgba;

    fn composite(&self) -> Self::F2 {
        match self {
            Basic::Clear => Compositor::<Rgba>::composite(&Clear),
            Basic::Src => Compositor::<Rgba>::composite(&Src),
            Basic::Dst => Compositor::<Rgba>::composite(&Dst),
            Basic::SrcOver => Compositor::<Rgba>::composite(&SrcOver),
            Basic::SrcIn => Compositor::<Rgba>::composite(&SrcIn),
            Basic::SrcOut => Compositor::<Rgba>::composite(&SrcOut),
            Basic::SrcAtop => Compositor::<Rgba>::composite(&SrcAtop),
            Basic::DstOver => Compositor::<Rgba>::composite(&DstOver),
            Basic::DstIn => Compositor::<Rgba>::composite(&DstIn),
            Basic::DstOut => Compositor::<Rgba>::composite(&DstOut),
            Basic::DstAtop => Compositor::<Rgba>::composite(&DstAtop),
            Basic::Xor => Compositor::<Rgba>::composite(&Xor),
            Basic::Add => Compositor::<Rgba>::composite(&Add),
            Basic::Darken => Compositor::<Rgba>::composite(&Darken),
            Basic::Lighten => Compositor::<Rgba>::composite(&Lighten),
            Basic::Multiply => Compositor::<Rgba>::composite(&Multiply),
            Basic::Screen => Compositor::<Rgba>::composite(&Screen),
            Basic::Overlay => Compositor::<Rgba>::composite(&Overlay),
            Basic::HardLight => Compositor::<Rgba>::composite(&HardLight),
            Basic::Dodge => Compositor::<Rgba>::composite(&Dodge),
            Basic::Burn => Compositor::<Rgba>::composite(&Burn),
            Basic::SoftLight => Compositor::<Rgba>::composite(&SoftLight),
            Basic::Difference => Compositor::<Rgba>::composite(&Difference),
            Basic::Exclusion => Compositor::<Rgba>::composite(&Exclusion),
        }
    }

    fn composite_with_alpha(&self) -> Self::F1 {
        match self {
            Basic::Clear => Compositor::<Rgba>::composite_with_alpha(&Clear),
            Basic::Src => Compositor::<Rgba>::composite_with_alpha(&Src),
            Basic::Dst => Compositor::<Rgba>::composite_with_alpha(&Dst),
            Basic::SrcOver => Compositor::<Rgba>::composite_with_alpha(&SrcOver),
            Basic::SrcIn => Compositor::<Rgba>::composite_with_alpha(&SrcIn),
            Basic::SrcOut => Compositor::<Rgba>::composite_with_alpha(&SrcOut),
            Basic::SrcAtop => Compositor::<Rgba>::composite_with_alpha(&SrcAtop),
            Basic::DstOver => Compositor::<Rgba>::composite_with_alpha(&DstOver),
            Basic::DstIn => Compositor::<Rgba>::composite_with_alpha(&DstIn),
            Basic::DstOut => Compositor::<Rgba>::composite_with_alpha(&DstOut),
            Basic::DstAtop => Compositor::<Rgba>::composite_with_alpha(&DstAtop),
            Basic::Xor => Compositor::<Rgba>::composite_with_alpha(&Xor),
            Basic::Add => Compositor::<Rgba>::composite_with_alpha(&Add),
            Basic::Darken => Compositor::<Rgba>::composite_with_alpha(&Darken),
            Basic::Lighten => Compositor::<Rgba>::composite_with_alpha(&Lighten),
            Basic::Multiply => Compositor::<Rgba>::composite_with_alpha(&Multiply),
            Basic::Screen => Compositor::<Rgba>::composite_with_alpha(&Screen),
            Basic::Overlay => Compositor::<Rgba>::composite_with_alpha(&Overlay),
            Basic::HardLight => Compositor::<Rgba>::composite_with_alpha(&HardLight),
            Basic::Dodge => Compositor::<Rgba>::composite_with_alpha(&Dodge),
            Basic::Burn => Compositor::<Rgba>::composite_with_alpha(&Burn),
            Basic::SoftLight => Compositor::<Rgba>::composite_with_alpha(&SoftLight),
            Basic::Difference => Compositor::<Rgba>::composite_with_alpha(&Difference),
            Basic::Exclusion => Compositor::<Rgba>::composite_with_alpha(&Exclusion),
        }
    }
}
