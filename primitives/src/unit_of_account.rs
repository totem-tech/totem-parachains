mod permitted_assets;
pub use permitted_assets::{Assets, *};
use crate::LedgerBalance;
use frame_support::{
	pallet_prelude::*,
	dispatch::{
		TypeInfo
	},
};

pub const DIVISOR_UNIT: LedgerBalance = 100_000;
pub const STORAGE_MULTIPLIER: LedgerBalance = 100_000_000_000_000_000_000;

/// Holds the details for each asset for storage
#[derive(MaxEncodedLen, Clone, Decode, Encode, TypeInfo, Debug, PartialEq)]
#[scale_info(skip_type_params(SymbolMaxChars))]
pub struct AssetDetails {
	/// The symbol of the asset
	pub symbol: Assets,
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
	/// The threshold of the asset's price which represents the minimum bound and maximum bound
	pub price_threshold: (LedgerBalance, LedgerBalance),
	/// The threshold of the asset's issuance which represents the minimum bound and maximum bound
	pub issuance_threshold: (LedgerBalance, LedgerBalance)
}

/// Holds the details for each asset for processing
#[derive(Clone, Decode, Encode, Debug, PartialEq)]
pub struct AssetData<T> {
	/// The symbol of the asset
	pub symbol: Assets,
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
	/// The threshold price of the asset which represents the minimum bound and maximum bound
	pub price_threshold: (u128, u128),
	/// The threshold issuance of the asset which represents the minimum bound and maximum bound
	pub issuance_threshold: (u128, u128)
}
