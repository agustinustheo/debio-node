#![cfg_attr(not(feature = "std"), no_std)]

pub mod interface;
pub mod weights;
use interface::OrderInterface;

use frame_support::{
	codec::{Decode, Encode},
	pallet_prelude::*,
	traits::Currency,
};
pub use pallet::*;
use primitives_price_and_currency::{CurrencyType, Price};
pub use scale_info::TypeInfo;
use sp_std::{prelude::*, vec};
use traits_genetic_testing::{DnaSampleTracking, DnaSampleTrackingId, GeneticTestingProvider};
use traits_order::{OrderEventEmitter, OrderStatusUpdater};
use traits_services::{types::ServiceFlow, ServiceInfo, ServicesProvider};
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub enum OrderStatus {
	Unpaid,
	Paid,
	Fulfilled,
	Refunded,
	Cancelled,
	Failed,
}
impl Default for OrderStatus {
	fn default() -> Self {
		OrderStatus::Unpaid
	}
}

#[derive(Encode, Decode, Clone, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct Order<Hash, AccountId, Balance, Moment> {
	pub id: Hash,
	pub service_id: Hash,
	pub customer_id: AccountId,
	pub customer_box_public_key: Hash,
	pub seller_id: AccountId,
	pub dna_sample_tracking_id: DnaSampleTrackingId,
	pub currency: CurrencyType,
	pub prices: Vec<Price<Balance>>,
	pub additional_prices: Vec<Price<Balance>>,
	pub status: OrderStatus,
	pub order_flow: ServiceFlow,
	pub created_at: Moment,
	pub updated_at: Moment,
}
#[allow(clippy::too_many_arguments)]
impl<Hash, AccountId, Balance, Moment> Order<Hash, AccountId, Balance, Moment> {
	pub fn new(
		id: Hash,
		service_id: Hash,
		customer_id: AccountId,
		customer_box_public_key: Hash,
		seller_id: AccountId,
		dna_sample_tracking_id: DnaSampleTrackingId,
		currency: CurrencyType,
		order_flow: ServiceFlow,
		prices: Vec<Price<Balance>>,
		additional_prices: Vec<Price<Balance>>,
		created_at: Moment,
		updated_at: Moment,
	) -> Self {
		Self {
			id,
			service_id,
			customer_id,
			customer_box_public_key,
			seller_id,
			dna_sample_tracking_id,
			currency,
			prices,
			additional_prices,
			status: OrderStatus::default(),
			order_flow,
			created_at,
			updated_at,
		}
	}

	pub fn get_id(&self) -> &Hash {
		&self.id
	}

	pub fn get_created_at(&self) -> &Moment {
		&self.created_at
	}

	pub fn get_service_id(&self) -> &Hash {
		&self.service_id
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_support::dispatch::DispatchResultWithPostInfo;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Services: ServicesProvider<Self, BalanceOf<Self>>;
		type GeneticTesting: GeneticTestingProvider<Self>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type OrdersWeightInfo: WeightInfo;
	}

