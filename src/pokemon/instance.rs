use serde::{Deserialize, Serialize};

use super::{Level, PokemonId, Gender, data::StatSet};

use crate::moves::SerializableMoveSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonInstance {

	pub id: PokemonId,
    pub nickname: Option<String>,
    pub level: Level,
    pub gender: Gender, // optional because it hasn't been calculated yet (some pokemon arent 50/50 gender)
    
    #[serde(default = "iv_default")]
	pub ivs: StatSet,
    #[serde(default)]
    pub evs: StatSet,
    
    pub moves: Option<SerializableMoveSet>,
    
    #[serde(default)]
	pub exp: u32,
    #[serde(default)]
    pub friendship: u8,
    
    pub current_hp: Option<u16>,

}

const fn iv_default() -> StatSet {
    StatSet {
        hp: 15,
        atk: 15,
        def: 15,
        sp_atk: 15,
        sp_def: 15,
        speed: 15,
    }
}