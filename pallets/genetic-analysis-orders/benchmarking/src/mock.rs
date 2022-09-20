#![cfg(test)]

use super::*;

use frame_support::{parameter_types, traits::ConstU128};
use frame_system::EnsureRoot;
use sp_io::TestExternalities;
use sp_runtime::{
	testing::Header,
	traits::{AccountIdLookup, IdentifyAccount, Verify},
	MultiSignature,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		GeneticData: genetic_data::{Pallet, Call, Storage, Event<T>},
		GeneticAnalysts: genetic_analysts::{Pallet, Call, Storage, Event<T>},
		GeneticAnalystServices: genetic_analyst_services::{Pallet, Call, Storage, Event<T>},
		GeneticAnalysis: genetic_analysis::{Pallet, Call, Storage, Event<T>},
		GeneticAnalysisOrders: genetic_analysis_orders::{Pallet, Call, Storage, Config<T>, Event<T>},
		UserProfile: user_profile::{Pallet, Call, Storage, Event<T>},
		OctopusAssets: pallet_assets::<Instance1>::{Call, Config<T>, Event<T>, Pallet, Storage},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type Call = Call;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = sp_core::H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type Header = sp_runtime::testing::Header;
	type Event = Event;
	type Origin = Origin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

/// The native token, uses 18 decimals of precision.
pub mod currency {
	use super::Balance;

	pub const UNITS: Balance = 1_000_000_000_000_000_000;
	pub const DOLLARS: Balance = UNITS;
}

pub type OctopusAssetId = u32;
pub type OctopusAssetBalance = u128;

parameter_types! {
	pub const ApprovalDeposit: Balance = currency::DOLLARS;
	pub const AssetDeposit: Balance = 100 * currency::DOLLARS;
	pub const MetadataDepositBase: Balance = 10 * currency::DOLLARS;
	pub const MetadataDepositPerByte: Balance = currency::DOLLARS;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config<pallet_assets::Instance1> for Test {
	type Event = Event;
	type Balance = OctopusAssetBalance;
	type AssetId = OctopusAssetId;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetAccountDeposit = ConstU128<{ currency::DOLLARS }>;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
}

type Balance = u128;

parameter_types! {
	pub const ExistentialDeposit: Balance = 10;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl genetic_data::Config for Test {
	type Event = Event;
	type GeneticDataWeightInfo = ();
}

impl genetic_analysts::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type GeneticAnalystServices = GeneticAnalystServices;
	type GeneticAnalystQualifications = GeneticAnalystQualifications;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type UserProfile = UserProfile;
	type GeneticAnalystWeightInfo = ();
}

impl genetic_analyst_services::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type GeneticAnalystServiceOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl genetic_analyst_qualifications::Config for Test {
	type Event = Event;
	type GeneticAnalystQualificationOwner = GeneticAnalysts;
	type WeightInfo = ();
}

impl genetic_analysis::Config for Test {
	type Event = Event;
	type RandomnessSource = RandomnessCollectiveFlip;
	type GeneticAnalysisOrders = GeneticAnalysisOrders;
	type GeneticAnalysisWeightInfo = ();
}

impl genetic_analysis_orders::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type Assets = OctopusAssets;
	type GeneticAnalysis = GeneticAnalysis;
	type GeneticAnalystServices = GeneticAnalystServices;
	type GeneticAnalysisOrdersWeightInfo = ();
}

impl user_profile::Config for Runtime {
	type Event = Event;
	type EthereumAddress = EthereumAddress;
}

pub struct ExternalityBuilder;

impl ExternalityBuilder {
	pub fn build() -> TestExternalities {
		let mut storage = system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
		storage.extend(
			GenesisConfig::<Runtime> {
				orders: OrdersConfig {
					escrow_key: hex![
						"18c79faa6203d8b8349b19cc72cc6bfd008c243ea998435847abf6618756ca0b"
					]
					.into(),
				},
			}
			.build_storage()
			.unwrap(),
		);
		storage.into()
	}
}
