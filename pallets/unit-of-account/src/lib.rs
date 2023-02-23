//! # Unit-Of-Account
//!
//! A module for calculating unit of account based on issued/stored currencies
//!
//! ## Overview
//!
//! The Unit-of-accounting module provides functionality for the following:
//!
//! * Add whitelisted account
//! * Remove whitelisted account
//! * Add new currency
//! * Remove currency
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Goals
//!
//! The Unit-of-Account in Totem is designed to make the following possible:
//!
//! * Add a new currency to the basket of currencies and then generate/calulate the weight and PEER of currencies
//!
//! ## UnitOfAccountInterface Interface
//!
//! `add_currency`: Adds a currency with it's issuance to the basket.
//!
//! `remove_currency`: Removes a currency from the basket.
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::{pallet_prelude::DispatchError, traits::ConstU32, BoundedVec};
use sp_std::{
	convert::{TryFrom, TryInto},
	prelude::*,
};
use totem_primitives::unit_of_account::{
	convert_float_to_storage, convert_storage_to_float, CurrencyDetails, UnitOfAccountInterface,
};

use core::cmp::Ordering;
pub use pallet::*;
use totem_primitives::LedgerBalance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core::cmp::Ordering;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use totem_primitives::{unit_of_account::CurrencyDetails, LedgerBalance};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]

	pub struct Pallet<T, I = ()>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The max length of a whitelisted account
		type MaxWhitelistedAccounts: Get<u32>;

		/// The max number of currencies allowed in the basket
		type MaxCurrencyInBasket: Get<u32>;

		/// The max number of symbol for currency allowed in the basket
		type MaxSymbolOfCurrency: Get<u32>;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub phantom: (PhantomData<T>, PhantomData<I>),
	}

	#[cfg(feature = "std")]
	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			Self { phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {}
	}

	/// The list of whitelisted accounts
	// TODO: make it a storage value that holds a bounded vec
	#[pallet::storage]
	#[pallet::getter(fn whitelisted_accounts)]
	pub type WhitelistedAccounts<T: Config<I>, I: 'static = ()> =
		StorageValue<_, BoundedVec<T::AccountId, T::MaxCurrencyInBasket>, ValueQuery>;

	/// Holds the vec of all currencies in the basket
	#[pallet::storage]
	#[pallet::getter(fn currency_basket)]
	pub type CurrencyBasket<T: Config<I>, I: 'static = ()> = StorageValue<
		_,
		BoundedVec<CurrencyDetails<T::MaxSymbolOfCurrency>, T::MaxCurrencyInBasket>,
		ValueQuery,
	>;

	/// The calculated Nominal Effective Exchange Rate which is
	/// also known as the unit of account
	#[pallet::storage]
	#[pallet::getter(fn unit_of_account)]
	pub type UnitOfAccount<T: Config<I>, I: 'static = ()> =
		StorageValue<_, LedgerBalance, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Account Whitelisted
		AccountWhitelisted(T::AccountId),
		/// Account removed from whitelisted accounts
		AccountRemoved(T::AccountId),
		/// Currency added to the basket
		CurrencyAddedToBasket(Vec<u8>),
		/// Currency removed from the basket
		CurrencyRemovedFromTheBasket(Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Whitelisted account out of bounds
		MaxWhitelistedAccountOutOfBounds,
		/// Already Whitelisted account
		AlreadyWhitelistedAccount,
		/// Unknown whitelisted account
		UnknownWhitelistedAccount,
		/// Max currencies exceeded
		MaxCurrenciesOutOfBound,
		/// Symbol out of Bound
		SymbolOutOfBound,
		/// Currency Symbol already exists
		CurrencySymbolAlreadyExists,
		/// Currency not found from basket
		CurrencyNotFoundFromBasket,
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Whitelist an account for the pallet
		///
		/// Parameters:
		/// - `origin`: A Root sudo origin
		/// - `account:` Account to whitelist
		#[pallet::weight(0)]
		#[pallet::call_index(0)]
		pub fn whitelist_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let mut whitelisted_accounts = WhitelistedAccounts::<T, I>::get();
			ensure!(
				!Self::whitelisted_account_exists(account.clone()),
				Error::<T, I>::AlreadyWhitelistedAccount
			);

			whitelisted_accounts
				.try_push(account.clone())
				.map_err(|_e| Error::<T, I>::MaxWhitelistedAccountOutOfBounds)?;

			WhitelistedAccounts::<T, I>::set(whitelisted_accounts);

			Self::deposit_event(Event::AccountWhitelisted(account.clone()));
			Ok(().into())
		}

		/// Removes an account that is already whitelisted
		///
		/// Parameters:
		/// - `origin`: A Root sudo origin
		/// - `account:` Account to remove from whitelist
		#[pallet::weight(0)]
		#[pallet::call_index(1)]
		pub fn remove_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let mut whitelisted_accounts = WhitelistedAccounts::<T, I>::get();
			ensure!(
				Self::whitelisted_account_exists(account.clone()),
				Error::<T, I>::UnknownWhitelistedAccount
			);

			let index = whitelisted_accounts
				.iter()
				.position(|whitelisted_account| {
					whitelisted_account.cmp(&account) == Ordering::Equal
				})
				.ok_or_else(|| Error::<T, I>::UnknownWhitelistedAccount)?;
			whitelisted_accounts.remove(index);

			WhitelistedAccounts::<T, I>::set(whitelisted_accounts);

			Self::deposit_event(Event::AccountRemoved(account));

			Ok(().into())
		}

		/// Add currency into the basket of currency
		///
		/// Parameters:
		/// - `origin`: A whitelisted callet origin
		/// - `symbol:` The currency symbol
		/// - `issuance`: The total currency issuance
		/// - `price`: The price to USD  for the currency
		#[pallet::weight(0)]
		#[pallet::call_index(2)]
		pub fn add_currency(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
			issuance: LedgerBalance,
			price: LedgerBalance,
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			ensure!(
				Self::whitelisted_account_exists(whitelisted_caller),
				Error::<T, I>::UnknownWhitelistedAccount
			);

			let does_currency_symbol_exists = Self::symbol_exists(symbol.clone());
			ensure!(!does_currency_symbol_exists, Error::<T, I>::CurrencySymbolAlreadyExists);

			<Self as UnitOfAccountInterface>::add_currency(symbol.clone(), issuance, price)?;

			Self::deposit_event(Event::CurrencyAddedToBasket(symbol));

			Ok(().into())
		}

		/// Removes currency from the basket of currency
		///
		/// Parameters:
		/// - `origin`: A whitelisted callet origin
		/// - `symbol:` The currency symbol to remove
		#[pallet::weight(0)]
		#[pallet::call_index(3)]
		pub fn remove_currency(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			ensure!(
				Self::whitelisted_account_exists(whitelisted_caller),
				Error::<T, I>::UnknownWhitelistedAccount
			);

			<Self as UnitOfAccountInterface>::remove_currency(symbol.clone())?;

			Self::deposit_event(Event::CurrencyRemovedFromTheBasket(symbol));

			Ok(().into())
		}
	}
}

