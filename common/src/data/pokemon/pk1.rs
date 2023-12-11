use crate::data::Species;

// Constants for memory access
const PARTY1_LIST_SIZE: usize = 6;
const PARTY1_POKEMON_COUNT_OFFSET: usize = 0x00;
const PARTY1_SPECIES_ID_START: usize = 0x01;
const PARTY1_SPECIES_ID_ENTRY_SIZE: usize = 0x01;
const PARTY1_POKEMON_DATA_START: usize = 0x08;
const PARTY1_POKEMON_DATA_ENTRY_SIZE: usize = 0x2C;
const PARTY1_ORIGINAL_TRAINER_NAMES_START: usize = 0x110;
const PARTY1_ORIGINAL_TRAINER_NAMES_ENTRY_SIZE: usize = 0xB;
const PARTY1_POKEMON_NAMES_START: usize = 0x152;
const PARTY1_POKEMON_NAMES_ENTRY_SIZE: usize = 0xB;

const BOX1_LIST_SIZE: usize = 20;
const BOX1_POKEMON_COUNT_OFFSET: usize = 0x00;
const BOX1_SPECIES_ID_START: usize = 0x01;
const BOX1_SPECIES_ID_ENTRY_SIZE: usize = 0x01;
const BOX1_POKEMON_DATA_START: usize = 0x16;
const BOX1_POKEMON_DATA_ENTRY_SIZE: usize = 0x21;
const BOX1_ORIGINAL_TRAINER_NAMES_START: usize = 0x2AA;
const BOX1_ORIGINAL_TRAINER_NAMES_ENTRY_SIZE: usize = 0xB;
const BOX1_POKEMON_NAMES_START: usize = 0x386;
const BOX1_POKEMON_NAMES_ENTRY_SIZE: usize = 0xB;

const PK1_SPECIES_OFFSET: usize = 0x00;
#[derive(Debug, Default)]
pub struct Pk1 {
    pub species: Species,
}

impl Pk1 {
    pub fn from_data(data: &[u8]) -> Pk1 {
        Pk1 {
            species: Species::from(data[PK1_SPECIES_OFFSET]),
            ..Default::default()
        }
    }

    pub fn from_party_data(data: &[u8], idx: usize) -> Pk1 {
        let data_start: usize = PARTY1_POKEMON_DATA_START + (PARTY1_POKEMON_DATA_ENTRY_SIZE * idx);
        let data_end: usize = data_start + PARTY1_POKEMON_DATA_ENTRY_SIZE;
        Pk1::from_data(&data[data_start..data_end])
    }

    pub fn from_box_data(data: &[u8], idx: usize) -> Pk1 {
        let data_start = BOX1_POKEMON_DATA_START + (BOX1_POKEMON_DATA_ENTRY_SIZE * idx);
        let data_end = data_start + BOX1_POKEMON_DATA_ENTRY_SIZE;
        Pk1::from_data(&data[data_start..data_end])
    }
}
