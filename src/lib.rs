use std::io;

use cast::usize;
use failure::ensure;
use failure::err_msg;
use failure::Error;

pub struct Image {
    data: Vec<u32>,
}

pub fn load_png(png: &[u8]) -> Result<Image, Error> {
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
        bytes_per_pixel * usize(output.width) == output.line_size,
        "line_width miscalculated: {} * {} != {}",
        bytes_per_pixel,
        output.width,
        output.line_size,
    );

    let scale = 4;
    for y in 0..output.height / scale {
        let row = usize(y * scale) * bytes_per_pixel * usize(output.width);
        let row = &bytes[row..row + (bytes_per_pixel * usize(output.width))];
        for x in 0..output.width / scale {
            print!(
                "{}",
                if row[bytes_per_pixel * usize(x * scale) + 3] > 0 {
                    'x'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
    Err(err_msg("unimplemented"))
}
