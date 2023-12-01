
use std::fs;

use common::data::save::Sav1;

fn main() -> anyhow::Result<()> {
    let data = fs::read("blue.srm")?;
    let sav = Sav1::from(data);

    println!("{:?}", sav);
    Ok(())
}
