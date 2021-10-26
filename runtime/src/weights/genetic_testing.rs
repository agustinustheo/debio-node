
//! Autogenerated weights for genetic_testing
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-10-26, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/debio-node
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// genetic-testing
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./runtime/src/weights


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for genetic_testing.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> genetic_testing::WeightInfo for WeightInfo<T> {
	fn reject_dna_sample() -> Weight {
		(57_887_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn process_dna_sample() -> Weight {
		(14_912_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn submit_test_result() -> Weight {
		(50_211_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn submit_independent_test_result() -> Weight {
		(45_449_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn submit_data_bounty_details() -> Weight {
		(20_093_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}
