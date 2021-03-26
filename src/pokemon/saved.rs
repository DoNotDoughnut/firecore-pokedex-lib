use serde::{Deserialize, Serialize};

use super::{Level, PokemonId, data::Gender, data::StatSet};

use crate::moves::SerializableMoveSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedPokemon {

    pub id: PokemonId,

    pub data: PokemonData,

    pub moves: Option<SerializableMoveSet>,
    
    pub current_hp: Option<u16>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonData {

    pub nickname: Option<String>,
    pub level: Level,
    pub gender: Gender,

    //	ability: Ability,
    
    #[serde(default = "iv_default")]
	pub ivs: StatSet,
    #[serde(default)]
    pub evs: StatSet,

    #[serde(default)]
	pub experience: u32,

    #[serde(default)]
    pub friendship: u8,

}

const fn iv_default() -> StatSet {
    StatSet::uniform(15)
}