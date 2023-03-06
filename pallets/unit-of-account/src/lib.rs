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
	traits::{ 
		Currency, 
		ConstU32,
		ExistenceRequirement::{
			AllowDeath,
		},
	},
	BoundedVec,
	sp_runtime::traits::{ 
		Convert,
		UniqueSaturatedInto,
		BadOrigin,
	},
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
		AssetDetails, 
		UnitOfAccountInterface,
	}
};

// use core::cmp::Ordering;
pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	// use core::cmp::Ordering;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    // use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	// use totem_primitives::{unit_of_account::AssetDetails, LedgerBalance};

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		/// For transferring balances
		type Currency: Currency<Self::AccountId>;

		/// The max length of a whitelisted account Initial 50
		#[pallet::constant]
		type MaxWhitelistedAccounts: Get<u32>;

		/// The max number of assets allowed in the basket
		#[pallet::constant]
		type MaxAssetsInBasket: Get<u32>;

		/// The max number of characters in the symbol for the asset
		#[pallet::constant]
		type SymbolMaxChars: Get<u32>;
		
		/// The whitelisting deposit ammount
		#[pallet::constant]
		type WhitelistDeposit: Get<u32>;

		/// Weightinfo for pallet
		type WeightInfo: WeightInfo;

		/// For converting [u8; 32] bytes to AccountId
		type BytesToAccountId: Convert<[u8; 32], Self::AccountId>;

	}

	type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// The current storage version.
	const STORAGE_VERSION: frame_support::traits::StorageVersion =
	frame_support::traits::StorageVersion::new(1);
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

	
	/// The list of whitelisted accounts that can update the basket of assets
	#[pallet::storage]
	#[pallet::getter(fn whitelisted_accounts)]
	pub type WhitelistedAccounts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId, 
		(), 
		ValueQuery
		>;
		
		/// A counter so that whitelist does not exceed the max number of accounts
		/// Current Max is 256
		#[pallet::storage]
		#[pallet::getter(fn whitelisted_accounts_count)]
		pub type WhitelistedAccountsCount<T: Config> = StorageValue<
		_, 
		u32, 
		ValueQuery
		>;
		
		/// Holds an array of all assets in the basket and their details
		/// Reading the storage will return the entire array
		#[pallet::storage]
		#[pallet::getter(fn asset_basket)]
		pub type AssetBasket<T: Config> = StorageValue<
		_,
		BoundedVec<AssetDetails<T::SymbolMaxChars>, T::MaxAssetsInBasket>,
		ValueQuery,
		>;
		
		/// Holds an array of all assets by symbol
		/// Used to quickly determine if an asset is in the basket 
		/// without having to read the asset details
		#[pallet::storage]
		#[pallet::getter(fn asset_symbol)]
		pub type AssetSymbol<T: Config> = StorageValue<
		_,
		BoundedVec<BoundedVec<u8, T::SymbolMaxChars>, T::MaxAssetsInBasket>,
		ValueQuery,
		>;
		
	/// The calculated Nominal Effective Exchange Rate which is
	/// also known as the Unit of Account
	#[pallet::storage]
	#[pallet::getter(fn unit_of_account)]
	pub type UnitOfAccount<T: Config> = StorageValue<
		_,
		LedgerBalance, 
		ValueQuery
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Whitelist an account. 
		/// A white-listed account is able to update the price or issuance quantity in the basket.
		/// There are a limited number of accounts that can be added to the whitelist. Current limit is 50 accounts set as a parameter.
		/// A white-listed account must pay a security deposit of to be added to the whitelist. This can be found in metadata. 
		/// Security deposit will be returned when the account is removed from the whitelist.
		///
		/// Parameters:
		/// - `origin`: Any signed origin. This is also the account adding itself to the whitelist.
		#[pallet::weight(T::WeightInfo::whitelist_account())]
		#[pallet::call_index(0)]
		pub fn whitelist_account(
			origin: OriginFor<T>,
		) -> DispatchResultWithPostInfo {
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			let who = ensure_signed(origin)?;

			// Check that the number of whitelisted accounts has not already reached the maximum 
			let mut counter = Self::whitelisted_accounts_count();
			if counter >= T::MaxWhitelistedAccounts::get() {
				return Err(Error::<T>::MaxWhitelistedAccountOutOfBounds.into());
			} else {
				match Self::whitelisted_accounts(who.clone()) {
					() => return Err(Error::<T>::AlreadyWhitelistedAccount.into()),
					_ => {
						// TODO This performs computation. We should cache this address to storage and read.
						let deposit_account = T::BytesToAccountId::convert(*b"totems/whitelist/deposit/account");
						
						// Transfer 1000 KPX to the deposit account. If this process fails, then return error.
						T::Currency::transfer(
							&who,
							&deposit_account,
							T::WhitelistDeposit::get().unique_saturated_into(),
							AllowDeath,
						)?;
					},
				}
				counter += 1;
			}
			
			WhitelistedAccountsCount::<T>::set(counter);
			WhitelistedAccounts::<T>::set(who.clone(), ());

			Self::deposit_event(Event::AccountWhitelisted(who));
			Ok(().into())
		}

		/// Removes an account that is already whitelisted
		/// This can only be carried out by sudo or the whitelisted account itself
		/// When the account is removed it also releases the lock whitelisting security deposit
		/// This  
		///
		/// Parameters:
		/// - `origin`: A whitelisted account
		#[pallet::weight(T::WeightInfo::remove_account())]
		#[pallet::call_index(1)]
		pub fn remove_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			let who = ensure_signed(origin)?;

			// Check that the account exists in the whitelist
			match Self::whitelisted_accounts(who.clone()) {
				() => {
					// TODO This performs computation. We should cache this address to storage and read.
					let deposit_account = T::BytesToAccountId::convert(*b"totems/whitelist/deposit/account");
					
					// Transfer 1000 KPX to the account. If this process fails, then return error.
					T::Currency::transfer(
						&deposit_account,
						&who,
						T::WhitelistDeposit::get().unique_saturated_into(),
						AllowDeath,
					)?;
				},
				_ => return Err(Error::<T>::UnknownWhitelistedAccount.into()),
			}
			
			let mut counter = Self::whitelisted_accounts_count();
			// decrease the whitelist counter
			counter -= 1;
			
			WhitelistedAccountsCount::<T>::set(counter);
			// Then remove the account from the whitelist
			WhitelistedAccounts::<T>::set(who.clone(), ());

			Self::deposit_event(Event::AccountRemoved(who));

			Ok(().into())
		}

		/// Adds a single new asset to the basket of assets
		/// 
		///
		/// Parameters:
		/// - `origin`: A whitelisted callet origin
		/// - `symbol:` The asset symbol
		/// - `issuance`: The total asset issuance
		/// - `price`: The price in the base currency for the asset
		#[pallet::weight(T::WeightInfo::add_new_asset())]
		#[pallet::call_index(2)]
		pub fn add_new_asset(
			origin: OriginFor<T>,
			symbol: BoundedVec<u8, T::SymbolMaxChars>,
			issuance: u128,
			price: u128,
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;
			// check that the caller is whitelisted
			ensure!(WhitelistedAccounts::<T>::contains_key(&whitelisted_caller), Error::<T>::NotWhitelistedAccount);

			// TODO Ensure that the total number of assets is not greater than the maximum allowed
			// check the count of assets in the array

			// Ensure that the length of the symbol is not greater than the nr bytes in parameters
			ensure!(symbol.len() as u32 <= T::SymbolMaxChars::get(), Error::<T>::SymbolLengthOutOfBounds);

			// TODO Convert to uppercase to ensure that the symbol is unique and not case sensitive
			// Note this is not a rudimentary check. It needs to consider the UTF-8 encoding of the symbol characters and may require looping through the characters

			// Check that the symbol is not already in use.
			ensure!(!Self::asset_in_array(&symbol), Error::<T>::SymbolAlreadyExists);

			// check that the issuance is not zero
			ensure!(!issuance == u128::MIN, Error::<T>::InvalidIssuanceValue);

			// check that the price is not zero
			ensure!(!price == u128::MIN, Error::<T>::InvalidPriceValue);
			
			// Create the struct to hold the asset details and populate it
			let asset = AssetDetails<SymbolMaxChars: Get<u32>> {
				symbol: symbol.clone(),
				// convert the issuance to LedgerBalance
				issuance,
				// convert the issuance to LedgerBalance
				price,
			};

			// Update counter for the number of assets in the basket

			// add the symbol to the array of symbols

			// add the asset to the array of assets

			// Self::deposit_event(Event::CurrencyAddedToBasket(symbol));

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
			// let whitelisted_caller = ensure_signed(origin)?;

			// ensure!(
			// 	Self::whitelisted_account_exists(whitelisted_caller).unwrap_or(false),
			// 	Error::<T, I>::UnknownWhitelistedAccount
			// );

			// <Self as UnitOfAccountInterface>::remove(symbol.clone())?;

			// Self::deposit_event(Event::CurrencyRemovedFromTheBasket(symbol));

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
			maybe_issuance: Option<u128>,
			maybe_price: Option<u128>,
		) -> DispatchResultWithPostInfo {
			// let whitelisted_caller = ensure_signed(origin)?;

			// ensure!(
			// 	Self::whitelisted_account_exists(whitelisted_caller).unwrap_or(false),
			// 	Error::<T, I>::UnknownWhitelistedAccount
			// );

			// if let Some(issuance) = maybe_issuance {
			// 	ensure!(issuance != 0, Error::<T, I>::InvalidIssuanceValue);
			// }

			// if let Some(price) = maybe_price {
			// 	ensure!(price != 0, Error::<T, I>::InvalidPriceValue);
			// }

			// <Self as UnitOfAccountInterface>::update(symbol.clone(), maybe_issuance, maybe_price)?;

			Self::deposit_event(Event::CurrencyUpdatedInTheBasket(symbol));

			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
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
	pub enum Error<T> {
		/// Whitelisted account out of bounds
		MaxWhitelistedAccountOutOfBounds,
		/// Already Whitelisted account
		AlreadyWhitelistedAccount,
		/// Unknown whitelisted account
		UnknownWhitelistedAccount,
		/// Not a whitelisted account
		NotWhitelistedAccount,
		/// Max currencies exceeded
		// MaxCurrenciesOutOfBound,
		/// Symbol too long
		SymbolLengthOutOfBounds,
		/// Currency Symbol already exists
		SymbolAlreadyExists,
		/// Currency not found from basket
		AssetNotFound,
		/// Invalid Issuance Value
		InvalidIssuanceValue,
		/// Invalid Price Value
		InvalidPriceValue,
	}
}

