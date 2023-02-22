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

use frame_support::{pallet_prelude::DispatchError, BoundedVec, bounded_vec};
use frame_support::traits::ConstU32;
use sp_std::{
	convert::{TryFrom, TryInto},
	prelude::*,
};
use totem_primitives::unit_of_account::{DIVISOR_UNIT, CurrencyDetails, UnitOfAccountInterface, STORAGE_MULTIPLIER};

pub use pallet::*;
use totem_primitives::LedgerBalance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use totem_primitives::LedgerBalance;
	use totem_primitives::unit_of_account::CurrencyDetails;

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
	#[pallet::storage]
	#[pallet::getter(fn whitelisted_accounts)]
	pub type WhitelistedAccounts<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		()
	>;


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
	pub type UnitOfAccount<T: Config<I>, I: 'static = ()> = StorageValue<
		_,
		LedgerBalance,
		ValueQuery,
	>;

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
		AlreadyWhitelistedAccount,
		/// Unknown whitelisted account
		UnknownWhitelistedAccount,
		/// Max currencies exceeded
		MaxCurrenciesOutOfBound,
		/// Symbol out of Bound
		SymbolOutOfBound,
		/// Currency not found from basket
		CurrencyNotFoundFromBasket
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::weight(0)]
		#[pallet::call_index(0)]
		pub fn whitelist_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(account.clone());
			ensure!(already_whitelisted.is_none(), Error::<T, I>::AlreadyWhitelistedAccount);

			<WhitelistedAccounts<T, I>>::insert(account.clone(), ());

			Self::deposit_event(Event::AccountWhitelisted(account.clone()));
			Ok(().into())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(1)]
		pub fn remove_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(account.clone());
			ensure!(already_whitelisted.is_some(), Error::<T, I>::UnknownWhitelistedAccount);

			<WhitelistedAccounts<T, I>>::remove(account.clone());

			Self::deposit_event(Event::AccountRemoved(account));

			Ok(().into())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(2)]
		pub fn add_currency(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
			issuance: LedgerBalance,
			price: LedgerBalance
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(whitelisted_caller.clone());
			ensure!(already_whitelisted.is_some(), Error::<T, I>::UnknownWhitelistedAccount);

			<Self as UnitOfAccountInterface>::add_currency(symbol.clone(), issuance, price)?;

			Self::deposit_event(Event::CurrencyAddedToBasket(symbol));

			Ok(().into())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(3)]
		pub fn remove_currency(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(whitelisted_caller.clone());
			ensure!(already_whitelisted.is_some(), Error::<T, I>::UnknownWhitelistedAccount);

			<Self as UnitOfAccountInterface>::remove_currency(symbol.clone())?;

			Self::deposit_event(Event::CurrencyRemovedFromTheBasket(symbol));

			Ok(().into())
		}
	}
}

