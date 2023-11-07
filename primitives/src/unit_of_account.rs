mod permitted_assets;
pub use permitted_assets::{Tickers, *};
use frame_support::{
	dispatch::{
		TypeInfo
	},
	codec::{
		Encode,
		Decode,
		MaxEncodedLen,
	},
};

// This replaces STORAGE_MULTIPLIER because multiplying large floats causes overflow issues. This can be chained to avoid overflows.
// e.g. CONVERSION_FACTOR_F64 * CONVERSION_FACTOR_F64
pub const CONVERSION_FACTOR_F64: f64 = 1_000_000_000.0;

/// Holds the details for each asset for storage
#[derive(MaxEncodedLen, Debug, Encode, Decode, Copy, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub struct TickerDetails<BlockNumber> {
	/// The symbol of the asset
	pub symbol: Tickers,
	/// The total issuance of the asset converted
	pub issuance: u64,
	/// The price of the asset in base currency (e.g. USD, but later TODO can be any asset)
	pub price: u64,
	/// weighting_per_asset converted
	pub weighting_per_asset: u64,
	/// weight_adjusted_price in unit of account
	pub weight_adjusted_price: u64,
	/// uoa_per_asset converted
	pub uoa_per_asset: u64,
	/// display decimals (not to used for direct conversion)
	pub display_decimals: u8,
	/// block number of last update
	pub last_update_block: BlockNumber,
}

/// Holds the details for each asset for processing
#[derive(Debug, Encode, Decode, Copy, Clone, PartialEq)]
pub struct TickerData<BlockNumber> {
/// The symbol of the asset
pub st_symbol: Tickers,
/// The decimals that should be displayed (smallest available monetary unit)
pub st_display_decimals: u8,
/// The total issuance of the asset
pub st_issuance: u64,
/// Price for processing the basket	
pub price: f64,
pub weighting: f64,
pub st_integer_weighting: u64,
pub weight_adjusted_price: f64,
pub st_integer_weight_adjusted_price: u64,
pub unit_of_account: f64,
pub st_integer_unit_of_account: u64,
pub st_last_update_block: BlockNumber,
}