// impl<T: Config> UnitOfAccountInterface for Pallet<T> {
// 	fn add(
// 		symbol: Vec<u8>,
// 		issuance: LedgerBalance,
// 		price: LedgerBalance,
// 	) -> Result<(), DispatchError> {
// 		let bounded_symbol = BoundedVec::<u8, T::MaxAssets>::try_from(symbol.clone())
// 			.map_err(|_e| Error::<T, I>::SymbolOutOfBound)?;

// 		let mut asset_basket = AssetBasket::<T>::get();
// 		// we need to calculate the total inverse of the currencies in the  basket
// 		let some_total_inverse_issuance_in_asset_basket =
// 			Self::calculate_total_inverse_issuance_in_basket();

// 		if let Some(total_inverse_issuance_in_asset_basket) =
// 			some_total_inverse_issuance_in_asset_basket
// 		{
// 			// calculate weight for the currency added
// 			if let Some(currency_weight) = Self::calculate_weight_for_currency(
// 				total_inverse_issuance_in_asset_basket.clone(),
// 				issuance.clone(),
// 			) {
// 				if let Some(weight_adjusted_price) =
// 					Self::calculate_weight_adjusted_price(currency_weight.clone(), price.clone())
// 				{
// 					let unit_of_account_currency = AssetDetails {
// 						symbol: bounded_symbol,
// 						issuance,
// 						price,
// 						weight: Some(currency_weight),
// 						weight_adjusted_price: Some(weight_adjusted_price),
// 						unit_of_account: None,
// 					};

