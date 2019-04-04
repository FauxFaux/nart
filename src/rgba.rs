use cast::u32;

pub struct RgbaVec {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RgbaVec {
    pub fn from_packed(packed: u32) -> RgbaVec {
        RgbaVec {
            r: (packed >> 0) as u8,
            g: (packed >> 8) as u8,
            b: (packed >> 16) as u8,
            a: (packed >> 24) as u8,
        }
    }

    pub fn to_packed(&self) -> u32 {
        (u32(self.r) << 0) + (u32(self.g) << 8) + (u32(self.b) << 16) + (u32(self.a) << 24)
    }

    pub fn blend_one_minus_src(&self, src: &RgbaVec) -> RgbaVec {
        RgbaVec {
            r: src.r + one_minus_a(src.a, self.r),
            g: src.g + one_minus_a(src.a, self.g),
            b: src.b + one_minus_a(src.a, self.b),
            a: self.a,
        }
    }
}

#[inline]
fn one_minus_a(a: u8, c: u8) -> u8 {
    ((1. - (a as f32) / 256.) * (c as f32)) as u8
}
