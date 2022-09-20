#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
pub use pallet::*;
pub use weights::WeightInfo;

pub mod interface;
pub use crate::interface::UserProfileInterface;
// use frame_support::pallet_prelude::*;
pub use codec::EncodeLike;
pub use scale_info::TypeInfo;

use primitives_profile_roles::ProfileRolesTrait;
use traits_user_profile::UserProfileProvider;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	pub use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	pub use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;

	#[pallet::config]
	/// Configure the pallet by specifying the parameters and types on which it depends.
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type EthereumAddress: Clone
			+ Copy
			+ PartialEq
			+ Eq
			+ Encode
			+ EncodeLike
			+ Decode
			+ Default
			+ TypeInfo
			+ sp_std::fmt::Debug;
		type ProfileRoles: Clone
			+ Copy
			+ PartialEq
			+ Eq
			+ Encode
			+ EncodeLike
			+ Decode
			+ Default
			+ TypeInfo
			+ ProfileRolesTrait
			+ sp_std::fmt::Debug;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { admin_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref admin_key) = self.admin_key {
				AdminKey::<T>::put(admin_key);
			}
		}
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ---- Types ----------------------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type EthereumAddressOf<T> = <T as Config>::EthereumAddress;
	pub type ProfileRolesOf<T> = <T as Config>::ProfileRoles;

	// ----- Storage ------------------
	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type AdminKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn eth_address_by_account_id)]
	pub type EthAddressByAccountId<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, EthereumAddressOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn account_id_by_eth_address)]
	pub type AccountIdByEthAddress<T> =
		StorageMap<_, Blake2_128Concat, EthereumAddressOf<T>, AccountIdOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn registered_account_id)]
	pub type RegisteredAccountId<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, bool>;

	#[pallet::storage]
	#[pallet::getter(fn profile_roles_by_account_id)]
	pub type ProfileRolesByAccountId<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, ProfileRolesOf<T>>;
	// -----------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// User AccountId registered as lab
		/// parameters. [Lab, who]
		EthAddressSet(EthereumAddressOf<T>, AccountIdOf<T>, ProfileRolesOf<T>),
		/// Update user profile admin key successful
		/// parameters. [who]
		UpdateUserProfileAdminKeySuccessful(AccountIdOf<T>),
		RegisteredAccountId(AccountIdOf<T>, bool),
		AdminSetProfileRoles(AccountIdOf<T>, ProfileRolesOf<T>),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		Unauthorized,
		FailedToSetProfileRole,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::set_eth_address())]
		pub fn set_eth_address(
			origin: OriginFor<T>,
			eth_address: EthereumAddressOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			<Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::set_eth_address_by_account_id(
				&who,
				&eth_address,
			);

			let roles = match <Self as UserProfileInterface<
				T,
				EthereumAddressOf<T>,
				ProfileRolesOf<T>,
			>>::get_account_profile_roles(&who)
			{
				Some(x) => x,
				None => ProfileRolesOf::<T>::default(),
			};

			Self::deposit_event(Event::<T>::EthAddressSet(eth_address, who, roles));

			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::register_account_id())]
		pub fn register_account_id(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			<Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::register_account_id(&who);
			match <Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::set_account_profile_role_to_customer(&who) {
				Ok(_) => {
					Self::deposit_event(Event::<T>::RegisteredAccountId(who, true));
					Ok(().into())
				},
				Err(_) => Err(Error::<T>::FailedToSetProfileRole.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::admin_set_eth_address())]
		pub fn admin_set_eth_address(
			origin: OriginFor<T>,
			account_id: AccountIdOf<T>,
			eth_address: EthereumAddressOf<T>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == AdminKey::<T>::get().unwrap(), Error::<T>::Unauthorized);

			<Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::set_eth_address_by_account_id(
				&account_id,
				&eth_address,
			);

			let roles = match <Self as UserProfileInterface<
				T,
				EthereumAddressOf<T>,
				ProfileRolesOf<T>,
			>>::get_account_profile_roles(&account_id)
			{
				Some(x) => x,
				None => ProfileRolesOf::<T>::default(),
			};

			Self::deposit_event(Event::<T>::EthAddressSet(eth_address, account_id, roles));

			Ok(().into())
		}

		#[pallet::weight(0)]
		pub fn admin_update_profile_roles(
			origin: OriginFor<T>,
			account_id: T::AccountId,
			profile_roles: ProfileRolesOf<T>,
		) -> DispatchResultWithPostInfo {
			let admin = ensure_signed(origin)?;

			ensure!(admin == AdminKey::<T>::get().unwrap(), Error::<T>::Unauthorized);

			match <Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::set_account_profile_roles(&account_id, &profile_roles) {
				Ok(_) => {
					Self::deposit_event(Event::<T>::AdminSetProfileRoles(account_id, profile_roles));
					Ok(Pays::No.into())
				},
				Err(_) => Err(Error::<T>::FailedToSetProfileRole.into()),
			}
		}

		#[pallet::weight(T::WeightInfo::update_admin_key())]
		pub fn update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			ensure!(who == AdminKey::<T>::get().unwrap(), Error::<T>::Unauthorized);

			match <Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::update_admin_key(
				&who,
				&account_id,
			) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateUserProfileAdminKeySuccessful(who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_admin_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			AdminKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateUserProfileAdminKeySuccessful(account_id));

			Ok(Pays::No.into())
		}
	}
}

impl<T: Config> UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>> for Pallet<T> {
	type Error = Error<T>;

