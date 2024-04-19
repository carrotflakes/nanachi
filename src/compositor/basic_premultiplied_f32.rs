use super::*;
use crate::pixel::PremultipliedRgba;

macro_rules! def_linear_compositor {
    (
        $name:ident ($aa:ident, $ba:ident => $ca:ident, $ax:ident, $bx:ident)
        {$($rest1:tt)+}
    ) => {
        impl Compositor<PremultipliedRgba> for $name {
            #[allow(unused_variables)]
            fn composite(&self, a: &PremultipliedRgba, b: &PremultipliedRgba, alpha: f32) -> PremultipliedRgba {
                let $aa = a.0[3];
                let $ba = b.0[3] * alpha as f32;
                $($rest1)+
                PremultipliedRgba([
                    a.0[0] * $ax + b.0[0] * alpha as f32 * $bx,
                    a.0[1] * $ax + b.0[1] * alpha as f32 * $bx,
                    a.0[2] * $ax + b.0[2] * alpha as f32 * $bx,
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
        let ax = 1.0 - b;
        let bx = 1.0;
    }
}

def_linear_compositor! {
    SrcIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = 0.0;
        let bx = b;
    }
}

def_linear_compositor! {
    SrcOut(a, b => c, ax, bx) {
        let c = (1.0 - a) * b;
        let ax = 0.0;
        let bx = 1.0 - a;
    }
}

def_linear_compositor! {
    SrcAtop(a, b => c, ax, bx) {
        if b == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let c = b;
        let ax = if a != 0.0 {(1.0 - b) * b / a} else {0.0};
        let bx = b;
    }
}

def_linear_compositor! {
    DstOver(a, b => c, ax, bx) {
        let c = a + b - a * b;
        if c == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = 1.0;
        let bx = 1.0 - a;
    }
}

def_linear_compositor! {
    DstIn(a, b => c, ax, bx) {
        let c = a * b;
        let ax = a;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    DstOut(a, b => c, ax, bx) {
        let c = a * (1.0 - b);
        let ax = 1.0 - b;
        let bx = 0.0;
    }
}

def_linear_compositor! {
    DstAtop(a, b => c, ax, bx) {
        if a == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let c = a;
        let ax = a;
        let bx = if b != 0.0 {(1.0 - a) * a / b} else {0.0};
    }
}

def_linear_compositor! {
    Xor(a, b => c, ax, bx) {
        let c = a + b - 2.0 * a * b;
        let ax = (1.0 - b) * c;
        let bx = (1.0 - a) * c;
    }
}

def_linear_compositor! {
    Add(a, b => c, ax, bx) {
        let c = (a + b).min(1.0);
        if c == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = 1.0;
        let bx = 1.0;
    }
}

macro_rules! def_compositor {
    (
        $name:ident ($a:ident, $b:ident, $aa:ident, $ba:ident, $ca:ident, ($a0:ident, $a1:ident, $a2:ident, $b0:ident, $b1:ident, $b2:ident))
        {$($rest1:tt)+} [$($rest2:expr,)+]
    ) => {
        impl Compositor<PremultipliedRgba> for $name {
            #[allow(unused_variables)]
            fn composite(&self, $a: &PremultipliedRgba, $b: &PremultipliedRgba, alpha: f32) -> PremultipliedRgba {
                let alpha = alpha as f32;
                let $aa = $a.0[3];
                let $ba = $b.0[3] * alpha;
                $($rest1)+
                let ($a0, $a1, $a2, $b0, $b1, $b2) = ($a.0[0] as f32, $a.0[1] as f32, $a.0[2] as f32, $b.0[0] as f32 * alpha, $b.0[1] as f32 * alpha, $b.0[2] as f32 * alpha);
                PremultipliedRgba([
                    $($rest2,)+
                    $ca,
                ])
            }
        }
    };
}

def_compositor! {
    Darken(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
        let aaba = aa * ba;
    } [
        a0 * ax + b0 * bx + if aaba == 0.0 {0.0} else {(a0 / aa).min(b0 / ba) * aaba},
        a1 * ax + b1 * bx + if aaba == 0.0 {0.0} else {(a1 / aa).min(b1 / ba) * aaba},
        a2 * ax + b2 * bx + if aaba == 0.0 {0.0} else {(a2 / aa).min(b2 / ba) * aaba},
    ]
}

def_compositor! {
    Lighten(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
        let aaba = aa * ba;
    } [
        a0 * ax + b0 * bx + if aaba == 0.0 {0.0} else {(a0 / aa).max(b0 / ba) * aaba},
        a1 * ax + b1 * bx + if aaba == 0.0 {0.0} else {(a1 / aa).max(b1 / ba) * aaba},
        a2 * ax + b2 * bx + if aaba == 0.0 {0.0} else {(a2 / aa).max(b2 / ba) * aaba},
    ]
}

def_compositor! {
    Multiply(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
    } [
        a0 * ax + b0 * bx + a0 * b0,
        a1 * ax + b1 * bx + a1 * b1,
        a2 * ax + b2 * bx + a2 * b2,
    ]
}

def_compositor! {
    Screen(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
    } [
        a0 * ax + b0 * bx + (a0 * ba + b0 * aa - a0 * b0),
        a1 * ax + b1 * bx + (a1 * ba + b1 * aa - a1 * b1),
        a2 * ax + b2 * bx + (a2 * ba + b2 * aa - a2 * b2),
    ]
}

def_compositor! {
    Overlay(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
        let aaba = aa * ba;
    } [
        a0 * ax + b0 * bx + if a0 < 0.5 * aa {2.0 * a0 * b0} else {aaba - 2.0 * (aa - a0) * (ba - b0)},
        a1 * ax + b1 * bx + if a1 < 0.5 * aa {2.0 * a1 * b1} else {aaba - 2.0 * (aa - a1) * (ba - b1)},
        a2 * ax + b2 * bx + if a2 < 0.5 * aa {2.0 * a2 * b2} else {aaba - 2.0 * (aa - a2) * (ba - b2)},
    ]
}

def_compositor! {
    HardLight(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
        let aaba = aa * ba;
    } [
        a0 * ax + b0 * bx + if b0 < 0.5 * ba {2.0 * a0 * b0} else {aaba - 2.0 * (aa - a0) * (ba - b0)},
        a1 * ax + b1 * bx + if b1 < 0.5 * ba {2.0 * a1 * b1} else {aaba - 2.0 * (aa - a1) * (ba - b1)},
        a2 * ax + b2 * bx + if b2 < 0.5 * ba {2.0 * a2 * b2} else {aaba - 2.0 * (aa - a2) * (ba - b2)},
    ]
}

def_compositor! {
    Dodge(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
        let aaba = aa * ba;
    } [
        a0 * ax + b0 * bx + if b0 < ba {(ba * a0 / (1.0 - b0 / ba)).min(aaba)} else {aaba},
        a1 * ax + b1 * bx + if b1 < ba {(ba * a1 / (1.0 - b1 / ba)).min(aaba)} else {aaba},
        a2 * ax + b2 * bx + if b2 < ba {(ba * a2 / (1.0 - b2 / ba)).min(aaba)} else {aaba},
    ]
}

def_compositor! {
    Burn(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
        let aaba = aa * ba;
    } [
        a0 * ax + b0 * bx + if 0.0 < b0 {aaba - (ba.powi(2) * (aa - a0) / b0).min(aaba)} else {0.0},
        a1 * ax + b1 * bx + if 0.0 < b1 {aaba - (ba.powi(2) * (aa - a1) / b1).min(aaba)} else {0.0},
        a2 * ax + b2 * bx + if 0.0 < b2 {aaba - (ba.powi(2) * (aa - a2) / b2).min(aaba)} else {0.0},
    ]
}

def_compositor! {
    SoftLight(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
        fn f(a: f32, b: f32) -> f32 {
            if 0.5 < b {
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
    } [
        a0 * ax + b0 * bx + f(a0 / aa.max(0.001), b0 / ba.max(0.001)) * aa * ba,
        a1 * ax + b1 * bx + f(a1 / aa.max(0.001), b1 / ba.max(0.001)) * aa * ba,
        a2 * ax + b2 * bx + f(a2 / aa.max(0.001), b2 / ba.max(0.001)) * aa * ba,
    ]
}

def_compositor! {
    Difference(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
    } [
        a0 * ax + b0 * bx + (a0 * ba - b0 * aa).abs(),
        a1 * ax + b1 * bx + (a1 * ba - b1 * aa).abs(),
        a2 * ax + b2 * bx + (a2 * ba - b2 * aa).abs(),
    ]
}

def_compositor! {
    Exclusion(a, b, aa, ba, ca, (a0, a1, a2, b0, b1, b2)) {
        let ca = aa + ba - aa * ba;
        if ca == 0.0 {
            return PremultipliedRgba([0.0, 0.0, 0.0, 0.0]);
        }
        let ax = aa * (1.0 - ba) * aa;
        let bx = ba * (1.0 - aa) * ba;
    } [
        a0 * ax + b0 * bx + (a0 * ba + b0 * aa - 2.0 * a0 * b0),
        a1 * ax + b1 * bx + (a1 * ba + b1 * aa - 2.0 * a1 * b1),
        a2 * ax + b2 * bx + (a2 * ba + b2 * aa - 2.0 * a2 * b2),
    ]
}

impl Compositor<PremultipliedRgba> for Basic {
    fn composite(
        &self,
        a: &PremultipliedRgba,
        b: &PremultipliedRgba,
        alpha: f32,
    ) -> PremultipliedRgba {
        use Basic::*;
        match self {
            Clear => Clear.composite(a, b, alpha),
            Src => Src.composite(a, b, alpha),
            Dst => Dst.composite(a, b, alpha),
            SrcOver => SrcOver.composite(a, b, alpha),
            SrcIn => SrcIn.composite(a, b, alpha),
            SrcOut => SrcOut.composite(a, b, alpha),
            SrcAtop => SrcAtop.composite(a, b, alpha),
            DstOver => DstOver.composite(a, b, alpha),
            DstIn => DstIn.composite(a, b, alpha),
            DstOut => DstOut.composite(a, b, alpha),
            DstAtop => DstAtop.composite(a, b, alpha),
            Xor => Xor.composite(a, b, alpha),
            Add => Add.composite(a, b, alpha),
            Darken => Darken.composite(a, b, alpha),
            Lighten => Lighten.composite(a, b, alpha),
            Multiply => Multiply.composite(a, b, alpha),
            Screen => Screen.composite(a, b, alpha),
            Overlay => Overlay.composite(a, b, alpha),
            HardLight => HardLight.composite(a, b, alpha),
            Dodge => Dodge.composite(a, b, alpha),
            Burn => Burn.composite(a, b, alpha),
            SoftLight => SoftLight.composite(a, b, alpha),
            Difference => Difference.composite(a, b, alpha),
            Exclusion => Exclusion.composite(a, b, alpha),
        }
    }
}
