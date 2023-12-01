use crate::data::pokemon::Pk1;

const SAV1_PARTY_DATA_OFFSET        : usize = 0x2F2C;
const SAV1_PARTY_DATA_LENGTH        : usize = 0x196;
const SAV1_BOX_DATA_LENGTH          : usize = 0x462;
const SAV1_CURR_BOX_NUMBER_OFFSET   : usize = 0x284C;
const SAV1_CURR_BOX_DATA_OFFSET     : usize = 0x30C0;

// Bank 2 holds boxes 1-6, Bank 3 holds boxes 7-12
const SAV1_BANK_2_OFFSET            : usize = 0x4000;
const SAV1_BANK_3_OFFSET            : usize = 0x6000;

type Party1 = [Pk1; 6];
type Box1 = [Pk1; 20];
type Boxes1 = [Box1; 12];

#[derive(Debug, Default)]
pub struct Sav1 {
    data: Vec<u8>,

    party: Party1,
    curr_box: Box1,
    boxes: Boxes1,
}

impl From<Vec<u8>> for Sav1 {
    fn from(data: Vec<u8>) -> Self {
        let mut sav = Self {
            data,
            ..Default::default()
        };

        let party_data = &sav.data[SAV1_PARTY_DATA_OFFSET..SAV1_PARTY_DATA_OFFSET+SAV1_PARTY_DATA_LENGTH];
        for i in 0..sav.party.len() {
            sav.party[i] = Pk1::from_party_data(party_data, i);
        }

        let curr_box_data = &sav.data[SAV1_CURR_BOX_DATA_OFFSET..SAV1_CURR_BOX_DATA_OFFSET+SAV1_BOX_DATA_LENGTH];
        for i in 0..sav.curr_box.len() {
            sav.curr_box[i] = Pk1::from_box_data(curr_box_data, i);
        }

        for box_idx in 0..sav.boxes.len() {
            let bank = if box_idx < 6 {
                SAV1_BANK_2_OFFSET
            } else {
                SAV1_BANK_3_OFFSET
            };

            let start = bank + (box_idx % 6) * SAV1_BOX_DATA_LENGTH;
            let end = start + SAV1_BOX_DATA_LENGTH;

            for i in 0..sav.boxes[box_idx].len() {
                sav.boxes[box_idx][i] = Pk1::from_box_data(&sav.data[start..end], i);
            }
        }

        sav
    }
}