// 					asset_basket
// 						.try_push(unit_of_account_currency)
// 						.map_err(|_e| Error::<T, I>::MaxCurrenciesOutOfBound)?;

// 					AssetBasket::<T>::set(asset_basket);

// 					// recalculate weight for each currency in the basket, since a new currency is just added
// 					Self::calculate_individual_weights(total_inverse_issuance_in_asset_basket);
// 					// newly calculated unit of account for the pallet
// 					if let Some(unit_of_account) = Self::calculate_unit_of_account() {
// 						UnitOfAccount::<T>::set(unit_of_account.clone());
// 						// since a new currency has been added, we need to recalculate for each currency
// 						Self::calculate_individual_currency_unit_of_account(unit_of_account);
// 					}
// 				}
// 			}
// 		} else {
// 			let unit_of_account_currency = AssetDetails {
// 				symbol: bounded_symbol,
// 				issuance,
// 				price,
// 				weight: None,
// 				weight_adjusted_price: None,
// 				unit_of_account: None,
// 			};

// 			asset_basket
// 				.try_push(unit_of_account_currency)
// 				.map_err(|_e| Error::<T, I>::MaxCurrenciesOutOfBound)?;

// 			AssetBasket::<T>::set(asset_basket);
// 		}

// 		Ok(())
// 	}

