use std::mem;
use std::slice;
use std::thread;
use std::time;

use cast::u32;
use cast::usize;
use failure::Error;
use minifb::Window;
use minifb::WindowOptions;

use crate::img::RgbaImage;
use crate::rgba::Rgba;
use std::intrinsics::transmute;

#[derive(Clone)]
pub struct NartOptions {
    pub name: String,
    pub width: usize,
    pub height: usize,
    pub resize: bool,
    pub frame_cap: usize,
}

pub struct Nart {
    pub win: Window,
    buffer: Vec<Rgba>,
    last_size: (usize, usize),
    last_frame: time::Instant,
    frame_ms: u32,
}

pub struct Buffer<'b> {
    inner: &'b mut [Rgba],
    width: usize,
    height: usize,
}

impl Nart {
    pub fn new(options: &NartOptions) -> Result<Nart, Error> {
        Ok(Nart {
            win: Window::new(
                &options.name,
                options.width,
                options.height,
                WindowOptions::default(),
            )?,
            buffer: vec![Rgba::black(); options.width * options.height],
            last_size: (options.width, options.height),
            last_frame: time::Instant::now(),
            frame_ms: u32(1000 / options.frame_cap).expect("max 1000"),
        })
    }

    pub fn buffer(&mut self) -> Buffer {
        let new_size = self.win.get_size();
        if new_size != self.last_size {
            self.last_size = new_size;
            self.buffer.resize(new_size.0 * new_size.1, Rgba::black());
        }

        Buffer {
            width: self.last_size.0,
            height: self.last_size.1,
            inner: &mut self.buffer,
        }
    }

    pub fn frame(&mut self) -> Result<(), Error> {
        let now = time::Instant::now();
        let frame_time = now.duration_since(self.last_frame);
        if 0 == frame_time.as_secs() {
            let elapsed = frame_time.subsec_millis();
            if elapsed < self.frame_ms {
                thread::sleep(time::Duration::from_millis(u64::from(
                    self.frame_ms - elapsed,
                )));
            }
        }
        self.update_now()?;
        self.last_frame = time::Instant::now();
        Ok(())
    }

    pub fn update_now(&mut self) -> Result<(), Error> {
        let rgbas: &[Rgba] = self.buffer.as_slice();
        assert_eq!(mem::size_of::<Rgba>(), mem::size_of::<u32>());
        let u32s: &[u32] = unsafe {
            // safe as Rgba is #[repr(transpartent)]. Not actually FFI, so
            // probably safe even with #[repr(C)], and currently works even
            // with default repr (as there is no overhead)
            let u32s = rgbas.as_ptr() as *const u32;
            ::std::slice::from_raw_parts(u32s, rgbas.len())
        };
        self.win.update_with_buffer(u32s)?;
        Ok(())
    }
}

impl<'b> AsMut<[Rgba]> for Buffer<'b> {
    fn as_mut(&mut self) -> &mut [Rgba] {
        self.inner
    }
}

impl<'b> Buffer<'b> {
    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut Rgba {
        assert_lt!(x, self.width);
        assert_lt!(y, self.height);
        &mut self.inner[y * self.width + x]
    }

    pub fn set(&mut self, (x, y): (usize, usize), new: Rgba) {
        *self.get_mut((x, y)) = new;
    }

    pub fn image_one_minus_src(&mut self, image: &RgbaImage, (left, top): (usize, usize)) {
        for y in 0..image.height {
            for x in 0..image.width {
                let src = image.get((x, y));

                let dest = self.get_mut((x + left, y + top));

                let dst = *dest;
                *dest = dst.blend_one_minus_src(&src);
            }
        }
    }

    pub fn draw_line(&mut self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) {
        use crate::xiaolin_wu::XiaolinWu as Alg;
        let draw: Alg = Alg::new((x1 as f32, y1 as f32), (x2 as f32, y2 as f32));
        for ((x, y), value) in draw {
            if x >= self.width || y >= self.height {
                // TODO: https://github.com/expenses/line_drawing/issues/8
                continue;
            }
            self.set((x, y), Rgba::grey(0, (value * 255.) as u8));
        }
    }

    pub fn draw_text(
        &mut self,
        text: &crate::text::Layout,
        (left, top): (usize, usize),
    ) -> Result<(), Error> {
        for g in &text.glyphs {
            let bb = match g.pixel_bounding_box() {
                Some(bb) => bb,
                None => continue,
            };

            g.draw(|x, y, v| {
                if v < 0.05 {
                    return;
                }
                let x = left + usize(bb.min.x).expect("TODO?") + usize(x);
                let y = top + usize(bb.min.y).expect("TODO?") + usize(y);
                let v = (v * 255.).floor() as u8;
                self.set((x, y), Rgba::black());
            });
        }

        Ok(())
    }
}

impl Default for NartOptions {
    fn default() -> Self {
        NartOptions {
            name: "nart".to_string(),
            width: 640,
            height: 480,
            resize: false,
            frame_cap: 60,
        }
    }
}
