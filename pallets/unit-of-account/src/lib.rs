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
//

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.

//! # Unit-Of-Account
//!
//! A module for calculating unit of account based on a basket of assets
//!
//! ## Overview
//!
//! The Unit-Of-Account module provides functionality for the following:
//!
//! * Add whitelisted account
//! * Remove whitelisted account
//! * Add new asset to the basket
//! * Remove asset from the basket
//! * updating the price of up to 100 assets in the basket
//! * updating the issuance up to 100 assets in the basket
//! * jointly updating all assets and prices
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Goals
//!
//! The Unit-Of-Account Pallet in Totem is designed to provide an exchange rate to the functional currency of the accounting engine.
//!
//! The purpose is that during the update of the accounts from another asset, the record will contain both the value of the asset
//! and the value in the functional currency, so that translation to the presentation currency can be done at the time of reporting.
//!

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
		Get,
		Currency,
		// ConstU32,
		ExistenceRequirement::{
			AllowDeath,
		},
	},
	BoundedVec,
	sp_runtime::{
		DispatchError,
		traits::{
			Convert,
			UniqueSaturatedInto,
			BadOrigin,
		},
	}
};
use sp_std::{
	convert::{TryInto},
	prelude::*,
};
use totem_primitives::{
	LedgerBalance,
	unit_of_account::{
		AssetDetails,
		AssetData,
	}
};

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

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

		/// The max number of assets allowed to be updated in one extrinsic
		#[pallet::constant]
		type MaxAssetsInput: Get<u32>;

		/// The max number of characters in the symbol for the asset
		#[pallet::constant]
		type SymbolMaxChars: Get<u32>;

		/// The whitelisting deposit ammount
		#[pallet::constant]
		type WhitelistDeposit: Get<u128>;

		/// Weightinfo for pallet
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type AccountBytes: Get<[u8; 32]>;

		/// For converting [u8; 32] bytes to AccountId  and f64 to LedgerBalance
		type UnitOfAccountConverter: Convert<[u8; 32], Self::AccountId> + Convert<f64, LedgerBalance>;
	}

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
	Option<()>,
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
	/// should vary with price changes, but should remain historically stable
	#[pallet::storage]
	#[pallet::getter(fn unit_of_account)]
	pub type UnitOfAccount<T: Config> = StorageValue<
	_,
	LedgerBalance,
	ValueQuery
	>;

	/// The Total Inverse Issuance of all assets
	/// This is also used as a cached value when new prices are updated
	#[pallet::storage]
	#[pallet::getter(fn total_inverse_issuance)]
	pub type TotalInverseIssuance<T: Config> = StorageValue<
	_,
	LedgerBalance,
	ValueQuery
	>;

	/// Deposit account which is a cached value to allow for faster read
	/// of the deposit account to hold funds for whitelisting an account
	#[pallet::storage]
	#[pallet::getter(fn deposit_account)]
	pub type DepositAccount<T: Config> = StorageValue<
		_,
		T::AccountId,
		OptionQuery
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
					Some(()) => return Err(Error::<T>::AlreadyWhitelistedAccount.into()),
					None => {
						let deposit_account = Self::get_deposit_account();

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
			WhitelistedAccounts::<T>::set(who.clone(), Some(()));

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
			account: Option<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			let account_to_whitelist;
			// If sudo then remove the account from the whitelist
			let is_sudo = ensure_root(origin.clone());
			if is_sudo.is_ok() {
				// checks if account to whitelist is valid
				if account.is_none() {
					return Err(Error::<T>::InvalidAccountToWhitelist.into())
				}
				account_to_whitelist = account.clone().unwrap();
				match Self::whitelisted_accounts(account.clone().unwrap()) {
					Some(()) => {
						let deposit_account =  Self::get_deposit_account();

						// Transfer 1000 KPX to the account. If this process fails, then return error.
						T::Currency::transfer(
							&deposit_account,
							&account.unwrap(),
							T::WhitelistDeposit::get().unique_saturated_into(),
							AllowDeath,
						)?;
					},
					None => return Err(Error::<T>::UnknownWhitelistedAccount.into()),
				}
			}  else {
				// else use the origin account to remove from the whitelist
				let who = ensure_signed(origin.clone())?;
				account_to_whitelist = who.clone();
				// Check that the account exists in the whitelist
				match Self::whitelisted_accounts(who.clone()) {
					Some(()) => {
						let deposit_account =  Self::get_deposit_account();

						// Transfer 1000 KPX to the account. If this process fails, then return error.
						T::Currency::transfer(
							&deposit_account,
							&who,
							T::WhitelistDeposit::get().unique_saturated_into(),
							AllowDeath,
						)?;
					},
					None => return Err(Error::<T>::UnknownWhitelistedAccount.into()),
				}
			}
			let mut counter = Self::whitelisted_accounts_count();
			// decrease the whitelist counter
			counter -= 1;

			WhitelistedAccountsCount::<T>::set(counter);
			// Then remove the account from the whitelist
			WhitelistedAccounts::<T>::remove(account_to_whitelist.clone());

			Self::deposit_event(Event::AccountRemoved(account_to_whitelist));

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
			// ------------- Checks ------------- //
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			// check that the origin is signed otherwise the extrinsic can be spammed
			let who = ensure_signed(origin)?;

			// check that the caller is whitelisted
			ensure!(WhitelistedAccounts::<T>::contains_key(&who), Error::<T>::NotWhitelistedAccount);

			// Ensure that the length of the symbol is not greater than the nr bytes in parameters
			// This line may be needed if the extrinsic does not reject a vec that is longer than SymbolMaxChars i.e. we need to test this
			// let symbol_ok = BoundedVec::<u8, T::SymbolMaxChars>::try_from(symbol.clone()).map_err(|_e| Error::<T>::SymbolLengthOutOfBounds)?;

			// Ensure that the total number of assets is not greater than the maximum allowed
			let mut assets = AssetSymbol::<T>::get();
			assets.try_push(symbol.clone()).map_err(|_| Error::<T>::AssetCannotBeAddedToBasket)?;

			// TODO Convert to uppercase to ensure that the symbol is unique and not case sensitive
			// Note this should not be a rudimentary check. It needs to consider the UTF-8 encoding of the symbol characters and the fact that each character is a u8

			// Check that the symbol is not already in use.
			ensure!(!Self::asset_in_array(&symbol), Error::<T>::SymbolAlreadyExists);

			// check that the issuance is not zero
			ensure!(issuance != u128::MIN, Error::<T>::InvalidIssuanceValue);

			// check that the price is not zero
			ensure!(price != u128::MIN, Error::<T>::InvalidPriceValue);

			// --------------- Processing ---------------- //
			// Create an intermediate basket including new asset
			let mut intermediate_basket = Self::combined_intermediate_basket(
				&symbol,
				&issuance,
				&price,
			);

			// get the total inverse issuance (f64)
			let tiv = Self::get_total_inverse_issuance(&intermediate_basket);

			// Partially recalculate the basket to get weighting_per_asset and weight_adjusted_price
			Self::partial_recalculation_of_basket(&mut intermediate_basket, tiv.clone());

			// from the updated basket calculate the unit of account
			let unit_of_account = Self::calculate_unit_of_account(&intermediate_basket);

			// final recalculations of the basket
			Self::final_recalculation_of_basket(&mut intermediate_basket, unit_of_account.clone());

			// Convert the new basket values to LedgerBalance types
			let new_basket = Self::conversion_of_basket_to_storage(intermediate_basket)?;

			// -------------- Update Storage ---------------- //
			// Add the symbol to the array of symbols in Storage
			AssetSymbol::<T>::set(assets);

			// Update the Unit of Account Value in Storage
			UnitOfAccount::<T>::set(Self::convert_float_to_storage(unit_of_account)?);

			// Update Total Inverse Issuance in Storage
			TotalInverseIssuance::<T>::set(Self::convert_float_to_storage(tiv)?);

			// Update the basket of assets in Storage
			AssetBasket::<T>::set(new_basket);

			Self::deposit_event(Event::AssetAddedToBasket(symbol));

			Ok(().into())
		}

		/// Removes asset from the basket
		/// Can only be performed by sudo because it does not remove the history of the asset
		/// nor does it guarantee that it will not be added back again in the future
		/// It should only be done in the exceptional cases where the storage limit is met and
		/// the community decides that it does not wish to extend the storage limit
		///
		/// Parameters:
		/// - `origin`: A sudo origin
		/// - `symbol:` The currency symbol to remove
		#[pallet::weight(T::WeightInfo::remove_currency())]
		#[pallet::call_index(3)]
		pub fn remove_asset(
			origin: OriginFor<T>,
			symbol: BoundedVec::<u8, T::SymbolMaxChars>,
		) -> DispatchResultWithPostInfo {
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			ensure_root(origin)?;

			let assets = Self::find_and_remove_asset(&symbol)?;

			let mut intermediate_basket = Self::reduced_intermediate_basket(
				&symbol,
			);

			let tiv = Self::get_total_inverse_issuance(&intermediate_basket);
			Self::partial_recalculation_of_basket(&mut intermediate_basket, tiv.clone());
			let unit_of_account = Self::calculate_unit_of_account(&intermediate_basket);
			Self::final_recalculation_of_basket(&mut intermediate_basket, unit_of_account.clone());
			let new_basket = Self::conversion_of_basket_to_storage(intermediate_basket)?;

			AssetSymbol::<T>::set(assets);
			UnitOfAccount::<T>::set(Self::convert_float_to_storage(unit_of_account)?);
			TotalInverseIssuance::<T>::set(Self::convert_float_to_storage(tiv)?);
			AssetBasket::<T>::set(new_basket);

			Self::deposit_event(Event::AssetRemoved(symbol));

			Ok(().into())
		}

		/// Updates multiple assets in the basket
		/// The input should be a boundedvec of values and must be limited to only 100 assets
		/// All the assets to update must exist or else the transaction fail for all assets to be updated
		/// An event can be used to communicate to the Front-end which asset caused the issue
		///
		/// Parameters:
		/// - `origin`: A whitelisted callet origin
		/// - `symbol:` The currency symbol to remove
		/// - `issuance:` The new currency issuance which can be None if not set
		/// - `price:` The new currency price which can be None if not set
		#[pallet::weight(T::WeightInfo::update_currency())]
		#[pallet::call_index(4)]
		pub fn update_currency(
			_origin: OriginFor<T>,
			// assets: Vec<AssetDetails<T::SymbolMaxChars>>,
		) -> DispatchResultWithPostInfo {
			if ensure_none(_origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			// Self::deposit_event(Event::CurrencyUpdatedInTheBasket(symbol));

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
		/// Asset added to the basket
		AssetAddedToBasket(BoundedVec<u8, T::SymbolMaxChars>),
		/// Asset removed from the basket
		AssetRemoved(BoundedVec<u8, T::SymbolMaxChars>),
		/// Asset updated in the basket
		AssetUpdated(BoundedVec<u8, T::SymbolMaxChars>),
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
		/// Max assets exceeded
		MaxAssetsOutOfBounds,
		/// Symbol too long
		SymbolLengthOutOfBounds,
		/// Asset Symbol already exists
		SymbolAlreadyExists,
		/// Asset cannot be added to the basket
		AssetCannotBeAddedToBasket,
		/// Asset not found during removal
		AssetNotFound,
		/// Invalid Issuance Value
		InvalidIssuanceValue,
		/// Invalid Price Value
		InvalidPriceValue,
		/// Conversion from basket failed!
		TryConvertFailed,
		/// Invalid Account To Whitelist
		InvalidAccountToWhitelist,
		/// Overflow error when converting float to storage value
		OverflowErrorConversionToStorage
	}
}