	// ----- This is template code, every pallet needs this ---
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}
	// --------------------------------------------------------

	// ---- Types --------------------------------------------
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;
	pub type HashOf<T> = <T as frame_system::Config>::Hash;
	pub type CurrencyOf<T> = <T as self::Config>::Currency;
	pub type BalanceOf<T> = <CurrencyOf<T> as Currency<AccountIdOf<T>>>::Balance;
	pub type OrderOf<T> = Order<HashOf<T>, AccountIdOf<T>, BalanceOf<T>, MomentOf<T>>;
	type OrderIdsOf<T> = Vec<HashOf<T>>;
	// -------------------------------------------------------

	// ------ Storage --------------------------
	#[pallet::storage]
	#[pallet::getter(fn order_by_id)]
	pub type Orders<T> = StorageMap<_, Blake2_128Concat, HashOf<T>, OrderOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn orders_by_customer_id)]
	pub type OrdersByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn orders_by_lab_id)]
	pub type OrdersBySeller<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn pending_genetic_analysis_orders_by_genetic_analyst_id)]
	pub type PendingOrdersBySeller<T> =
		StorageMap<_, Blake2_128Concat, AccountIdOf<T>, OrderIdsOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn last_order_by_customer_id)]
	pub type LastOrderByCustomer<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, HashOf<T>>;

	#[pallet::storage]
	#[pallet::getter(fn admin_key)]
	pub type EscrowKey<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;
	// -----------------------------------------

	// ----- Genesis Configs ------------------
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub escrow_key: Option<T::AccountId>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { escrow_key: None }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			if let Some(ref escrow_key) = self.escrow_key {
				EscrowKey::<T>::put(escrow_key);
			}
		}
	}
	// ----------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Order created
		/// parameters, [Order]
		OrderCreated(OrderOf<T>),
		/// Order paid
		/// parameters, [Order]
		OrderPaid(OrderOf<T>),
		/// Order Fulfilled
		/// parameters, [Order]
		OrderFulfilled(OrderOf<T>),
		/// Order Refunded
		/// parameters, [Order]
		OrderRefunded(OrderOf<T>),
		/// Order Cancelled
		/// parameters, [Order]
		OrderCancelled(OrderOf<T>),
		/// Order Not Found
		/// parameters, []
		OrderNotFound,
		/// Update Order escrow key
		/// parameters. [who]
		UpdateOrderEscrowKeySuccessful(AccountIdOf<T>),
		/// Order Failed
		/// parameters, [Order]
		OrderFailed(OrderOf<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Service id does not exist
		ServiceDoesNotExist,
		/// Order does not exist
		OrderNotFound,
		/// Unauthorized to fulfill order - user is not the seller who owns the service
		UnauthorizedOrderFulfillment,
		/// Unauthorized to cancel order - user is not the customer who created the order
		UnauthorizedOrderCancellation,
		// Genetic Testing is ongoing, cannot be cancelled
		OngoingOrderCannotBeCancelled,
		/// Can not fulfill order before Specimen is processed
		DnaSampleNotSuccessfullyProcessed,
		/// Refund not allowed, Order is not expired yet
		OrderNotYetExpired,
		/// Unauthorized Account
		Unauthorized,
		/// Error on creating DNA sample
		DnaSampleInitalizationError,
		/// Customer eth address not found
		CustomerEthAddressNotFound,
		/// Seller eth address not found
		SellerEthAddressNotFound,
		/// Service Price Index not found
		PriceIndexNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::OrdersWeightInfo::create_order())]
		pub fn create_order(
			origin: OriginFor<T>,
			service_id: T::Hash,
			price_index: u32,
			customer_box_public_key: T::Hash,
			order_flow: ServiceFlow,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::create_order(
				&who,
				&service_id,
				price_index,
				&customer_box_public_key,
				order_flow,
			) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderCreated(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::cancel_order())]
		pub fn cancel_order(origin: OriginFor<T>, order_id: T::Hash) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::cancel_order(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderCancelled(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::set_order_paid())]
		pub fn set_order_paid(
			origin: OriginFor<T>,
			order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::set_order_paid(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderPaid(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::fulfill_order())]
		pub fn fulfill_order(
			origin: OriginFor<T>,
			order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::fulfill_order(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderFulfilled(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::set_order_refunded())]
		pub fn set_order_refunded(
			origin: OriginFor<T>,
			order_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::set_order_refunded(&who, &order_id) {
				Ok(order) => {
					Self::deposit_event(Event::<T>::OrderRefunded(order));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(T::OrdersWeightInfo::update_escrow_key())]
		pub fn update_escrow_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			match <Self as OrderInterface<T>>::update_escrow_key(&who, &account_id) {
				Ok(_) => {
					Self::deposit_event(Event::UpdateOrderEscrowKeySuccessful(who.clone()));
					Ok(().into())
				},
				Err(error) => Err(error.into()),
			}
		}

		#[pallet::weight(0)]
		pub fn sudo_update_escrow_key(
			origin: OriginFor<T>,
			account_id: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			EscrowKey::<T>::put(&account_id);

			Self::deposit_event(Event::UpdateOrderEscrowKeySuccessful(account_id));

			Ok(Pays::No.into())
		}
	}
}

impl<T: Config> OrderInterface<T> for Pallet<T> {
	type Order = OrderOf<T>;
	type Error = Error<T>;

	fn create_order(
		customer_id: &T::AccountId,
		service_id: &T::Hash,
		price_index: u32,
		customer_box_public_key: &T::Hash,
		order_flow: ServiceFlow,
	) -> Result<Self::Order, Self::Error> {
		let service = T::Services::service_by_id(service_id);
		if service.is_none() {
			return Err(Error::<T>::ServiceDoesNotExist)
		}
		let service = service.unwrap();
		let order_id = Self::generate_order_id(customer_id, service_id);
		let seller_id = service.get_owner_id();
		let prices_by_currency = service.get_prices_by_currency();

		if prices_by_currency.is_empty() ||
			prices_by_currency.len() - 1 < price_index.try_into().unwrap()
		{
			return Err(Error::<T>::PriceIndexNotFound)
		}

		let price_by_currency = &prices_by_currency[price_index as usize];

		let currency = &price_by_currency.currency;
		let prices = &price_by_currency.price_components;
		let additional_prices = &price_by_currency.additional_prices;

		let now = pallet_timestamp::Pallet::<T>::get();

		// Initialize DnaSample
		let dna_sample = T::GeneticTesting::register_dna_sample(seller_id, customer_id, &order_id);
		if dna_sample.is_err() {
			return Err(Error::<T>::DnaSampleInitalizationError)
		}
		let dna_sample = dna_sample.ok().unwrap();

		let order = Order::new(
			order_id,
			*service_id,
			customer_id.clone(),
			*customer_box_public_key,
			seller_id.clone(),
			dna_sample.get_tracking_id().clone(),
			currency.clone(),
			order_flow,
			prices.clone(),
			additional_prices.clone(),
			now,
			now,
		);
		Self::insert_order_to_storage(&order);

		Ok(order)
	}

	fn cancel_order(
		customer_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let order = Orders::<T>::get(order_id);
		if order.is_none() {
			return Err(Error::<T>::OrderNotFound)
		}
		let order = order.unwrap();

		if order.customer_id != customer_id.clone() {
			return Err(Error::<T>::UnauthorizedOrderCancellation)
		}

		let dna_sample =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id).unwrap();
		if !dna_sample.is_registered() {
			return Err(Error::<T>::OngoingOrderCannotBeCancelled)
		}

		// Delete dna sample associated with the order
		let _ = T::GeneticTesting::delete_dna_sample(&order.dna_sample_tracking_id);

		let order = Self::update_order_status(order_id, OrderStatus::Cancelled).unwrap();

		Ok(order)
	}

	fn set_order_paid(
		escrow_account_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		if escrow_account_id.clone() != EscrowKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		let order = Self::update_order_status(order_id, OrderStatus::Paid);
		if order.is_none() {
			return Err(Error::<T>::OrderNotFound)
		}

		Ok(order.unwrap())
	}

	fn fulfill_order(
		seller_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		let order = Orders::<T>::get(order_id);
		if order.is_none() {
			return Err(Error::<T>::OrderNotFound)
		}
		let order = order.unwrap();

		// Only the seller can fulfill the order
		if order.seller_id != seller_id.clone() {
			return Err(Error::<T>::UnauthorizedOrderFulfillment)
		}

		let dna_sample =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id);
		if !dna_sample.unwrap().process_success() {
			return Err(Error::<T>::DnaSampleNotSuccessfullyProcessed)
		}

		let order = Self::update_order_status(order_id, OrderStatus::Fulfilled);

		Ok(order.unwrap())
	}

	fn set_order_refunded(
		escrow_account_id: &T::AccountId,
		order_id: &T::Hash,
	) -> Result<Self::Order, Self::Error> {
		if escrow_account_id.clone() != EscrowKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		let order = Orders::<T>::get(order_id);
		if order.is_none() {
			return Err(Error::<T>::OrderNotFound)
		}

		let order_can_be_refunded = Self::order_can_be_refunded(order.unwrap());
		if !order_can_be_refunded {
			return Err(Error::<T>::OrderNotYetExpired)
		}

		let order = Self::update_order_status(order_id, OrderStatus::Refunded);
		Ok(order.unwrap())
	}

	fn update_escrow_key(
		account_id: &T::AccountId,
		escrow_key: &T::AccountId,
	) -> Result<(), Self::Error> {
		if account_id.clone() != EscrowKey::<T>::get().unwrap() {
			return Err(Error::<T>::Unauthorized)
		}

		EscrowKey::<T>::put(escrow_key);

		Ok(())
	}

	fn is_pending_order_ids_by_seller_exist(account_id: &T::AccountId) -> bool {
		match PendingOrdersBySeller::<T>::get(account_id) {
			Some(_arr) => !_arr.is_empty(),
			None => false,
		}
	}
}

