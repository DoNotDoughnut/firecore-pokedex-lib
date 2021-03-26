use serde::{Deserialize, Serialize};

use super::pokemon::types::PokemonType;

pub type MoveId = u16;

pub type SerializableMoveSet = smallvec::SmallVec<[SerializableMove; 4]>;


#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct PokemonMove {

	pub number: MoveId,
	pub name: String,
	pub category: MoveCategory,
	pub pokemon_type: Option<PokemonType>,
	pub power: Option<usize>,
	pub accuracy: Option<u8>,
	pub pp: u8,
	
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableMove {

	pub move_id: MoveId,
	pub remaining_pp: u8,
	
}


#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum MoveCategory {
	
	Physical,
	Special,
	Status,	
	
}

impl std::fmt::Display for PokemonMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Default for MoveCategory {
    fn default() -> Self {
        Self::Status
    }
}