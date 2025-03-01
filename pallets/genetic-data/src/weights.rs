//! Autogenerated weights for genetic_data
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-18, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=genetic-data
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output=./pallets/genetic-data/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for genetic_data.
pub trait WeightInfo { 
	fn add_genetic_data() -> Weight; 
	fn update_genetic_data() -> Weight; 
	fn remove_genetic_data() -> Weight; 
}

/// Weights for genetic_data using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>); 
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { 
	// Storage: GeneticData GeneticDataCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: GeneticData GeneticDataByOwner (r:1 w:1) 
	// Storage: GeneticData GeneticDataCount (r:1 w:1) 
	// Storage: GeneticData GeneticDataById (r:0 w:1) 
	fn add_genetic_data() -> Weight { 
		106_823_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
	// Storage: GeneticData GeneticDataById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_genetic_data() -> Weight { 
		82_546_000_u64 
			.saturating_add(T::DbWeight::get().reads(2_u64)) 
			.saturating_add(T::DbWeight::get().writes(1_u64)) 
	}
	// Storage: GeneticData GeneticDataById (r:1 w:1) 
	// Storage: GeneticData GeneticDataByOwner (r:1 w:1) 
	// Storage: GeneticData GeneticDataCount (r:1 w:1) 
	// Storage: GeneticData GeneticDataCountByOwner (r:1 w:1) 
	fn remove_genetic_data() -> Weight { 
		115_370_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
}

// For backwards compatibility and tests
impl WeightInfo for () { 
	// Storage: GeneticData GeneticDataCountByOwner (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	// Storage: GeneticData GeneticDataByOwner (r:1 w:1) 
	// Storage: GeneticData GeneticDataCount (r:1 w:1) 
	// Storage: GeneticData GeneticDataById (r:0 w:1) 
	fn add_genetic_data() -> Weight { 
		106_823_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
	// Storage: GeneticData GeneticDataById (r:1 w:1) 
	// Storage: Timestamp Now (r:1 w:0) 
	fn update_genetic_data() -> Weight { 
		82_546_000_u64
			.saturating_add(RocksDbWeight::get().reads(2_u64)) 
			.saturating_add(RocksDbWeight::get().writes(1_u64)) 
	} 
	// Storage: GeneticData GeneticDataById (r:1 w:1) 
	// Storage: GeneticData GeneticDataByOwner (r:1 w:1) 
	// Storage: GeneticData GeneticDataCount (r:1 w:1) 
	// Storage: GeneticData GeneticDataCountByOwner (r:1 w:1) 
	fn remove_genetic_data() -> Weight { 
		115_370_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
}
