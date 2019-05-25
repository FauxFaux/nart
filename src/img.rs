use std::convert::TryFrom;
use std::io;

use failure::ensure;
use failure::err_msg;
use failure::Error;

use crate::rgba::Rgba;
use std::convert::TryInto;

pub struct RgbaImage {
    pub width: usize,
    pub height: usize,
    data: Vec<Rgba>,
}

impl RgbaImage {
    pub fn get(&self, (x, y): (usize, usize)) -> Rgba {
        assert_lt!(x, self.width);
        assert_lt!(y, self.height);
        self.data[y * self.width + x]
    }
}

pub fn load_png(png: &[u8]) -> Result<RgbaImage, Error> {
    let img = png::Decoder::new(io::Cursor::new(png));
    let (output, mut reader) = img.read_info()?;
    let mut bytes = vec![0u8; output.buffer_size()];
    reader.next_frame(&mut bytes)?;

    drop(reader);

    ensure!(
        png::ColorType::RGBA == output.color_type,
        "RGBA images only, not {:?}",
        output.color_type,
    );

    ensure!(
        png::BitDepth::Eight == output.bit_depth,
        "8-bit images only, not {:?}",
        output.bit_depth,
    );

    let bytes_per_pixel = output.color_type.samples();

    ensure!(
        bytes_per_pixel * usize::try_from(output.width).expect("width < usize") == output.line_size,
        "line_width miscalculated: {} * {} != {}",
        bytes_per_pixel,
        output.width,
        output.line_size,
    );

    Ok(RgbaImage {
        data: bytes_to_rgba(bytes),
        width: output.width.try_into().expect("width < usize"),
        height: output.height.try_into().expect("width < usize"),
    })
}

fn bytes_to_rgba(bytes: Vec<u8>) -> Vec<Rgba> {
    // I really have no idea how endian works here
    bytes
        .chunks(4)
        .map(|v| Rgba::from_packed(u32::from_ne_bytes(v.try_into().expect("chunked"))))
        .collect()
}
