use strum_macros::EnumIter;


#[derive(Debug, Default, EnumIter, PartialEq)]
pub enum Species {
    #[default]
    DEFAULT,
    UNKNOWN,
    Bulbasaur,
    Ivysaur,
    Venusaur,
    Charmander,
    Charmeleon,
    Charizard,
    Squirtle,
    Wartortle,
    Blastoise,
    Caterpie,
    Metapod,
    Butterfree,
    Weedle,
    Kakuna,
    Beedrill,
    Pidgey,
    Pidgeotto,
    Pidgeot,
    Rattata,
    Raticate,
    Spearow,
    Fearow,
    Ekans,
    Arbok,
    Pikachu,
    Raichu,
    Sandshrew,
    Sandslash,
    NidoranF,
    Nidorina,
    Nidoqueen,
    NidoranM,
    Nidorino,
    Nidoking,
    Clefairy,
    Clefable,
    Vulpix,
    Ninetales,
    Jigglypuff,
    Wigglytuff,
    Zubat,
    Golbat,
    Oddish,
    Gloom,
    Vileplume,
    Paras,
    Parasect,
    Venonat,
    Venomoth,
    Diglett,
    Dugtrio,
    Meowth,
    Persian,
    Psyduck,
    Golduck,
    Mankey,
    Primeape,
    Growlithe,
    Arcanine,
    Poliwag,
    Poliwhirl,
    Poliwrath,
    Abra,
    Kadabra,
    Alakazam,
    Machop,
    Machoke,
    Machamp,
    Bellsprout,
    Weepinbell,
    Victreebel,
    Tentacool,
    Tentacruel,
    Geodude,
    Graveler,
    Golem,
    Ponyta,
    Rapidash,
    Slowpoke,
    Slowbro,
    Magnemite,
    Magneton,
    Farfetchd,
    Doduo,
    Dodrio,
    Seel,
    Dewgong,
    Grimer,
    Muk,
    Shellder,
    Cloyster,
    Gastly,
    Haunter,
    Gengar,
    Onix,
    Drowzee,
    Hypno,
    Krabby,
    Kingler,
    Voltorb,
    Electrode,
    Exeggcute,
    Exeggutor,
    Cubone,
    Marowak,
    Hitmonlee,
    Hitmonchan,
    Lickitung,
    Koffing,
    Weezing,
    Rhyhorn,
    Rhydon,
    Chansey,
    Tangela,
    Kangaskhan,
    Horsea,
    Seadra,
    Goldeen,
    Seaking,
    Staryu,
    Starmie,
    MrMime,
    Scyther,
    Jynx,
    Electabuzz,
    Magmar,
    Pinsir,
    Tauros,
    Magikarp,
    Gyarados,
    Lapras,
    Ditto,
    Eevee,
    Vaporeon,
    Jolteon,
    Flareon,
    Porygon,
    Omanyte,
    Omastar,
    Kabuto,
    Kabutops,
    Aerodactyl,
    Snorlax,
    Articuno,
    Zapdos,
    Moltres,
    Dratini,
    Dragonair,
    Dragonite,
    Mewtwo,
    Mew,
}

