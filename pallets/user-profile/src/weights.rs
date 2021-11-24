//! Autogenerated weights for user_profile
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-24, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=user-profile
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/user-profile/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for user_profile.
pub trait WeightInfo {
	fn set_eth_address() -> Weight;
}

/// Weights for user_profile using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: UserProfile AccountIdByEthAddress (r:0 w:1)
	// Storage: UserProfile EthAddressByAccountId (r:0 w:1)
	fn set_eth_address() -> Weight {
		(44_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: UserProfile AccountIdByEthAddress (r:0 w:1)
	// Storage: UserProfile EthAddressByAccountId (r:0 w:1)
	fn set_eth_address() -> Weight {
		(44_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
