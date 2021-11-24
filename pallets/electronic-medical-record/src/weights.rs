//! Autogenerated weights for electronic_medical_record
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
// --pallet=electronic-medical-record
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --raw
// --output=./pallets/electronic-medical-record/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for electronic_medical_record.
pub trait WeightInfo {
	fn add_electronic_medical_record() -> Weight;
	fn remove_electronic_medical_record() -> Weight;
	fn add_electronic_medical_record_file() -> Weight;
	fn remove_electronic_medical_record_file() -> Weight;
}

/// Weights for electronic_medical_record using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:0 w:1)
	fn add_electronic_medical_record() -> Weight {
		(82_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	fn remove_electronic_medical_record() -> Weight {
		(95_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCountByElectronicMedicalRecordId (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:0 w:1)
	fn add_electronic_medical_record_file() -> Weight {
		(92_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCountByElectronicMedicalRecordId (r:1 w:1)
	fn remove_electronic_medical_record_file() -> Weight {
		(104_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:0 w:1)
	fn add_electronic_medical_record() -> Weight {
		(82_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordByOwner (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordCountByOwner (r:1 w:1)
	fn remove_electronic_medical_record() -> Weight {
		(95_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCountByElectronicMedicalRecordId (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:0 w:1)
	fn add_electronic_medical_record_file() -> Weight {
		(92_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordById (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCount (r:1 w:1)
	// Storage: ElectronicMedicalRecord ElectronicMedicalRecordFileCountByElectronicMedicalRecordId (r:1 w:1)
	fn remove_electronic_medical_record_file() -> Weight {
		(104_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
}