impl<T: Config> Pallet<T> {
	fn asset_in_array(symbol: &BoundedVec<u8, T::SymbolMaxChars>) -> bool {
		let asset_symbol = <AssetSymbol<T>>::get();
		match asset_symbol.iter().any(|s| s == symbol) {
			true => true,
			false => false,
		}
	}

	fn find_and_remove_asset(
		symbol: &BoundedVec<u8, T::SymbolMaxChars>,
	) -> Result<BoundedVec<BoundedVec<u8, T::SymbolMaxChars>, T::MaxAssetsInBasket>, DispatchError> {
		let mut assets = AssetSymbol::<T>::get();
		match assets.iter().position(|s| s == symbol) {
			Some(idx) => {
				assets.remove(idx);
				Ok(assets)
			},
			None => Err(Error::<T>::AssetNotFound.into())
		}
	}

	fn invert_issuance(issuance: u128) -> Option<f64> {
		let inverted_issuance = 1u128 as f64 / issuance as f64;
		return Some(inverted_issuance)
	}

	fn get_total_inverse_issuance(basket: &Vec<AssetData<f64, T::SymbolMaxChars>>) -> f64 {
		let total_inverse_in_asset_basket = basket
		.iter()
		.fold(0.0f64, |acc, asset| acc + match asset.inverse_issuance {
			Some(iv) => iv,
			None => 0.0f64
		});
		return total_inverse_in_asset_basket
	}

