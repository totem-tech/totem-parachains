//                              Næ§@@@ÑÉ©
//                        æ@@@@@@@@@@@@@@@@@@
//                    Ñ@@@@?.?@@@@@@@@@@@@@@@@@@@N
//                 ¶@@@@@?^%@@.=@@@@@@@@@@@@@@@@@@@@
//               N@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^@@@».............?@@@@@@@@@É
//              Ñ@@@@@@@@?^@@@@@@@@@@@@@@@@@@'?@@@@@@@@Ñ
//              @@@@@@@@@?^@@@»..............»@@@@@@@@@@
//              @@@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@@@
//              @@@@@@@@@?^ë@@&.@@@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^´@@@o.%@@@@@@@@@@@@@@@@@@@@©
//                @@@@@@@?.´@@@@@ë.........*.±@@@@@@@æ
//                 @@@@@@@@?´.I@@@@@@@@@@@@@@.&@@@@@N
//                  N@@@@@@@@@@ë.*=????????=?@@@@@Ñ
//                    @@@@@@@@@@@@@@@@@@@@@@@@@@@¶
//                        É@@@@@@@@@@@@@@@@Ñ¶
//                             Næ§@@@ÑÉ©

// Copyright 2020 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Damilare Akinlose      email: dami@totemaccounting.com
// - Chris D'Costa          email: chris.dcosta@totemaccounting.com

// Totem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Totem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.

//! # Unit-Of-Account
//!
//! A module for calculating unit of account based on a basket of assets
//!
//! ## Overview
//!
//! The Unit-of-Account module provides functionality for the following:
//!
//! * Add whitelisted account
//! * Remove whitelisted account
//! * Add new asset to the basket
//! * Remove asset from the basket
//! * updating the price of an asset in the basket
//! * updating the issuance of an asset in the basket
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
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;

use frame_support::{
	dispatch::DispatchResultWithPostInfo,
	traits::ConstU32, 
	BoundedVec
};
use sp_std::{
	convert::{TryFrom, TryInto},
	prelude::*,
};
// use totem_primitives::LedgerBalance;
use totem_primitives::{
	LedgerBalance, 
	unit_of_account::{
		convert_float_to_storage, 
		convert_storage_to_float, 
		CurrencyDetails, 
		UnitOfAccountInterface,
	}
};

