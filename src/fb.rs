use std::thread;
use std::time;

use cast::u32;
use failure::Error;
use minifb::Window;
use minifb::WindowOptions;

use crate::img::RgbaImage;
use crate::rgba::RgbaVec;

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
    buffer: Vec<u32>,
    last_size: (usize, usize),
    last_frame: time::Instant,
    frame_ms: u32,
}

pub struct Buffer<'b> {
    inner: &'b mut [u32],
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
            buffer: vec![0; options.width * options.height],
            last_size: (options.width, options.height),
            last_frame: time::Instant::now(),
            frame_ms: u32(1000 / options.frame_cap).expect("max 1000"),
        })
    }

    pub fn buffer(&mut self) -> Buffer {
        let new_size = self.win.get_size();
        if new_size != self.last_size {
            self.last_size = new_size;
            self.buffer.resize(new_size.0 * new_size.1, 0);
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
        self.win.update_with_buffer(&self.buffer)?;
        self.last_frame = time::Instant::now();
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), Error> {
        self.win.update_with_buffer(&self.buffer)?;
        Ok(())
    }
}

impl<'b> AsMut<[u32]> for Buffer<'b> {
    fn as_mut(&mut self) -> &mut [u32] {
        self.inner
    }
}

impl<'b> Buffer<'b> {
    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut u32 {
        &mut self.inner[y * self.width + x]
    }

    pub fn set(&mut self, (x, y): (usize, usize), new: u32) {
        *self.get_mut((x, y)) = new;
    }

    pub fn image_one_minus_src(&mut self, image: &RgbaImage, (left, top): (usize, usize)) {
        for y in 0..image.height {
            for x in 0..image.width {
                let src = RgbaVec::from_packed(image.get((x, y)));

                let dest = self.get_mut((x + left, y + top));

                let dst = RgbaVec::from_packed(*dest);
                *dest = dst.blend_one_minus_src(&src).to_packed();
            }
        }
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