	fn partial_recalculation_of_basket(basket: &mut Vec<AssetData<f64, T::SymbolMaxChars>>, tiv: f64) -> &mut Vec<AssetData<f64, T::SymbolMaxChars>> {
		for asset in &mut *basket {
			asset.weighting_per_asset = match asset.inverse_issuance.clone() {
				Some(i) => Some(i / tiv.clone()),
				None => None
			};
			asset.weight_adjusted_price = match asset.weighting_per_asset.clone() {
				Some(w) => Some(w * asset.price as f64),
				None => None
			};
		}

		return basket
	}

	fn final_recalculation_of_basket(basket: &mut Vec<AssetData<f64, T::SymbolMaxChars>>, uoa: f64) -> &mut Vec<AssetData<f64, T::SymbolMaxChars>> {
		for asset in &mut *basket {
			asset.uoa_per_asset = match asset.weight_adjusted_price.clone() {
				Some(u) => Some(u / uoa.clone()),
				None => None
			};
		}

		return basket
	}

	fn calculate_unit_of_account(basket: &Vec<AssetData<f64, T::SymbolMaxChars>>) -> f64 {
		let unit_of_account = basket
		.iter()
		.fold(0.0f64, |acc, asset| acc + match asset.weight_adjusted_price {
			Some(wap) => wap,
			None => 0.0f64
		});

		return unit_of_account
	}

