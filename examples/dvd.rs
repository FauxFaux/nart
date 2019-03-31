use failure::Error;

fn main() -> Result<(), Error> {
    nart::load_png(&[])?;
    Ok(())
}
