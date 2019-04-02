use failure::Error;
use minifb::Window;
use minifb::WindowOptions;

pub struct Nart {
    pub win: Window,
    buffer: Vec<u32>,
    last_size: (usize, usize),
}

pub struct Buffer<'b> {
    inner: &'b mut [u32],
    width: usize,
    height: usize,
}

impl Nart {
    pub fn new(name: &str, width: usize, height: usize, resize: bool) -> Result<Nart, Error> {
        Ok(Nart {
            win: Window::new(name, width, height, WindowOptions::default())?,
            buffer: vec![0; width * height],
            last_size: (width, height),
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