	fn conversion_of_basket_to_storage(
		basket: Vec<AssetData<f64, T::SymbolMaxChars>>,
	) -> Result<BoundedVec::<AssetDetails<T::SymbolMaxChars>, T::MaxAssetsInBasket>, DispatchError> {
		let mut new_basket: BoundedVec::<AssetDetails<T::SymbolMaxChars>, T::MaxAssetsInBasket> = Default::default();
			for asset in basket {
				let converted_entry = AssetDetails {
					symbol: asset.symbol.clone(),
					issuance: asset.issuance as LedgerBalance,
					price: asset.price as LedgerBalance,
					weighting_per_asset: match asset.weighting_per_asset {
						Some(wpa) => Self::convert_float_to_storage(wpa)?,
						None => 0 as LedgerBalance,
					},
					weight_adjusted_price: match asset.weight_adjusted_price {
						Some(wap) => Self::convert_float_to_storage(wap)?,
						None => 0 as LedgerBalance,
					},
					uoa_per_asset: match asset.uoa_per_asset {
						Some(uoa) => Self::convert_float_to_storage(uoa)?,
						None => 0 as LedgerBalance,
					}
				};
				new_basket.try_push(converted_entry).map_err(|_| Error::<T>::MaxAssetsOutOfBounds)?;
			}

		Ok(new_basket)
	}

	fn combined_intermediate_basket(
		symbol: &BoundedVec<u8, T::SymbolMaxChars>,
		issuance: &u128,
		price: &u128,
	) -> Vec<AssetData<f64, T::SymbolMaxChars>> {

		let current_asset_basket = AssetBasket::<T>::get();
		let mut intermediate_basket = Vec::new();

		let new_entry = AssetData {
			symbol: symbol.clone(),
			issuance: issuance.clone(),
			inverse_issuance: Self::invert_issuance(issuance.clone()),
			price: price.clone(),
			weighting_per_asset: None,
			weight_adjusted_price: None,
			uoa_per_asset: None,
		};
		intermediate_basket.push(new_entry);

		// Move data to new array erasing values that are to be recalculated
		for asset in current_asset_basket {
			let existing_entry = AssetData {
				symbol: asset.symbol.clone(),
				issuance: asset.issuance.clone() as u128,
				inverse_issuance: Self::invert_issuance(asset.issuance as u128),
				price: asset.price as u128,
				weighting_per_asset: None,
				weight_adjusted_price: None,
				uoa_per_asset: None,
			};
			intermediate_basket.push(existing_entry);
		}
		intermediate_basket
	}

	fn reduced_intermediate_basket(
		symbol: &BoundedVec<u8, T::SymbolMaxChars>,
	) -> Vec<AssetData<f64, T::SymbolMaxChars>> {
		let current_asset_basket = AssetBasket::<T>::get();
		let mut intermediate_basket = Vec::new();
		// Move data to new array erasing values that are to be recalculated
		for asset in current_asset_basket {
			if asset.symbol == *symbol {
				continue
			}
			let existing_entry = AssetData {
				symbol: asset.symbol.clone(),
				issuance: asset.issuance.clone() as u128,
				inverse_issuance: Self::invert_issuance(asset.issuance as u128),
				price: asset.price as u128,
				weighting_per_asset: None,
				weight_adjusted_price: None,
				uoa_per_asset: None,
			};
			intermediate_basket.push(existing_entry);
		}
		intermediate_basket
	}

	fn get_deposit_account() -> T::AccountId {
		let deposit_account;
		if DepositAccount::<T>::get().is_some() {
			deposit_account = DepositAccount::<T>::get().unwrap();
		} else {
			deposit_account = T::UnitOfAccountConverter::convert(T::AccountBytes::get());
			DepositAccount::<T>::put(deposit_account.clone());
		}

		deposit_account
	}

	fn convert_float_to_storage(amount: f64) -> Result<LedgerBalance, DispatchError> {
		let converted_amount = T::UnitOfAccountConverter::convert(amount);

		let max_ledger_balance= LedgerBalance::MAX;
		if converted_amount > max_ledger_balance {
			return Err(Error::<T>::OverflowErrorConversionToStorage.into());
		}

		Ok(converted_amount)
	}
}