use core::cmp::Ordering;
pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	// use core::cmp::Ordering;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    // use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	// use totem_primitives::{unit_of_account::CurrencyDetails, LedgerBalance};

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The max length of a whitelisted account
		type MaxWhitelistedAccounts: Get<u32>;

		/// The max number of currencies allowed in the basket
		type MaxAssetsInBasket: Get<u32>;

		/// The max number of symbol for currency allowed in the basket
		type MaxAssets: Get<u32>;

		/// Weightinfo for pallet.
		type WeightInfo: WeightInfo;

	}

	/// The current storage version.
	const STORAGE_VERSION: frame_support::traits::StorageVersion =
	frame_support::traits::StorageVersion::new(1);
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// Whitelist an account for the pallet
		///
		/// Parameters:
		/// - `origin`: A Root sudo origin
		/// - `account:` Account to whitelist
		#[pallet::weight(T::WeightInfo::whitelist_account())]
		#[pallet::call_index(0)]
		pub fn whitelist_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ensure!(
				!Self::whitelisted_account_exists(account.clone()).unwrap_or(false),
				Error::<T, I>::AlreadyWhitelistedAccount
			);

			let mut whitelisted_accounts = WhitelistedAccounts::<T, I>::get();

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
		#[pallet::weight(T::WeightInfo::remove_account())]
		#[pallet::call_index(1)]
		pub fn remove_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let mut whitelisted_accounts = WhitelistedAccounts::<T, I>::get();
			ensure!(
				Self::whitelisted_account_exists(account.clone()).unwrap_or(false),
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
		#[pallet::weight(T::WeightInfo::add_currency())]
		#[pallet::call_index(2)]
		pub fn add_currency(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
			issuance: LedgerBalance,
			price: LedgerBalance,
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			ensure!(
				Self::whitelisted_account_exists(whitelisted_caller).unwrap_or(false),
				Error::<T, I>::UnknownWhitelistedAccount
			);

			let does_currency_symbol_exists = Self::symbol_exists(symbol.clone()).unwrap_or(false);
			ensure!(!does_currency_symbol_exists, Error::<T, I>::CurrencySymbolAlreadyExists);

			ensure!(issuance != 0, Error::<T, I>::InvalidIssuanceValue);

			ensure!(price != 0, Error::<T, I>::InvalidPriceValue);

			<Self as UnitOfAccountInterface>::add(symbol.clone(), issuance, price)?;

			Self::deposit_event(Event::CurrencyAddedToBasket(symbol));

			Ok(().into())
		}

		/// Removes currency from the basket of currency
		///
		/// Parameters:
		/// - `origin`: A whitelisted callet origin
		/// - `symbol:` The currency symbol to remove
		#[pallet::weight(T::WeightInfo::remove_currency())]
		#[pallet::call_index(3)]
		pub fn remove_currency(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			ensure!(
				Self::whitelisted_account_exists(whitelisted_caller).unwrap_or(false),
				Error::<T, I>::UnknownWhitelistedAccount
			);

			<Self as UnitOfAccountInterface>::remove(symbol.clone())?;

			Self::deposit_event(Event::CurrencyRemovedFromTheBasket(symbol));

			Ok(().into())
		}

		/// Updates currency in the basket of currency
		///
		/// Parameters:
		/// - `origin`: A whitelisted callet origin
		/// - `symbol:` The currency symbol to remove
		/// - `issuance:` The new currency issuance which can be None if not set
		/// - `price:` The new currency price which can be None if not set
		#[pallet::weight(T::WeightInfo::update_currency())]
		#[pallet::call_index(4)]
		pub fn update_currency(
			origin: OriginFor<T>,
			symbol: Vec<u8>,
			maybe_issuance: Option<LedgerBalance>,
			maybe_price: Option<LedgerBalance>,
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			ensure!(
				Self::whitelisted_account_exists(whitelisted_caller).unwrap_or(false),
				Error::<T, I>::UnknownWhitelistedAccount
			);

			if let Some(issuance) = maybe_issuance {
				ensure!(issuance != 0, Error::<T, I>::InvalidIssuanceValue);
			}

			if let Some(price) = maybe_price {
				ensure!(price != 0, Error::<T, I>::InvalidPriceValue);
			}

			<Self as UnitOfAccountInterface>::update(symbol.clone(), maybe_issuance, maybe_price)?;

			Self::deposit_event(Event::CurrencyUpdatedInTheBasket(symbol));

			Ok(().into())
		}
	}

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
		/// Currency updated in the basket
		CurrencyUpdatedInTheBasket(Vec<u8>),
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
		/// Invalid Issuance Value
		InvalidIssuanceValue,
		/// Invalid Price Value
		InvalidPriceValue,
	}

	/// The list of whitelisted accounts
	// TODO: make it a storage value that holds a bounded vec
	#[pallet::storage]
	#[pallet::getter(fn whitelisted_accounts)]
	pub type WhitelistedAccounts<T: Config<I>, I: 'static = ()> = StorageValue<
		_, 
		BoundedVec<T::AccountId, T::MaxWhitelistedAccounts>, 
		ValueQuery
	>;

	/// Holds the vec of all currencies in the basket
	#[pallet::storage]
	#[pallet::getter(fn currency_basket)]
	pub type CurrencyBasket<T: Config<I>, I: 'static = ()> = StorageValue<
		_,
		BoundedVec<CurrencyDetails<T::MaxAssets>, T::MaxAssetsInBasket>,
		ValueQuery,
	>;

	/// The calculated Nominal Effective Exchange Rate which is
	/// also known as the unit of account
	#[pallet::storage]
	#[pallet::getter(fn unit_of_account)]
	pub type UnitOfAccount<T: Config<I>, I: 'static = ()> = StorageValue<
		_, 
		LedgerBalance, 
		ValueQuery
	>;

	// #[pallet::hooks]
	// impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

}

