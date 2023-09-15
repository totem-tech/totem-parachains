// use frame_support::dispatch;
// use sp_std::vec::Vec;
use crate::LedgerBalance;
use frame_support::{
	pallet_prelude::*,
	dispatch::{ 
		// DispatchResult, 
		// EncodeLike, 
		TypeInfo 
	}, 
};

pub const DIVISOR_UNIT: LedgerBalance = 100_000;
pub const STORAGE_MULTIPLIER: LedgerBalance = 100_000_000_000_000_000_000;

/// UnitOfAccount trait definition to be used in other pallets
// pub trait UnitOfAccountInterface {
// 	/// Registers a new asset symbol 
// 	fn add(
// 		symbol: Vec<u8>, 
// 		issuance: u128, 
// 		price: u128
// 	) -> Result<(), dispatch::DispatchError>;
	
// 	/// Removes an asset using a symbol.
// 	fn remove(
// 		symbol: Vec<u8>
// 	) -> Result<(), dispatch::DispatchError>;
	
// 	/// Updates a asset
// 	fn update(
// 		symbol: Vec<u8>, 
// 		issuance: Option<u128>, 
// 		price: Option<u128>
// 	) -> Result<(), dispatch::DispatchError>;
// }

/// Holds the details for each asset for storage
#[derive(MaxEncodedLen, Clone, Decode, Encode, TypeInfo, Debug, PartialEq)]
#[scale_info(skip_type_params(SymbolMaxChars))]
pub struct AssetDetails<SymbolMaxChars: Get<u32>> {
	/// The symbol of the asset
	pub symbol: BoundedVec<u8, SymbolMaxChars>,
	/// The total issuance of the asset converted
	pub issuance: LedgerBalance,
	/// The price of the asset in base currency (e.g. USD, but later TODO can be any asset)
	pub price: LedgerBalance,
	/// weighting_per_asset converted
	pub weighting_per_asset: LedgerBalance,
	/// weight_adjusted_price in unit of account
	pub weight_adjusted_price: LedgerBalance,
	/// uoa_per_asset converted
	pub uoa_per_asset: LedgerBalance,
}

/// Holds the details for each asset for processing
#[derive(Clone, Decode, Encode, Debug, PartialEq)]
pub struct AssetData<T, SymbolMaxChars: Get<u32>> {
	/// The symbol of the asset
	pub symbol: BoundedVec<u8, SymbolMaxChars>,
	/// The total issuance of the asset
	pub issuance: u128,
	pub inverse_issuance: Option<T>,
	/// The price of the asset in base currency (e.g. USD, but later TODO can be any asset)
	pub price: u128,
	/// weighting_per_asset = inverse_issuance / sum_total_inverse_issuances
	pub weighting_per_asset: Option<T>,
	/// weight_adjusted_price = weighting_per_asset * price_in_base_asset
	pub weight_adjusted_price: Option<T>,
	/// uoa_per_asset = price_in_base_asset / (100_000 * unit_of_account)
	pub uoa_per_asset: Option<T>,
}

// TODO Thes two functions should be moved to the file that contains all the unit conversion functions under the common directory.
pub fn convert_float_to_storage(amount: f64) -> LedgerBalance {
	// TODO This needs to be checked for overflow
	(amount * STORAGE_MULTIPLIER as f64) as LedgerBalance
}

pub fn convert_storage_to_float(amount: LedgerBalance) -> f64 {
	// TODO This needs to be checked for overflow
	amount as f64 / STORAGE_MULTIPLIER as f64
}
