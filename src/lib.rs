use std::io;

use failure::Error;
use failure::err_msg;

pub struct Image {
    data: Vec<u32>,
}

pub fn load_png(png: &[u8]) -> Result<Image, Error> {
    let img = png::Decoder::new(io::Cursor::new(png));
    let (output, mut reader) = img.read_info()?;
    let mut bytes = vec![0u8; output.buffer_size()];
    reader.next_frame(&mut bytes)?;
    Err(err_msg("unimplemented"))
}