impl<T: Config<I>, I: 'static> UnitOfAccountInterface for Pallet<T, I> {
	fn add(
		symbol: Vec<u8>,
		issuance: LedgerBalance,
		price: LedgerBalance,
	) -> Result<(), DispatchError> {
		let bounded_symbol = BoundedVec::<u8, T::MaxAssets>::try_from(symbol.clone())
			.map_err(|_e| Error::<T, I>::SymbolOutOfBound)?;

		let mut currency_basket = CurrencyBasket::<T, I>::get();
		// we need to calculate the total inverse of the currencies in the  basket
		let some_total_inverse_issuance_in_currency_basket =
			Self::calculate_total_inverse_issuance_in_basket();

		if let Some(total_inverse_issuance_in_currency_basket) =
			some_total_inverse_issuance_in_currency_basket
		{
			// calculate weight for the currency added
			if let Some(currency_weight) = Self::calculate_weight_for_currency(
				total_inverse_issuance_in_currency_basket.clone(),
				issuance.clone(),
			) {
				if let Some(weight_adjusted_price) =
					Self::calculate_weight_adjusted_price(currency_weight.clone(), price.clone())
				{
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
					if let Some(unit_of_account) = Self::calculate_unit_of_account() {
						UnitOfAccount::<T, I>::set(unit_of_account.clone());
						// since a new currency has been added, we need to recalculate for each currency
						Self::calculate_individual_currency_unit_of_account(unit_of_account);
					}
				}
			}
		} else {
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
		}

		Ok(())
	}

	fn remove(symbol: Vec<u8>) -> Result<(), DispatchError> {
		let mut currency_details = CurrencyBasket::<T, I>::get();
		let index = currency_details
			.iter()
			.position(|item| item.symbol == symbol)
			.ok_or_else(|| Error::<T, I>::CurrencyNotFoundFromBasket)?;
		currency_details.remove(index);

		CurrencyBasket::<T, I>::set(currency_details);

		// calculates the total_inverse_issuance(weights) in the basket, since a currency is removed
		let some_total_inverse_issuance = Self::calculate_total_inverse_issuance_in_basket();
		if let Some(total_inverse_issuance) = some_total_inverse_issuance {
			// recalculate weight for each currency in the basket, since a currency is removed
			Self::calculate_individual_weights(total_inverse_issuance);
			// newly calculated unit of account for the pallet
			if let Some(unit_of_account) = Self::calculate_unit_of_account() {
				UnitOfAccount::<T, I>::set(unit_of_account.clone());
				// since a currency has been removed, we need to recalculate for each currency
				Self::calculate_individual_currency_unit_of_account(unit_of_account);
			}
		}

		Ok(())
	}

	fn update(
		symbol: Vec<u8>,
		issuance: Option<LedgerBalance>,
		price: Option<LedgerBalance>,
	) -> Result<(), DispatchError> {
		// we need to calculate the total inverse of the currencies in the  basket
		<CurrencyBasket<T, I>>::mutate(|basket| {
			if let Some(currency) = basket.iter_mut().find(|c| c.symbol == symbol) {
				if issuance.is_some() {
					currency.price = issuance.unwrap();
				}

				if price.is_some() {
					currency.price = price.unwrap();
				}
			}
		});

		// calculates the total_inverse_issuance(weights) in the basket, since a currency is removed
		let some_total_inverse_issuance = Self::calculate_total_inverse_issuance_in_basket();
		if let Some(total_inverse_issuance) = some_total_inverse_issuance {
			// recalculate weight for each currency in the basket, since a currency is removed
			Self::calculate_individual_weights(total_inverse_issuance);
			// newly calculated unit of account for the pallet
			if let Some(unit_of_account) = Self::calculate_unit_of_account() {
				UnitOfAccount::<T, I>::set(unit_of_account.clone());
				// since a currency has been removed, we need to recalculate for each currency
				Self::calculate_individual_currency_unit_of_account(unit_of_account);
			}
		}

		Ok(())
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	fn whitelisted_account_exists(account_id: T::AccountId) -> Option<bool> {
		let whitelisted_accounts = <WhitelistedAccounts<T, I>>::get();

		let is_whitelisted_account = whitelisted_accounts
			.iter()
			.any(|whitelisted_account| whitelisted_account.cmp(&account_id) == Ordering::Equal);

		Some(is_whitelisted_account)
	}

	fn symbol_exists(symbol: Vec<u8>) -> Option<bool> {
		let currency_basket = <CurrencyBasket<T, I>>::get();

		let does_symbol_exist =
			currency_basket.iter().any(|currency_details| currency_details.symbol == symbol);
		Some(does_symbol_exist)
	}

	pub fn calculate_individual_weights(total_inverse_issuance: f64) {
		let mut currency_basket = CurrencyBasket::<T, I>::get();

		for currency_details in currency_basket.iter_mut() {
			if let Some(currency_weight) = Self::calculate_weight_for_currency(
				total_inverse_issuance.clone(),
				currency_details.issuance.clone(),
			) {
				if let Some(weight_adjusted_price) = Self::calculate_weight_adjusted_price(
					currency_weight.clone(),
					currency_details.price,
				) {
					currency_details.weight = Some(currency_weight);
					currency_details.weight_adjusted_price = Some(weight_adjusted_price);
				}
			}
		}

		CurrencyBasket::<T, I>::set(currency_basket);
	}

	pub fn calculate_weight_adjusted_price(
		currency_weight: LedgerBalance,
		price: LedgerBalance,
	) -> Option<LedgerBalance> {
		let storage_value = convert_float_to_storage(
			convert_storage_to_float(currency_weight) * convert_storage_to_float(price),
		);

		Some(storage_value)
	}

	pub fn calculate_weight_for_currency(
		total_inverse_issuance_in_currency_basket: f64,
		issuance: LedgerBalance,
	) -> Option<LedgerBalance> {
		let currency_issuance_inverse = 1 as f64 / issuance as f64;

		let weight_of_currency =
			currency_issuance_inverse / total_inverse_issuance_in_currency_basket;

		let weight_of_currency = convert_float_to_storage(weight_of_currency);

		Some(weight_of_currency)
	}

	pub fn calculate_total_inverse_issuance_in_basket() -> Option<f64> {
		let unit_of_account_in_currency_basket: Vec<CurrencyDetails<T::MaxAssets>> =
			CurrencyBasket::<T, I>::get().into_iter().collect();

		let total_inverse_in_currency_basket: f64 = unit_of_account_in_currency_basket
			.iter()
			.fold(0 as f64, |acc, unit| acc + (1 as f64) / unit.issuance as f64);

		if total_inverse_in_currency_basket == 0.0 {
			return None
		} else {
			return Some(total_inverse_in_currency_basket)
		}
	}

	pub fn calculate_unit_of_account() -> Option<LedgerBalance> {
		let unit_of_account_in_currency_basket: Vec<CurrencyDetails<T::MaxAssets>> =
			CurrencyBasket::<T, I>::get().into_iter().collect();

		let unit_of_account =
			unit_of_account_in_currency_basket.iter().fold(0 as f64, |acc, unit| {
				acc + (convert_storage_to_float(unit.weight.unwrap()) *
					convert_storage_to_float(unit.price))
			});

		Some(convert_float_to_storage(unit_of_account))
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