impl<T: Config<I>, I: 'static> UnitOfAccountInterface for Pallet<T, I> {
	fn add_currency(
		symbol: Vec<u8>,
		issuance: LedgerBalance,
		price: LedgerBalance,
	) -> Result<(), DispatchError> {
		let bounded_symbol = BoundedVec::<u8, T::MaxSymbolOfCurrency>::try_from(symbol.clone())
			.map_err(|_e| Error::<T, I>::SymbolOutOfBound)?;

		// we need to calculate the total inverse of the currencies in the  basket
		let total_inverse_issuance_in_currency_basket =
			Self::calculate_total_inverse_issuance_in_basket();

		let mut currency_basket = CurrencyBasket::<T, I>::get();
		if total_inverse_issuance_in_currency_basket == 0.0 {
			let unit_of_account_currency = CurrencyDetails {
				symbol: bounded_symbol,
				issuance,
				price,
				weight: None,
				weight_adjusted_price: None,
				unit_of_account: None,
			};

			currency_basket
				.try_push(unit_of_account_currency)
				.map_err(|_e| Error::<T, I>::MaxCurrenciesOutOfBound)?;

			CurrencyBasket::<T, I>::set(currency_basket);
		} else {
			// calculate weight for the currency added
			let currency_weight = Self::calculate_weight_for_currency(
				total_inverse_issuance_in_currency_basket.clone(),
				issuance.clone(),
			);
			let weight_adjusted_price =
				Self::calculate_weight_adjusted_price(currency_weight.clone(), price.clone());

			let unit_of_account_currency = CurrencyDetails {
				symbol: bounded_symbol,
				issuance,
				price,
				weight: Some(currency_weight),
				weight_adjusted_price: Some(weight_adjusted_price),
				unit_of_account: None,
			};

			currency_basket
				.try_push(unit_of_account_currency)
				.map_err(|_e| Error::<T, I>::MaxCurrenciesOutOfBound)?;

			CurrencyBasket::<T, I>::set(currency_basket);

			// recalculate weight for each currency in the basket, since a new currency is just added
			Self::calculate_individual_weights(total_inverse_issuance_in_currency_basket);
			// newly calculated unit of account for the pallet
			let unit_of_account = Self::calculate_unit_of_account();
			UnitOfAccount::<T, I>::set(unit_of_account.clone());
			// since a new currency has been added, we need to recalculate for each currency
			Self::calculate_individual_currency_unit_of_account(unit_of_account);
		}

		Ok(())
	}

	fn remove_currency(symbol: Vec<u8>) -> Result<(), DispatchError> {
		let mut currency_details = CurrencyBasket::<T, I>::get();
		let index = currency_details
			.iter()
			.position(|item| item.symbol == symbol)
			.ok_or_else(|| Error::<T, I>::CurrencyNotFoundFromBasket)?;
		currency_details.remove(index);

		CurrencyBasket::<T, I>::set(currency_details);

		// calculates the total_inverse_issuance(weights) in the basket, since a currency is removed
		let total_inverse_issuance = Self::calculate_total_inverse_issuance_in_basket();
		// recalculate weight for each currency in the basket, since a currency is removed
		Self::calculate_individual_weights(total_inverse_issuance);
		// newly calculated unit of account for the pallet
		let unit_of_account = Self::calculate_unit_of_account();
		UnitOfAccount::<T, I>::set(unit_of_account.clone());
		// since a currency has been removed, we need to recalculate for each currency
		Self::calculate_individual_currency_unit_of_account(unit_of_account);

		Ok(())
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	fn whitelisted_account_exists(account_id: T::AccountId) -> bool {
		let whitelisted_accounts = <WhitelistedAccounts<T, I>>::get();

		whitelisted_accounts
			.iter()
			.any(|whitelisted_account| whitelisted_account.cmp(&account_id) == Ordering::Equal)
	}

	fn symbol_exists(symbol: Vec<u8>) -> bool {
		let currency_basket = <CurrencyBasket<T, I>>::get();

		currency_basket.iter().any(|currency_details| currency_details.symbol == symbol)
	}

	pub fn get_all_currency_details() -> Vec<CurrencyDetails<T::MaxSymbolOfCurrency>> {
		let unit_of_account_in_currency_basket: Vec<CurrencyDetails<T::MaxSymbolOfCurrency>> =
			CurrencyBasket::<T, I>::get().into_iter().collect();
		unit_of_account_in_currency_basket
	}

	pub fn calculate_individual_weights(total_inverse_issuance: f64) {
		let mut currency_basket = CurrencyBasket::<T, I>::get();

		for currency_details in currency_basket.iter_mut() {
			let currency_weight = Self::calculate_weight_for_currency(
				total_inverse_issuance.clone(),
				currency_details.issuance.clone(),
			);
			let weight_adjusted_price = Self::calculate_weight_adjusted_price(
				currency_weight.clone(),
				currency_details.price,
			);

			currency_details.weight = Some(currency_weight);
			currency_details.weight_adjusted_price = Some(weight_adjusted_price);
		}

		CurrencyBasket::<T, I>::set(currency_basket);
	}

	pub fn calculate_weight_adjusted_price(
		currency_weight: LedgerBalance,
		price: LedgerBalance,
	) -> LedgerBalance {
		convert_float_to_storage(
			convert_storage_to_float(currency_weight) * convert_storage_to_float(price),
		)
	}

	pub fn calculate_weight_for_currency(
		total_inverse_issuance_in_currency_basket: f64,
		issuance: LedgerBalance,
	) -> LedgerBalance {
		let currency_issuance_inverse = 1 as f64 / issuance as f64;

		let weight_of_currency =
			currency_issuance_inverse / total_inverse_issuance_in_currency_basket;

		let weight_of_currency = convert_float_to_storage(weight_of_currency);

		weight_of_currency
	}

	pub fn calculate_total_inverse_issuance_in_basket() -> f64 {
		let unit_of_account_in_currency_basket = Self::get_all_currency_details();

		let total_inverse_in_currency_basket: f64 = unit_of_account_in_currency_basket
			.iter()
			.fold(0 as f64, |acc, unit| acc + (1 as f64) / unit.issuance as f64);

		total_inverse_in_currency_basket
	}

	pub fn calculate_unit_of_account() -> LedgerBalance {
		let unit_of_account_in_currency_basket = Self::get_all_currency_details();

		let unit_of_account =
			unit_of_account_in_currency_basket.iter().fold(0 as f64, |acc, unit| {
				acc + (convert_storage_to_float(unit.weight.unwrap()) *
					convert_storage_to_float(unit.price))
			});

		convert_float_to_storage(unit_of_account)
	}

	pub fn calculate_individual_currency_unit_of_account(unit_of_account: LedgerBalance) {
		let mut currency_basket = CurrencyBasket::<T, I>::get();

		for currency_details in currency_basket.iter_mut() {
			let price_f64 = convert_storage_to_float(currency_details.price);
			let unit_of_account_f64 = convert_storage_to_float(unit_of_account.clone());
			let unit_of_account_for_currency_f64 = price_f64 / unit_of_account_f64;
			let unit_of_account_for_currency =
				convert_float_to_storage(unit_of_account_for_currency_f64);
			currency_details.unit_of_account = Some(unit_of_account_for_currency);
		}

		CurrencyBasket::<T, I>::set(currency_basket);
	}
}