impl Species {
    pub fn to_filename(&self) -> String {
        return match self {
            Species::DEFAULT |
            Species::UNKNOWN => String::new(),
            Species::NidoranM => String::from("nidoran-m"),
            Species::NidoranF => String::from("nidoran-f"),
            Species::MrMime => String::from("mr-mime"),
            _ => format!("{:?}", self).to_lowercase(),
        }
    }
}
impl From<u8> for Species {
    fn from(id: u8) -> Self {
        return match id {
            001 => Self::Rhydon,
            002 => Self::Kangaskhan,
            003 => Self::NidoranM,
            004 => Self::Clefairy,
            005 => Self::Spearow,
            006 => Self::Voltorb,
            007 => Self::Nidoking,
            008 => Self::Slowbro,
            009 => Self::Ivysaur,
            010 => Self::Exeggutor,
            011 => Self::Lickitung,
            012 => Self::Exeggcute,
            013 => Self::Grimer,
            014 => Self::Gengar,
            015 => Self::NidoranF,
            016 => Self::Nidoqueen,
            017 => Self::Cubone,
            018 => Self::Rhyhorn,
            019 => Self::Lapras,
            020 => Self::Arcanine,
            021 => Self::Mew,
            022 => Self::Gyarados,
            023 => Self::Shellder,
            024 => Self::Tentacool,
            025 => Self::Gastly,
            026 => Self::Scyther,
            027 => Self::Staryu,
            028 => Self::Blastoise,
            029 => Self::Pinsir,
            030 => Self::Tangela,
            033 => Self::Growlithe,
            034 => Self::Onix,
            035 => Self::Fearow,
            036 => Self::Pidgey,
            037 => Self::Slowpoke,
            038 => Self::Kadabra,
            039 => Self::Graveler,
            040 => Self::Chansey,
            041 => Self::Machoke,
            042 => Self::MrMime,
            043 => Self::Hitmonlee,
            044 => Self::Hitmonchan,
            045 => Self::Arbok,
            046 => Self::Parasect,
            047 => Self::Psyduck,
            048 => Self::Drowzee,
            049 => Self::Golem,
            051 => Self::Magmar,
            053 => Self::Electabuzz,
            054 => Self::Magneton,
            055 => Self::Koffing,
            057 => Self::Mankey,
            059 => Self::Diglett,
            060 => Self::Tauros,
            064 => Self::Farfetchd,
            065 => Self::Venonat,
            066 => Self::Dragonite,
            070 => Self::Doduo,
            071 => Self::Poliwag,
            072 => Self::Jynx,
            073 => Self::Moltres,
            074 => Self::Articuno,
            075 => Self::Zapdos,
            076 => Self::Ditto,
            077 => Self::Meowth,
            078 => Self::Krabby,
            082 => Self::Vulpix,
            083 => Self::Ninetales,
            084 => Self::Pikachu,
            085 => Self::Raichu,
            088 => Self::Dratini,
            089 => Self::Dragonair,
            090 => Self::Kabuto,
            091 => Self::Kabutops,
            092 => Self::Horsea,
            093 => Self::Seadra,
            096 => Self::Sandshrew,
            097 => Self::Sandslash,
            098 => Self::Omanyte,
            099 => Self::Omastar,
            100 => Self::Jigglypuff,
            101 => Self::Wigglytuff,
            102 => Self::Eevee,
            103 => Self::Flareon,
            104 => Self::Jolteon,
            105 => Self::Vaporeon,
            106 => Self::Machop,
            107 => Self::Zubat,
            108 => Self::Ekans,
            109 => Self::Paras,
            110 => Self::Poliwhirl,
            111 => Self::Poliwrath,
            112 => Self::Weedle,
            113 => Self::Kakuna,
            114 => Self::Beedrill,
            116 => Self::Dodrio,
            117 => Self::Primeape,
            118 => Self::Dugtrio,
            119 => Self::Venomoth,
            120 => Self::Dewgong,
            123 => Self::Caterpie,
            124 => Self::Metapod,
            125 => Self::Butterfree,
            126 => Self::Machamp,
            128 => Self::Golduck,
            129 => Self::Hypno,
            130 => Self::Golbat,
            131 => Self::Mewtwo,
            132 => Self::Snorlax,
            133 => Self::Magikarp,
            136 => Self::Muk,
            138 => Self::Kingler,
            139 => Self::Cloyster,
            141 => Self::Electrode,
            142 => Self::Clefable,
            143 => Self::Weezing,
            144 => Self::Persian,
            145 => Self::Marowak,
            147 => Self::Haunter,
            148 => Self::Abra,
            149 => Self::Alakazam,
            150 => Self::Pidgeotto,
            151 => Self::Pidgeot,
            152 => Self::Starmie,
            153 => Self::Bulbasaur,
            154 => Self::Venusaur,
            155 => Self::Tentacruel,
            157 => Self::Goldeen,
            158 => Self::Seaking,
            163 => Self::Ponyta,
            164 => Self::Rapidash,
            165 => Self::Rattata,
            166 => Self::Raticate,
            167 => Self::Nidorino,
            168 => Self::Nidorina,
            169 => Self::Geodude,
            170 => Self::Porygon,
            171 => Self::Aerodactyl,
            173 => Self::Magnemite,
            176 => Self::Charmander,
            177 => Self::Squirtle,
            178 => Self::Charmeleon,
            179 => Self::Wartortle,
            180 => Self::Charizard,
            185 => Self::Oddish,
            186 => Self::Gloom,
            187 => Self::Vileplume,
            188 => Self::Bellsprout,
            189 => Self::Weepinbell,
            190 => Self::Victreebel,
            _ => Self::UNKNOWN,
        }
    }
}