	fn set_eth_address_by_account_id(
		account_id: &T::AccountId,
		eth_address: &EthereumAddressOf<T>,
	) {
		EthAddressByAccountId::<T>::insert(account_id, eth_address);
		AccountIdByEthAddress::<T>::insert(eth_address, account_id);
	}

	fn set_account_profile_roles(
		account_id: &T::AccountId,
		role: &ProfileRolesOf<T>,
	) -> Result<(), Self::Error> {
		ProfileRolesByAccountId::<T>::insert(account_id, role);
		Ok(())
	}

	fn set_account_profile_role_to_customer(
		account_id: &T::AccountId,
	) -> Result<ProfileRolesOf<T>, Self::Error> {
		let mut roles = match <Self as UserProfileInterface<
			T,
			EthereumAddressOf<T>,
			ProfileRolesOf<T>,
		>>::get_account_profile_roles(account_id)
		{
			Some(x) => x,
			None => ProfileRolesOf::<T>::default(),
		};
		roles.set_is_customer(true);
		ProfileRolesByAccountId::<T>::insert(account_id, roles);
		Ok(roles)
	}

	fn register_account_id(account_id: &T::AccountId) {
		RegisteredAccountId::<T>::insert(account_id, true);
	}

	fn update_admin_key(
		account_id: &T::AccountId,
		admin_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != AdminKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		AdminKey::<T>::put(admin_key);

		Ok(())
	}

	fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthereumAddressOf<T>> {
		EthAddressByAccountId::<T>::get(account_id)
	}

	fn get_account_profile_roles(account_id: &T::AccountId) -> Option<ProfileRolesOf<T>> {
		ProfileRolesByAccountId::<T>::get(account_id)
	}

	fn get_account_id_by_eth_address(eth_address: &EthereumAddressOf<T>) -> Option<AccountIdOf<T>> {
		AccountIdByEthAddress::<T>::get(eth_address)
	}

	fn get_registered_account_id(account_id: &T::AccountId) -> Option<bool> {
		RegisteredAccountId::<T>::get(account_id)
	}
}

impl<T: Config> UserProfileProvider<T, EthereumAddressOf<T>, ProfileRolesOf<T>> for Pallet<T> {
	type Error = Error<T>;
	type ProfileRoles = ProfileRolesOf<T>;

	fn get_eth_address_by_account_id(account_id: &T::AccountId) -> Option<EthereumAddressOf<T>> {
		<Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::get_eth_address_by_account_id(
			account_id,
		)
	}
	fn get_registered_account_id(account_id: &T::AccountId) -> Option<bool> {
		<Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::get_registered_account_id(
			account_id,
		)
	}
	fn get_account_profile_roles(account_id: &T::AccountId) -> Option<ProfileRolesOf<T>> {
		<Self as UserProfileInterface<T, EthereumAddressOf<T>, ProfileRolesOf<T>>>::get_account_profile_roles(
			account_id,
		)
	}

	fn set_account_profile_roles(account_id: &T::AccountId, role: &ProfileRolesOf<T>) {
		ProfileRolesByAccountId::<T>::insert(account_id, role);
	}
	fn set_account_profile_role_to_lab(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error> {
		let mut roles = match <Self as UserProfileInterface<
			T,
			EthereumAddressOf<T>,
			ProfileRolesOf<T>,
		>>::get_account_profile_roles(account_id)
		{
			Some(x) => x,
			None => ProfileRolesOf::<T>::default(),
		};
		roles.set_is_lab(true);
		ProfileRolesByAccountId::<T>::insert(account_id, roles);
		Ok(roles)
	}
	fn set_account_profile_role_to_customer(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error> {
		let mut roles = match <Self as UserProfileInterface<
			T,
			EthereumAddressOf<T>,
			ProfileRolesOf<T>,
		>>::get_account_profile_roles(account_id)
		{
			Some(x) => x,
			None => ProfileRolesOf::<T>::default(),
		};
		roles.set_is_customer(true);
		ProfileRolesByAccountId::<T>::insert(account_id, roles);
		Ok(roles)
	}
	fn set_account_profile_role_to_genetic_analyst(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error> {
		let mut roles = match <Self as UserProfileInterface<
			T,
			EthereumAddressOf<T>,
			ProfileRolesOf<T>,
		>>::get_account_profile_roles(account_id)
		{
			Some(x) => x,
			None => ProfileRolesOf::<T>::default(),
		};
		roles.set_is_genetic_analyst(true);
		ProfileRolesByAccountId::<T>::insert(account_id, roles);
		Ok(roles)
	}
	fn set_account_profile_role_to_doctor(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error> {
		let mut roles = match <Self as UserProfileInterface<
			T,
			EthereumAddressOf<T>,
			ProfileRolesOf<T>,
		>>::get_account_profile_roles(account_id)
		{
			Some(x) => x,
			None => ProfileRolesOf::<T>::default(),
		};
		roles.set_is_doctor(true);
		ProfileRolesByAccountId::<T>::insert(account_id, roles);
		Ok(roles)
	}
	fn set_account_profile_role_to_hospital(
		account_id: &T::AccountId,
	) -> Result<Self::ProfileRoles, Self::Error> {
		let mut roles = match <Self as UserProfileInterface<
			T,
			EthereumAddressOf<T>,
			ProfileRolesOf<T>,
		>>::get_account_profile_roles(account_id)
		{
			Some(x) => x,
			None => ProfileRolesOf::<T>::default(),
		};
		roles.set_is_hospital(true);
		ProfileRolesByAccountId::<T>::insert(account_id, roles);
		Ok(roles)
	}
}
