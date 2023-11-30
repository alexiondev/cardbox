
use std::fs;

use anyhow;
use log::info;
use strum::IntoEnumIterator;

use common::data::Species;

const ASSETS_DIR: &str = "assets/sprites/box";

pub fn import_named(path: &str) -> anyhow::Result<()> {
    for pokemon in Species::iter() {
        match pokemon {
            Species::DEFAULT |
            Species::UNKNOWN => continue,
            _ => import(path, &pokemon.to_filename())?,
        };
    }

    Ok(())
}

fn import(path: &str, pokemon: &str) -> anyhow::Result<()> {
    let filename = format!("{pokemon}.png");
    let from = format!("{path}/{filename}");
    let to = format!("{ASSETS_DIR}/{filename}");

    info!("Attempting to copy {from} to {to}.");

    fs::copy(from, to)?;

    Ok(())
}