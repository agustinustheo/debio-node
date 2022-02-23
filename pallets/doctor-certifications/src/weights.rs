//! Autogenerated weights for doctor_certifications
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-02-23, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=doctor-certifications
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/doctor-certifications/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for doctor_certifications.
pub trait WeightInfo {
	fn create_certification() -> Weight;
	fn update_certification() -> Weight;
	fn delete_certification() -> Weight;
}

/// Weights for doctor_certifications using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertifications (r:0 w:1)
	fn create_certification() -> Weight {
		61_277_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	fn update_certification() -> Weight {
		39_553_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	fn delete_certification() -> Weight {
		69_187_000_u64
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertifications (r:0 w:1)
	fn create_certification() -> Weight {
		61_277_000_u64
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	fn update_certification() -> Weight {
		39_553_000_u64
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: DoctorCertifications DoctorCertifications (r:1 w:1)
	// Storage: Doctors Doctors (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCount (r:1 w:1)
	// Storage: DoctorCertifications DoctorCertificationsCountByOwner (r:1 w:1)
	fn delete_certification() -> Weight {
		69_187_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