use frame_support::{sp_runtime::traits::Hash, sp_std::convert::TryInto};

impl<T: Config> Pallet<T> {
	pub fn generate_order_id(customer_id: &T::AccountId, service_id: &T::Hash) -> T::Hash {
		let mut customer_id_bytes = customer_id.encode();
		let mut service_id_bytes = service_id.encode();
		let account_info = frame_system::Pallet::<T>::account(customer_id);
		let mut nonce_bytes = account_info.nonce.encode();

		customer_id_bytes.append(&mut service_id_bytes);
		customer_id_bytes.append(&mut nonce_bytes);

		let seed = &customer_id_bytes;
		T::Hashing::hash(seed)
	}

	pub fn update_order_status(order_id: &T::Hash, status: OrderStatus) -> Option<OrderOf<T>> {
		Orders::<T>::mutate(order_id, |order| match order {
			None => None,
			Some(order) => {
				order.status = status;
				order.updated_at = pallet_timestamp::Pallet::<T>::get();
				Some(order.clone())
			},
		})
	}

	pub fn insert_order_to_storage(order: &OrderOf<T>) {
		Orders::<T>::insert(order.id, order);
		LastOrderByCustomer::<T>::insert(&order.customer_id, order.id);
		Self::insert_order_id_into_orders_by_seller(order);
		Self::insert_order_id_into_pending_orders_by_seller(order);
		Self::insert_order_id_into_orders_by_customer(order);
	}

