extern crate dotenv;

use color_eyre::eyre::Result;
use dotenv::dotenv;

pub fn main() -> Result<()> {
    color_eyre::install()?;
    dotenv().ok();

    println!("Hallo awqwarddev!");
    panic!("Guess I'll die");

    Ok(())
}
