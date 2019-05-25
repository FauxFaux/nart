use std::convert::TryFrom;

use failure::Error;
use minifb::Key;
use nart::fb::Nart;
use nart::fb::NartOptions;
use nart::rgba::Rgba;
use rand_core::RngCore;
use rand_core::SeedableRng;

fn main() -> Result<(), Error> {
    let image = nart::img::load_png(include_bytes!("transparent-dice.png"))?;

    let mut nart = Nart::new(&NartOptions {
        name: "dvd".to_string(),
        resize: true,
        frame_cap: 60,
        ..Default::default()
    })?;

    let mut rng = ByteRand::new();
    let font = nart::text::Font::sans_noto();
    let text = font.layout("DIGITALLY VERIFIABLE DISC", 32.);

    while nart.win.is_open() && !nart.win.is_key_down(Key::Escape) {
        let (width, height) = nart.win.get_size();

        let mut buffer = nart.buffer();

        for cell in buffer.as_mut() {
            let rand = rng.next();
            *cell = Rgba::grey(rand, 255);
        }

        for _ in 0..10 {
            let x = usize(rng.next_u32()) % (width - image.width);
            let y = usize(rng.next_u32()) % (height - image.height);

            buffer.image_one_minus_src(&image, (x, y));

            let x = usize(rng.next_u32()) % (width - usize(text.width().ceil() as u32));
            let y = usize(rng.next_u32()) % (height - 32);

            buffer.draw_text(&text, (x, y))?;

            let x1 = usize(rng.next_u32()) % width;
            let y1 = usize(rng.next_u32()) % height;

            let x2 = usize(rng.next_u32()) % width;
            let y2 = usize(rng.next_u32()) % height;

            buffer.draw_line((x1, y1), (x2, y2));
        }

        nart.frame()?;
    }

    Ok(())
}

fn usize(val: u32) -> usize {
    usize::try_from(val).expect("u32 -> usize")
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

    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }
}
