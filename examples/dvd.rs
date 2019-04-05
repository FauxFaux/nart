use failure::Error;
use minifb::Key;
use nart::fb::Nart;
use nart::rgba::RgbaVec;
use rand_core::RngCore;
use rand_core::SeedableRng;

fn main() -> Result<(), Error> {
    let image = nart::img::load_png(include_bytes!("dvd.png"))?;

    let mut nart = Nart::new("dvd", 640, 480, true)?;

    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);

    while nart.win.is_open() && !nart.win.is_key_down(Key::Escape) {
        let mut buffer = nart.buffer();

        let mut pos = 0;
        'fill: loop {
            let mut rand = rng.next_u64();
            for _ in 0..8 {
                let brightness = rand as u8;
                rand >>= 8;
                buffer.as_mut()[pos] = RgbaVec {
                    r: brightness,
                    g: brightness,
                    b: brightness,
                    a: 255,
                }
                .to_packed();
                pos += 1;
                if buffer.as_mut().len() == pos {
                    break 'fill;
                }
            }
        }

        buffer.image_one_minus_src(&image, (10, 10));

        nart.update()?;
    }

    Ok(())
}
