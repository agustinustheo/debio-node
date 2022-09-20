#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	codec::{Decode, Encode},
	RuntimeDebug,
};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum DurationType {
	WorkingDays,
	Hours,
	Days,
}
impl Default for DurationType {
	fn default() -> Self {
		DurationType::WorkingDays
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ExpectedDuration {
	pub duration: u64,
	pub duration_type: DurationType,
}

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum MenstrualSubscriptionDuration {
	Monthly,
	Quarterly,
	Yearly,
}
impl Default for MenstrualSubscriptionDuration {
	fn default() -> Self {
		MenstrualSubscriptionDuration::Monthly
	}
}