impl<T: Config<I>, I: 'static> UnitOfAccountInterface for Pallet<T, I> {
	fn add_currency(symbol: Vec<u8>, issuance: LedgerBalance, price: LedgerBalance) -> Result<(), DispatchError> {
		//println!("add currency");
		let bounded_symbol = BoundedVec::<u8, T::MaxSymbolOfCurrency>::try_from(symbol.clone())
			.map_err(|_e| Error::<T, I>::SymbolOutOfBound)?;

		// we need to calculate the total inverse of the currencies in the  basket
		let total_inverse_issuance_in_currency_basket = Self::calculate_total_inverse_issuance_in_basket();

		let mut currency_basket = CurrencyBasket::<T, I>::get();
		if total_inverse_issuance_in_currency_basket == 0 {
			println!("total weights is 0 for {:?}", symbol.clone());

			let unit_of_account_currency = CurrencyDetails {
				symbol: bounded_symbol,
				issuance,
				price,
				weight: None,
				weight_adjusted_price: None,
				unit_of_account: None
			};

			currency_basket
				.try_push(unit_of_account_currency)
				.map_err(|_e| Error::<T, I>::MaxCurrenciesOutOfBound)?;

			CurrencyBasket::<T, I>::set(currency_basket);


		} else {
			let weight_of_currency = Self::calculate_weight_for_currency(issuance.clone());

			let unit_of_account_currency = CurrencyDetails {
				symbol: bounded_symbol,
				issuance,
				price,
				weight: Some(weight_of_currency),
				weight_adjusted_price: Some(weight_of_currency * price),
				unit_of_account: None
			};

			currency_basket
				.try_push(unit_of_account_currency)
				.map_err(|_e| Error::<T, I>::MaxCurrenciesOutOfBound)?;

			CurrencyBasket::<T, I>::set(currency_basket);

			// recalculate weight for each currency in the basket, since a new currency is just added
			Self::calculate_individual_weights();
			// calculates the total_inverse_issuance(weights) in the basket, since a new currency is just added
			Self::calculate_total_inverse_issuance_in_basket();
			// since a new currency has been added, we need to recalculate for each currency
			Self::calculate_individual_currency_unit_of_account();
			// newly calculated unit of account for the pallet
			let unit_of_account = Self::calculate_unit_of_account();
			UnitOfAccount::<T, I>::set(unit_of_account);
		}


		Ok(())
	}

	fn remove_currency(symbol: Vec<u8>) -> Result<(), DispatchError> {
		let mut currency_details = CurrencyBasket::<T, I>::get();
		let index = currency_details.iter()
			.position(|item| item.symbol == symbol).ok_or_else(|| Error::<T, I>::CurrencyNotFoundFromBasket)?;
		currency_details.remove(index);

		CurrencyBasket::<T, I>::set(currency_details);

		// recalculate weight for each currency in the basket, since a currency is removed
		Self::calculate_individual_weights();
		// calculates the total_inverse_issuance(weights) in the basket, since a currency is removed
		Self::calculate_total_inverse_issuance_in_basket();
		// since a currency has been removed, we need to recalculate for each currency
		Self::calculate_individual_currency_unit_of_account();
		// newly calculated unit of account for the pallet
		let unit_of_account = Self::calculate_unit_of_account();
		UnitOfAccount::<T, I>::set(unit_of_account);

		Ok(())
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	pub fn get_all_currency_details() -> Vec<CurrencyDetails<T::MaxSymbolOfCurrency>> {
		let unit_of_account_in_currency_basket:  Vec<CurrencyDetails<T::MaxSymbolOfCurrency>>  =  CurrencyBasket::<T, I>::get().into_iter().collect();
		unit_of_account_in_currency_basket
	}
	pub fn calculate_individual_weights() {
		let mut currency_basket = CurrencyBasket::<T, I>::get();

		for currency_details in currency_basket.iter_mut() {
			let currency_weight = Self::calculate_weight_for_currency(currency_details.issuance.clone());
			dbg!(currency_weight);
			//println!("currency_weight {}", &currency_weight);
			let weight_adjusted_price = currency_weight * currency_details.price;
			dbg!(weight_adjusted_price);
			//println!("weight_adjusted_price {}", &weight_adjusted_price);

			currency_details.weight = Some(currency_weight);
			currency_details.weight_adjusted_price = Some(weight_adjusted_price);
		}

		CurrencyBasket::<T, I>::set(currency_basket);
	}

	pub fn calculate_weight_for_currency(issuance: LedgerBalance) -> LedgerBalance {
		let total_inverse_issuance_in_currency_basket = Self::calculate_total_inverse_issuance_in_basket();
		dbg!(&total_inverse_issuance_in_currency_basket);

		let currency_issuance_inverse = (1  * STORAGE_MULTIPLIER) / issuance;
		dbg!(currency_issuance_inverse);

		let weight_of_currency = (currency_issuance_inverse  * STORAGE_MULTIPLIER) / total_inverse_issuance_in_currency_basket;
		dbg!(weight_of_currency);

		weight_of_currency

	}

	pub fn calculate_total_inverse_issuance_in_basket() -> LedgerBalance  {
		let unit_of_account_in_currency_basket = Self::get_all_currency_details();

		let total_inverse_in_currency_basket = unit_of_account_in_currency_basket.iter()
			.fold(0, |acc, unit| acc + ((1*STORAGE_MULTIPLIER)/unit.issuance));

		dbg!(total_inverse_in_currency_basket);
		//println!("total_inverse_in_currency_basket {}", total_inverse_in_currency_basket);

		total_inverse_in_currency_basket
	}

	pub fn calculate_unit_of_account() -> LedgerBalance  {
		let unit_of_account_in_currency_basket = Self::get_all_currency_details();

		let unit_of_account  =  unit_of_account_in_currency_basket.iter()
			.fold(0, |acc, unit| acc + (unit.weight.unwrap() * unit.price));

		unit_of_account
	}

	pub fn calculate_individual_currency_unit_of_account() {
		let  unit_of_account = Self::calculate_unit_of_account();
		dbg!(unit_of_account);

		let mut currency_basket = CurrencyBasket::<T, I>::get();
		dbg!(currency_basket.len());

		for currency_details in currency_basket.iter_mut() {

			let unit_of_account_for_currency = (currency_details.price) /  unit_of_account;
			dbg!(&currency_details.symbol);
			dbg!(&unit_of_account_for_currency);
			currency_details.unit_of_account = Some(unit_of_account_for_currency);
		}

		CurrencyBasket::<T, I>::set(currency_basket);
	}
}
