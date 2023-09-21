mod permitted_assets;
pub use permitted_assets::{Tickers, *};
use crate::LedgerBalance;
use frame_support::{
	pallet_prelude::*,
	dispatch::{
		TypeInfo
	},
};
// Used for converting floats 1.08 to 1_080_000_000_000 for correct storage as a LedgerBalance.
// The front end will apply 12 decimals when reading to return the value to 1.08
pub const STORAGE_MULTIPLIER: LedgerBalance = 1_000_000_000_000;

/// Holds the details for each asset for storage
#[derive(MaxEncodedLen, Clone, Decode, Encode, TypeInfo, Debug, PartialEq)]
#[scale_info(skip_type_params(SymbolMaxChars))]
pub struct TickerDetails {
	/// The symbol of the asset
	pub symbol: Tickers,
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
pub struct TickerData<T> {
	/// The symbol of the asset
	pub symbol: Tickers,
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
