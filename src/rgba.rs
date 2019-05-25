#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rgba(u32);

impl Rgba {
    pub fn from_u8s(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba(
            (u32::from(r) << 0) + (u32::from(g) << 8) + (u32::from(b) << 16) + (u32::from(a) << 24),
        )
    }

    pub fn from_packed(val: u32) -> Rgba {
        Rgba(val)
    }

    pub fn grey(luma: u8, alpha: u8) -> Rgba {
        Rgba::from_u8s(luma, luma, luma, alpha)
    }

    pub fn black() -> Rgba {
        Rgba::from_u8s(0, 0, 0, 255)
    }

    pub fn packed(&self) -> u32 {
        self.0
    }

    pub fn r(&self) -> u8 {
        (self.0 >> 0) as u8
    }

    pub fn g(&self) -> u8 {
        (self.0 >> 8) as u8
    }

    pub fn b(&self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub fn a(&self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub fn blend_one_minus_src(&self, src: &Rgba) -> Rgba {
        Rgba::from_u8s(
            src.r() + one_minus_a(src.a(), self.r()),
            src.g() + one_minus_a(src.a(), self.g()),
            src.b() + one_minus_a(src.a(), self.b()),
            self.a(),
        )
    }
}

#[inline]
fn one_minus_a(a: u8, c: u8) -> u8 {
    ((1. - (a as f32) / 256.) * (c as f32)) as u8
}
