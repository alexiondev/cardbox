
use std::fs;

use common::data::{save::Sav1, Species, SpeciesId};

fn main() -> anyhow::Result<()> {
    let data = fs::read("blue.srm")?;
    let party_data = &data[0x2F2C..12480];
    // let sav = Sav1::from(data);

    let count = party_data[0];
    println!("Party Size: {}", count);

    for i in 1..7 {
        let id = party_data[i];
        println!("{:?}", Species::from(SpeciesId::Gen1(id)));
    }
    Ok(())
}
