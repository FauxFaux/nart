use failure::Error;

fn main() -> Result<(), Error> {
    nart::load_png(include_bytes!("dvd.png"))?;
    Ok(())
}
