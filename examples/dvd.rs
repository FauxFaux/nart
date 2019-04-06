use failure::Error;
use minifb::Key;
use nart::fb::Nart;
use nart::rgba::RgbaVec;
use rand_core::RngCore;
use rand_core::SeedableRng;

fn main() -> Result<(), Error> {
    let image = nart::img::load_png(include_bytes!("dvd.png"))?;

    let mut nart = Nart::new("dvd", 640, 480, true)?;

    let mut rng = ByteRand::new();

    while nart.win.is_open() && !nart.win.is_key_down(Key::Escape) {
        let mut buffer = nart.buffer();

        for cell in buffer.as_mut() {
            let rand = rng.next();
            *cell = RgbaVec {
                r: rand,
                g: rand,
                b: rand,
                a: 255,
            }
            .to_packed();
        }

        buffer.image_one_minus_src(&image, (10, 10));

        nart.update()?;
    }

    Ok(())
}

struct ByteRand {
    rng: rand_xoshiro::Xoshiro256Plus,
    curr: u64,
    bip: u8,
}

impl ByteRand {
    fn new() -> ByteRand {
        let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
        ByteRand {
            curr: rng.next_u64(),
            bip: 0,
            rng,
        }
    }

    fn next(&mut self) -> u8 {
        if 8 == self.bip {
            self.bip = 0;
            self.curr = self.rng.next_u64();
        }
        let ret = self.curr as u8;
        self.curr >>= 8;
        self.bip += 1;
        ret
    }
}
