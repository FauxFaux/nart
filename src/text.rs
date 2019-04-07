use failure::Error;
use rusttype::point;
use rusttype::Point;
use rusttype::PositionedGlyph;
use rusttype::Scale;

pub struct Font {
    inner: rusttype::Font<'static>,
}

pub struct Layout<'f, 's> {
    font: &'f Font,
    text: &'s str,
    pub(super) glyphs: Vec<PositionedGlyph<'f>>,
}

impl Font {
    pub fn sans_noto() -> Font {
        let bytes = &include_bytes!("../assets/NotoSans-Regular.ttf")[..];
        Font {
            inner: rusttype::Font::from_bytes(bytes).expect("static data"),
        }
    }

    pub fn layout<'f, 's>(&'f self, text: &'s str, pt: f32) -> Layout<'f, 's> {
        let scale = Scale { x: pt, y: pt };
        let offset = point(0., self.inner.v_metrics(scale).ascent);
        Layout {
            font: self,
            text,
            glyphs: self.inner.layout(text, scale, offset).collect(),
        }
    }
}

impl<'f, 's> Layout<'f, 's> {
    pub fn width(&self) -> f32 {
        match self.glyphs.last() {
            None => 0.,
            Some(g) => g.position().x + g.unpositioned().h_metrics().advance_width,
        }
    }
}