// 	fn remove(symbol: Vec<u8>) -> Result<(), DispatchError> {
// 		let mut currency_details = AssetBasket::<T>::get();
// 		let index = currency_details
// 			.iter()
// 			.position(|item| item.symbol == symbol)
// 			.ok_or_else(|| Error::<T, I>::CurrencyNotFoundFromBasket)?;
// 		currency_details.remove(index);

// 		AssetBasket::<T>::set(currency_details);

// 		// calculates the total_inverse_issuance(weights) in the basket, since a currency is removed
// 		let some_total_inverse_issuance = Self::calculate_total_inverse_issuance_in_basket();
// 		if let Some(total_inverse_issuance) = some_total_inverse_issuance {
// 			// recalculate weight for each currency in the basket, since a currency is removed
// 			Self::calculate_individual_weights(total_inverse_issuance);
// 			// newly calculated unit of account for the pallet
// 			if let Some(unit_of_account) = Self::calculate_unit_of_account() {
// 				UnitOfAccount::<T>::set(unit_of_account.clone());
// 				// since a currency has been removed, we need to recalculate for each currency
// 				Self::calculate_individual_currency_unit_of_account(unit_of_account);
// 			}
// 		}

// 		Ok(())
// 	}

// 	fn update(
// 		symbol: Vec<u8>,
// 		issuance: Option<u128>,
// 		price: Option<u128>,
// 	) -> Result<(), DispatchError> {
// 		// we need to calculate the total inverse of the currencies in the  basket
// 		<AssetBasket<T>>::mutate(|basket| {
// 			if let Some(currency) = basket.iter_mut().find(|c| c.symbol == symbol) {
// 				if issuance.is_some() {
// 					currency.price = issuance.unwrap();
// 				}

// 				if price.is_some() {
// 					currency.price = price.unwrap();
// 				}
// 			}
// 		});

// 		// calculates the total_inverse_issuance(weights) in the basket, since a currency is removed
// 		let some_total_inverse_issuance = Self::calculate_total_inverse_issuance_in_basket();
// 		if let Some(total_inverse_issuance) = some_total_inverse_issuance {
// 			// recalculate weight for each currency in the basket, since a currency is removed
// 			Self::calculate_individual_weights(total_inverse_issuance);
// 			// newly calculated unit of account for the pallet
// 			if let Some(unit_of_account) = Self::calculate_unit_of_account() {
// 				UnitOfAccount::<T>::set(unit_of_account.clone());
// 				// since a currency has been removed, we need to recalculate for each currency
// 				Self::calculate_individual_currency_unit_of_account(unit_of_account);
// 			}
// 		}

// 		Ok(())
// 	}
// }

impl<T: Config> Pallet<T> {
	fn asset_in_array(symbol: &BoundedVec<u8, T::SymbolMaxChars>) -> bool {
		let asset_symbol = <AssetSymbol<T>>::get();
		match asset_symbol.iter().any(|s| s == symbol) {
			true => true,
			false => false,
		} 
	}


