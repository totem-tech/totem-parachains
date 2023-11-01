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

// Copyright 2023 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
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
//! * updating the price of a single in the basket
//! * updating the issuance a single in the basket
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Goals
//!
//! The Unit-Of-Account Pallet in Totem is designed to provide an exchange rate to the functional currency of the accounting engine.
//!
//! The purpose is that during the update of the accounting record only the functional currency is important.
//! A transactional currency is noted in the record by its exchange rate from the functional currency, and to do this a lookup is performed 
//! to obtain the exchange rate for the `applicable_period_blocknumber` that the transaction occured.
//! In most cases this will be the current block, but in the case of retrospective updates to the accounting record this may be in the 
//! past.
//! This enables the option of using the presentation currency mechanism in the front-end, but will also allow the display of values 
//! for each accounting record at the time of the transaction.
//! The main goal therefore is to keep a continuous record of the exchange rate for the functional currency per block.
//! The changes are likely to be very small so that the functional currency remains a stable unit of account.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// mod benchmarking;
// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

	use frame_support::{
		ensure,
		pallet_prelude::*,
		dispatch::{
			result::{
				Result
			}, 
		},
		traits::{
			Get,
			Currency,
			ExistenceRequirement::{
				AllowDeath,
			},
		},
		sp_runtime::{
			DispatchError,
			traits::{
				Convert,
				BadOrigin,
				UniqueSaturatedInto,
			},
		},
	};
	use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;
	use sp_std::vec::Vec;

	// pub use weights::WeightInfo;
	pub use crate::weights::WeightInfo;

	use totem_primitives::{
		unit_of_account::{
			TickerDetails,
			TickerData,
			Tickers,
			CONVERSION_FACTOR_F64,
		},
	};

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
	Blake2_128Concat, T::AccountId,
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

	/// Always contains an updated list of Tickers and their details at any point in time.
	/// This means that state history will contain previous values of the basket.
	#[pallet::storage]
	#[pallet::getter(fn basket)]
	pub type Basket<T: Config> = StorageValue<
	_,
	Option<BoundedVec<TickerDetails, T::TickersLimit>>,
	ValueQuery
	>;

	/// The calculated Nominal Effective Exchange Rate which is
	/// also known as the Financial Index from which the unit of account for each currency is calculated
	/// should vary little with price changes, and should remain historically stable relative to all currencies
	#[pallet::storage]
	#[pallet::getter(fn financial_index)]
	pub type FinancialIndex<T: Config> = StorageValue<
	_,
	u64,
	ValueQuery
	>;

	/// The Total Inverse Issuance of all assets
	/// This is also used as a cached value when new prices are updated
	#[pallet::storage]
	#[pallet::getter(fn total_inverse_issuance)]
	pub type TotalInverseIssuance<T: Config> = StorageValue<
	_,
	u64,
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
	
	#[pallet::config]
	/// The module configuration trait.
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		/// For transferring balances
		type Currency: Currency<Self::AccountId>;
		
		/// The max length of a whitelisted account Initial 50
		#[pallet::constant]
		type MaxWhitelistedAccounts: Get<u32>;
		
		/// The max number of assets allowed to be updated in one extrinsic
		#[pallet::constant]
		type TickersLimit: Get<u32>;
		
		/// The whitelisting deposit ammount
		#[pallet::constant]
		type WhitelistDeposit: Get<u128>;
		
		/// used to convert bytes to address format
		#[pallet::constant]
		type AccountBytes: Get<[u8; 32]>;
		
		/// For converting [u8; 32] bytes to AccountId  and f64 to LedgerBalance
		type UnitOfAccountConverter: Convert<[u8; 32], Self::AccountId>; 
		
		/// Weightinfo for pallet
		type WeightInfo: WeightInfo;
	}
		
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Account Whitelisted
		AccountWhitelisted(T::AccountId),
		/// Account removed from whitelisted accounts
		AccountRemoved(T::AccountId),
		/// Asset added to the basket
		AssetAddedToBasket(Tickers),
		/// Asset removed from the basket
		AssetRemoved(Tickers),
		/// Asset price updated in the basket
		AssetPriceUpdated(Tickers),
		/// Asset issuance updated in the basket
		AssetIssuanceUpdated(Tickers),
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
		/// Invalid Issuance Value
		InvalidIssuanceValue,
		/// Invalid Price Value
		InvalidPriceValue,
		/// Invalid Account To Whitelist
		InvalidAccountToWhitelist,
		/// Problem computing the inverse issuance
		ErrorInverseIssuance,
		/// Problem computing weights
		ErrorComputingWeights,
		/// Problem applying weights to prices
		ErrorApplyingWeights,
		/// Problem computing the financial inex
		ErrorFinancialIndex,
		/// Problem computing the Unit of Account
		ErrorComputingUoA,
		/// Problem converting the Intermediate basket
		ErrorConvertingBasket,
		/// Error gettting the basket from storage
		ErrorGettingBasket,
		/// Conversion Overflow error
		ConversionOverflow,
		/// Error updating storage
		ErrorUpdatingStorage,
		/// Error removing asset
		ErrorRemovingAsset,
		/// Error intermediate basket is empty
		EmptyIntermediateBasket,
		/// Conversion error to Bounded Vec
		VecConversion,
	}
		
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Whitelists an account, allowing it to be used to provide updated values to the basket
		/// Requires a locked security deposit of 1000 KPX to be whitelisted
		///
		/// Parameters:
		/// - `origin`: The account to be whitelisted
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::whitelist_account())]
		pub fn whitelist_account(
			origin: OriginFor<T>,
		) -> DispatchResult {
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
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::remove_account())]
		pub fn remove_account(
			origin: OriginFor<T>,
			account: Option<T::AccountId>,
		) -> DispatchResult {
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
		
		/// Adds a new asset to the basket
		/// Can only be performed by a whitelisted caller
		/// The asset must not already exist in the basket
		/// The issuance and price values must not be zero
		///
		/// Parameters:
		/// - `origin`: A whitelisted caller origin
		/// - `symbol`: The currency symbol of the asset to add
		/// - `issuance`: The issuance value of the asset
		/// - `price`: The price value of the asset. Must be converted to u64 by multiplying by the conversion factor squared
		/// - `display_decimals`: The number of decimal places to display for the asset
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::add_new_asset())]
		pub fn add_new_asset(
			origin: OriginFor<T>,
			symbol: Tickers,
			issuance: u64,
			price: u64,
			display_decimals: u8,
		) -> DispatchResult {
			// ------------- Checks ------------- //
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			// check that the origin is signed otherwise the extrinsic can be spammed
			let who = ensure_signed(origin)?;
			// check that the caller is whitelisted
			ensure!(WhitelistedAccounts::<T>::contains_key(&who), Error::<T>::NotWhitelistedAccount);
			// check that the issuance is not zero
			ensure!(issuance != u64::MIN, Error::<T>::InvalidIssuanceValue);
			// check that the price is not zero
			ensure!(price != u64::MIN, Error::<T>::InvalidPriceValue);			
			let mut intermediate_basket: Vec<TickerData> = Vec::new();
			Self::get_intermediate_basket(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorGettingBasket)?;
			// Add the new asset to the intermediary vec
			intermediate_basket.push(TickerData {
				st_symbol: symbol.clone(), // also used in event
				st_display_decimals: display_decimals,
				st_issuance: issuance,
				price: Self::convert_int_to_float(price),
				weighting: 0.0,
				st_integer_weighting: 0,
				weight_adjusted_price: 0.0,
				st_integer_weight_adjusted_price: 0,
				unit_of_account: 0.0,
				st_integer_unit_of_account: 0,
			});
			// use process and update storage using the intermediate_basket
			Self::update_from_intermediate(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorUpdatingStorage)?;
			
			Self::deposit_event(Event::AssetAddedToBasket(symbol));
			
			Ok(().into())
		}
		
		/// Removes asset from the basket
		/// Can only be performed by sudo because it does not remove the history of the asset
		/// nor does it guarantee that it will not be added back again in the future
		/// It should only be done in the exceptional cases
		///
		/// Parameters:
		/// - `origin`: A sudo origin
		/// - `symbol:` The currency symbol to remove
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::remove_asset())]
		pub fn remove_asset(
			origin: OriginFor<T>,
			symbol: Tickers,
		) -> DispatchResult {
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			ensure_root(origin)?;
			
			let mut intermediate_basket: Vec<TickerData> = Vec::new();
			Self::get_intermediate_basket(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorGettingBasket)?;
			// Remove the asset from the intermediate basket
			intermediate_basket.retain(|x| x.st_symbol != symbol);
			Self::update_from_intermediate(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorUpdatingStorage)?;
			
			Self::deposit_event(Event::AssetRemoved(symbol));
			
			Ok(().into())
		}
		
		/// Updates an asset's price in the basket
		/// The price value must not be zero
		///
		/// Parameters:
		/// - `origin`: A whitelisted caller origin
		/// - `symbol:` The asset symbol to update
		/// - `price:` The new price of the asset - this must be converted to u64 by multiplying by the conversion factor squared
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::update_asset_price())]
		pub fn update_asset_price(
			origin: OriginFor<T>,
			symbol: Tickers,
			price: u64,
		) -> DispatchResult {
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			// check that the origin is signed otherwise the extrinsic can be spammed
			let who = ensure_signed(origin)?;
			// check that the caller is whitelisted
			ensure!(WhitelistedAccounts::<T>::contains_key(&who), Error::<T>::NotWhitelistedAccount);
			// check that the price is not zero
			ensure!(price != u64::MIN, Error::<T>::InvalidPriceValue);			
			let mut intermediate_basket: Vec<TickerData> = Vec::new();
			Self::get_intermediate_basket(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorGettingBasket)?;
			
			// If the asset exists, update its price in the intermediate basket directly.
			// The extrinsic cannot accept f64 as input so it needs to be converted.
			for asset in &mut intermediate_basket {
				asset.price = Self::convert_int_to_float(price);
			}
			
			// use process and update storage using the intermediate_basket
			Self::update_from_intermediate(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorUpdatingStorage)?;
			
			Self::deposit_event(Event::AssetPriceUpdated(symbol));
			Ok(().into())
		}
		
		/// Updates an asset's issuance in the basket
		/// The issuance value must not be zero
		///
		/// Parameters:
		/// - `origin`: A whitelisted caller origin
		/// - `symbol:` The asset symbol to update
		/// - `issuance:` The new issuance of the asset
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::update_asset_issuance())]
		pub fn update_asset_issuance(
			origin: OriginFor<T>,
			symbol: Tickers,
			issuance: u64,
		) -> DispatchResult {
			if ensure_none(origin.clone()).is_ok() {
				return Err(BadOrigin.into())
			}
			// check that the origin is signed otherwise the extrinsic can be spammed
			let who = ensure_signed(origin)?;
			// check that the caller is whitelisted
			ensure!(WhitelistedAccounts::<T>::contains_key(&who), Error::<T>::NotWhitelistedAccount);
			// check that the issuance is not zero
			ensure!(issuance != u64::MIN, Error::<T>::InvalidIssuanceValue);		
			let mut intermediate_basket: Vec<TickerData> = Vec::new();
			Self::get_intermediate_basket(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorGettingBasket)?;
			
			// If the asset exists, update its issuance in the intermediate basket directly.
			for asset in &mut intermediate_basket {
				if asset.st_symbol == symbol {
					asset.st_issuance = issuance;
					break;
				}
			}
			
			// use process and update storage using the intermediate_basket
			Self::update_from_intermediate(&mut intermediate_basket).map_err(|_| Error::<T>::ErrorUpdatingStorage)?;
			
			Self::deposit_event(Event::AssetIssuanceUpdated(symbol));
			Ok(().into())
		}
	}
	
	impl<T: Config> Pallet<T> {
		/// Utility to convert a f64 price to a u64 price
		fn convert_float_to_int(price: f64) -> u64 {
			// Multiply f64 by a large float conversion factor without truncation
			let price_float = price * CONVERSION_FACTOR_F64 * CONVERSION_FACTOR_F64;
				
			// Round the result to the nearest integer
			let price_u64 = price_float.round() as u64;
			return price_u64;
		}
		
		/// Utility to convert a u64 price to a f64 price
		fn convert_int_to_float(price: u64) -> f64 {
			// Divide u64 by a large float conversion factor without truncation and cast to f64
			let price_f64 = price as f64 / CONVERSION_FACTOR_F64 / CONVERSION_FACTOR_F64 ;
			
			return price_f64;
		}

		/// Gets the deposit account from storage. This is more efficient than recalculating it every time.
		fn get_deposit_account() -> T::AccountId {
			DepositAccount::<T>::get().unwrap_or_else(|| {
				let get_deposit_account = T::UnitOfAccountConverter::convert(T::AccountBytes::get());
				DepositAccount::<T>::put(get_deposit_account.clone());
				get_deposit_account
			})
		}

		/// Utility to get the current basket from storage
		fn get_intermediate_basket(intermediate_basket: &mut Vec<TickerData>) -> Result<(), DispatchError> {
			// read the Basket storage into a new array
			match Self::basket() {
				Some(basket) => {
					*intermediate_basket = basket.into_iter().map(|x| {
						TickerData {
							st_symbol: x.symbol,
							st_display_decimals: x.display_decimals,
							st_issuance: x.issuance,
							price: Self::convert_int_to_float(x.price),
							weighting: 0.0,
							st_integer_weighting: 0,
							weight_adjusted_price: 0.0,
							st_integer_weight_adjusted_price: 0,
							unit_of_account: 0.0,
							st_integer_unit_of_account: 0,
						}
					}).collect::<Vec<TickerData>>();
				},
				None => intermediate_basket.clear(),
			};
			
			Ok(())
		}

		/// Conversion of the basket from float to integer for storage
		fn conversion_of_basket_to_storage(
			basket: &Vec<TickerData>,
		) -> Result<BoundedVec<TickerDetails, T::TickersLimit>, DispatchError> {
			let mut new_basket: Vec<TickerDetails> = Default::default();
			for asset in basket {
				let converted_entry = TickerDetails {
					symbol: asset.st_symbol,
					issuance: asset.st_issuance,
					price: Self::convert_float_to_int(asset.price),
					weighting_per_asset: asset.st_integer_weighting,
					weight_adjusted_price: asset.st_integer_weight_adjusted_price,
					uoa_per_asset: asset.st_integer_unit_of_account,
					display_decimals: asset.st_display_decimals,
				};
				new_basket.push(converted_entry);
			}
			let bounded_vec: BoundedVec<TickerDetails, T::TickersLimit> = new_basket.try_into()
			.map_err(|_| Error::<T>::VecConversion)?;
	
			Ok(bounded_vec)
		}
		
		/// Computes the value of weights for every currency using the given formula.
		fn compute_weights(intermediate_basket: &mut Vec<TickerData>, sum_inverse_issuance: f64) -> Option<()> {
			if sum_inverse_issuance.is_finite() {
				for ticker_data in intermediate_basket.iter_mut() {
					let weight = (1.0 / ticker_data.st_issuance as f64) / sum_inverse_issuance;
					
					// Multiply f64 by a large float conversion factor without truncation
					let weighted_float = weight * CONVERSION_FACTOR_F64 * CONVERSION_FACTOR_F64;
					
					// Round the result to the nearest integer
					let integer_weighting = weighted_float.round() as u64;
					
					ticker_data.weighting = weight;
					ticker_data.st_integer_weighting = integer_weighting;
				}
				Some(())
			} else {
				None
			}
		}

		/// Computes the sum of the inverse of the issuance of all assets.
		fn sum_issuances(intermediate_basket: &Vec<TickerData>) -> Option<f64> {
			if intermediate_basket.is_empty() {
				None
			} else {
				let mut sum_inverse_issuance: f64 = 0.0;
				for ticker_data in intermediate_basket {
					sum_inverse_issuance += 1.0 / ticker_data.st_issuance as f64;
				}
				Some(sum_inverse_issuance)
			}
		}    
		
		/// Computes the sum of the units of account of all assets creating an financial index.
		fn sum_units_creating_financial_index(intermediate_basket: &Vec<TickerData>) -> Option<f64> {
			if intermediate_basket.is_empty() {
				None
			} else {
				let mut sum_units: f64 = 0.0;
				for ticker_data in intermediate_basket {
					sum_units += ticker_data.weight_adjusted_price;
				}
				// sum_units = sum_units * CONVERSION_FACTOR_F64 * CONVERSION_FACTOR_F64;
				Some(sum_units)
			}
		}
		
		/// Applies the weights to the exchange rates of all assets to create the weight_adjusted_price per currency.
		fn apply_weights_to_prices(intermediate_basket: &mut Vec<TickerData>) -> Option<()> {
			if intermediate_basket.is_empty() {
				None
			} else {
				
				for ticker_data in intermediate_basket.iter_mut() {
					let weight_adjusted_price = ticker_data.price * ticker_data.weighting;
					
					// Multiply f64 by a large float conversion factor without truncation
					let weight_adjusted_price_float = weight_adjusted_price * CONVERSION_FACTOR_F64 * CONVERSION_FACTOR_F64;
					
					// Round the result to the nearest integer
					let integer_weight_adjusted_price = weight_adjusted_price_float.round() as u64;
					
					ticker_data.weight_adjusted_price = weight_adjusted_price;
					ticker_data.st_integer_weight_adjusted_price = integer_weight_adjusted_price;
				}
				Some(())
			}
		}
		
		/// Creates the new unit of account per currency
		fn compute_units_of_account(intermediate_basket: &mut Vec<TickerData>, financial_index: f64) -> Option<()> {
			if intermediate_basket.is_empty() {
				None
			} else {
				
				for ticker_data in intermediate_basket.iter_mut() {
					let unit_of_account = ticker_data.weight_adjusted_price / financial_index;
					
					// Multiply f64 by a large float conversion factor without truncation
					let unit_of_account_float = unit_of_account * CONVERSION_FACTOR_F64 * CONVERSION_FACTOR_F64;
					
					// Round the result to the nearest integer
					let integer_unit_of_account = unit_of_account_float.round() as u64;
					
					ticker_data.unit_of_account = unit_of_account;
					ticker_data.st_integer_unit_of_account = integer_unit_of_account;
				}
				Some(())
			}
		}
		
		/// Handles processing on intermediate basket and returns financial_index: f64 and a new_basket: Vec<TickerDetails> 
		fn update_from_intermediate(intermediate_basket: &mut Vec<TickerData>) -> Result<(), DispatchError> {
			if intermediate_basket.is_empty() {
				Err(Error::<T>::EmptyIntermediateBasket.into())
			} else {
				
				// Get the sum of the inverse of the issuance of all assets.
				let sum_inverse_issuance = match Self::sum_issuances(&intermediate_basket) {
					Some(sum) => sum,
					None => return Err(Error::<T>::ErrorInverseIssuance.into()),
				};
				
				// Compute the weight of each asset.
				if Self::compute_weights(intermediate_basket, sum_inverse_issuance.clone()).is_none() {
					return Err(Error::<T>::ErrorComputingWeights.into());
				}
				
				// Apply weights to prices.
				if Self::apply_weights_to_prices(intermediate_basket).is_none() {
					return Err(Error::<T>::ErrorApplyingWeights.into());
				}
				
				// generate financial index.
				let financial_index = Self::sum_units_creating_financial_index(&intermediate_basket).ok_or(Error::<T>::ErrorFinancialIndex)?;

				// Compute units of account.
				if Self::compute_units_of_account(intermediate_basket, financial_index.clone()).is_none() {
					return Err(Error::<T>::ErrorComputingUoA.into());
				}
				
				// Convert the intermediate_basket to the format required for storage.
				// let new_basket = Self::conversion_of_basket_to_storage(intermediate_basket).ok_or(Error::<T>::ErrorConvertingBasket)?;
				let new_basket = Self::conversion_of_basket_to_storage(intermediate_basket).map_err(|_| Error::<T>::ErrorConvertingBasket)?;
				// Update the Unit of Account Value in Storage
				FinancialIndex::<T>::set(Self::convert_float_to_int(financial_index));
				
				// Update Total Inverse Issuance in Storage
				TotalInverseIssuance::<T>::set(Self::convert_float_to_int(sum_inverse_issuance));
				
				// Update the basket of assets in Storage
				Basket::<T>::set(Some(new_basket));
				
				Ok(())
			}
		}
	}
}