	pub fn insert_order_id_into_orders_by_seller(order: &OrderOf<T>) {
		match OrdersBySeller::<T>::get(&order.seller_id) {
			None => {
				OrdersBySeller::<T>::insert(&order.seller_id, vec![order.id]);
			},
			Some(mut orders) => {
				orders.push(order.id);
				OrdersBySeller::<T>::insert(&order.seller_id, orders);
			},
		}
	}

	pub fn insert_order_id_into_orders_by_customer(order: &OrderOf<T>) {
		match OrdersByCustomer::<T>::get(&order.customer_id) {
			None => {
				OrdersByCustomer::<T>::insert(&order.customer_id, vec![order.id]);
			},
			Some(mut orders) => {
				orders.push(order.id);
				OrdersByCustomer::<T>::insert(&order.customer_id, orders);
			},
		}
	}

	pub fn insert_order_id_into_pending_orders_by_seller(order: &OrderOf<T>) {
		match PendingOrdersBySeller::<T>::get(&order.seller_id) {
			None => {
				PendingOrdersBySeller::<T>::insert(&order.seller_id, vec![order.id]);
			},
			Some(mut orders) => {
				orders.push(order.id);
				PendingOrdersBySeller::<T>::insert(&order.seller_id, orders);
			},
		}
	}

	pub fn remove_order_id_from_pending_orders_by_seller(
		seller_id: &T::AccountId,
		order_id: &T::Hash,
	) {
		let mut orders = PendingOrdersBySeller::<T>::get(seller_id).unwrap_or_default();
		orders.retain(|o_id| o_id != order_id);
		PendingOrdersBySeller::<T>::insert(seller_id, orders);
	}

	pub fn remove_order_id_from_orders_by_seller(seller_id: &T::AccountId, order_id: &T::Hash) {
		let mut orders = OrdersBySeller::<T>::get(seller_id).unwrap_or_default();
		orders.retain(|o_id| o_id != order_id);
		OrdersBySeller::<T>::insert(seller_id, orders);
	}

	pub fn remove_order_id_from_orders_by_customer(customer_id: &T::AccountId, order_id: &T::Hash) {
		let mut orders = OrdersByCustomer::<T>::get(customer_id).unwrap_or_default();
		orders.retain(|o_id| o_id != order_id);
		OrdersByCustomer::<T>::insert(customer_id, orders);
	}

	pub fn order_can_be_refunded(order: OrderOf<T>) -> bool {
		let dna_sample =
			T::GeneticTesting::dna_sample_by_tracking_id(&order.dna_sample_tracking_id).unwrap();
		if !dna_sample.is_rejected() {
			return false
		}
		true
	}
}

impl<T: Config> OrderEventEmitter<T> for Pallet<T> {
	fn emit_event_order_failed(order_id: &HashOf<T>) {
		match Self::order_by_id(order_id) {
			None => Self::deposit_event(Event::OrderNotFound),
			Some(order) => Self::deposit_event(Event::OrderFailed(order)),
		}
	}
}

impl<T: Config> OrderStatusUpdater<T> for Pallet<T> {
	fn update_status_failed(order_id: &HashOf<T>) {
		match Self::order_by_id(order_id) {
			None => Self::deposit_event(Event::OrderNotFound),
			Some(order) => {
				Self::update_order_status(&order.id, OrderStatus::Failed);
			},
		}
	}

	fn remove_order_id_from_pending_orders_by_seller(
		seller_id: &AccountIdOf<T>,
		order_id: &HashOf<T>,
	) {
		Self::remove_order_id_from_pending_orders_by_seller(seller_id, order_id);
	}

	fn is_pending_order_by_seller_exist(seller_id: &AccountIdOf<T>) -> bool {
		Self::is_pending_order_ids_by_seller_exist(seller_id)
	}
}