	// 	fn whitelisted_account_exists(account_id: T::AccountId) -> Option<bool> {
// 		let whitelisted_accounts = <WhitelistedAccounts<T, I>>::get();

// 		let is_whitelisted_account = whitelisted_accounts
// 			.iter()
// 			.any(|whitelisted_account| whitelisted_account.cmp(&account_id) == Ordering::Equal);

// 		Some(is_whitelisted_account)
// 	}

// 	fn symbol_exists(symbol: Vec<u8>) -> Option<bool> {
// 		let asset_basket = <AssetBasket<T>>::get();

// 		let does_symbol_exist =
// 			asset_basket.iter().any(|currency_details| currency_details.symbol == symbol);
// 		Some(does_symbol_exist)
// 	}

// 	pub fn calculate_individual_weights(total_inverse_issuance: f64) {
// 		let mut asset_basket = AssetBasket::<T>::get();

// 		for currency_details in asset_basket.iter_mut() {
// 			if let Some(currency_weight) = Self::calculate_weight_for_currency(
// 				total_inverse_issuance.clone(),
// 				currency_details.issuance.clone(),
// 			) {
// 				if let Some(weight_adjusted_price) = Self::calculate_weight_adjusted_price(
// 					currency_weight.clone(),
// 					currency_details.price,
// 				) {
// 					currency_details.weight = Some(currency_weight);
// 					currency_details.weight_adjusted_price = Some(weight_adjusted_price);
// 				}
// 			}
// 		}

// 		AssetBasket::<T>::set(asset_basket);
// 	}

// 	pub fn calculate_weight_adjusted_price(
// 		currency_weight: LedgerBalance,
// 		price: LedgerBalance,
// 	) -> Option<LedgerBalance> {
// 		let storage_value = convert_float_to_storage(
// 			convert_storage_to_float(currency_weight) * convert_storage_to_float(price),
// 		);

// 		Some(storage_value)
// 	}

// 	pub fn calculate_weight_for_currency(
// 		total_inverse_issuance_in_asset_basket: f64,
// 		issuance: LedgerBalance,
// 	) -> Option<LedgerBalance> {
// 		let currency_issuance_inverse = 1 as f64 / issuance as f64;

// 		let weight_of_currency =
// 			currency_issuance_inverse / total_inverse_issuance_in_asset_basket;

// 		let weight_of_currency = convert_float_to_storage(weight_of_currency);

// 		Some(weight_of_currency)
// 	}

// 	pub fn calculate_total_inverse_issuance_in_basket() -> Option<f64> {
// 		let unit_of_account_in_asset_basket: Vec<AssetDetails<T::MaxAssets>> =
// 			AssetBasket::<T>::get().into_iter().collect();

// 		let total_inverse_in_asset_basket: f64 = unit_of_account_in_asset_basket
// 			.iter()
// 			.fold(0 as f64, |acc, unit| acc + (1 as f64) / unit.issuance as f64);

// 		if total_inverse_in_asset_basket == 0.0 {
// 			return None
// 		} else {
// 			return Some(total_inverse_in_asset_basket)
// 		}
// 	}

// 	pub fn calculate_unit_of_account() -> Option<LedgerBalance> {
// 		let unit_of_account_in_asset_basket: Vec<AssetDetails<T::MaxAssets>> =
// 			AssetBasket::<T>::get().into_iter().collect();

// 		let unit_of_account =
// 			unit_of_account_in_asset_basket.iter().fold(0 as f64, |acc, unit| {
// 				acc + (convert_storage_to_float(unit.weight.unwrap()) *
// 					convert_storage_to_float(unit.price))
// 			});

// 		Some(convert_float_to_storage(unit_of_account))
// 	}

// 	pub fn calculate_individual_currency_unit_of_account(unit_of_account: LedgerBalance) {
// 		let mut asset_basket = AssetBasket::<T>::get();

// 		for currency_details in asset_basket.iter_mut() {
// 			let price_f64 = convert_storage_to_float(currency_details.price);
// 			let unit_of_account_f64 = convert_storage_to_float(unit_of_account.clone());
// 			let unit_of_account_for_currency_f64 = price_f64 / unit_of_account_f64;
// 			let unit_of_account_for_currency =
// 				convert_float_to_storage(unit_of_account_for_currency_f64);
// 			currency_details.unit_of_account = Some(unit_of_account_for_currency);
// 		}

// 		AssetBasket::<T>::set(asset_basket);
// 	}
}
