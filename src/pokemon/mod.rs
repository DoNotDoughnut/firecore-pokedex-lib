use serde::{Deserialize, Serialize};
use data::breeding::Breeding;
use data::LearnableMove;
use data::PokedexData;
use data::StatSet;
use data::training::Training;

pub mod data;
pub mod types;

pub mod instance;

pub mod party;
pub mod texture;

pub type PokemonId = u16;
pub type Level = u8;
pub type Stat = u8;
#[derive(Serialize, Deserialize)]
pub struct Pokemon {

	// #[serde(default)]
	// pub data_format: u8, // Current = 1

	pub data: PokedexData,
	pub base: StatSet,

	pub training: Training,
	pub breeding: Breeding,
	
	pub moves: Vec<LearnableMove>,
	
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Gender {
	
	None,
	Male,
	Female,
	
}