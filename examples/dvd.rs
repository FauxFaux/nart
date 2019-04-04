use failure::Error;
use minifb::Key;
use nart::fb::Nart;

fn main() -> Result<(), Error> {
    let mut noise;
    let mut carry;
    let mut seed = 0xbeefu32;

    let image = nart::img::load_png(include_bytes!("dvd.png"))?;

    let mut nart = Nart::new("dvd", 640, 480, true)?;

    while nart.win.is_open() && !nart.win.is_key_down(Key::Escape) {
        let mut buffer = nart.buffer();

        for i in buffer.as_mut().iter_mut() {
            noise = seed;
            noise >>= 3;
            noise ^= seed;
            carry = noise & 1;
            noise >>= 1;
            seed >>= 1;
            seed |= carry << 30;
            noise &= 0xFF;
            *i = (noise << 16) | (noise << 8) | noise;
        }

        buffer.image_one_minus_src(&image, (10, 10));

        nart.update()?;
    }

    Ok(())
}
