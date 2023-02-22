use frame_support::dispatch;
use sp_std::vec::Vec;
use crate::LedgerBalance;
use frame_support::{dispatch::{ DispatchResult, EncodeLike, TypeInfo }, pallet_prelude::*};

pub const DIVISOR_UNIT: LedgerBalance = 100_000;
pub const STORAGE_MULTIPLIER: LedgerBalance = 100_000_000_000_000_000_000;

/// UnitOfAccount trait definition to be used in other pallets
pub trait UnitOfAccountInterface {
	/// Registers a new currency
	fn add_currency(symbol: Vec<u8>, issuance: LedgerBalance, price: LedgerBalance) -> Result<(), dispatch::DispatchError>;
	/// Removes a currency
	fn remove_currency(symbol: Vec<u8>) -> Result<(), dispatch::DispatchError>;
}

/// Holds the currency details of each currency and the derivatives
#[derive(MaxEncodedLen, Clone, Decode, Encode, TypeInfo, Debug)]
#[scale_info(skip_type_params(MaxSymbolOfCurrency))]
pub struct CurrencyDetails<MaxSymbolOfCurrency: Get<u32>> {
	/// The currency symbol
	pub symbol: BoundedVec<u8, MaxSymbolOfCurrency>,
	/// The total issuance of the currency
	pub issuance: LedgerBalance,
	/// The price of the currency
	pub price: LedgerBalance,
	/// weighting_per_currency = inverse_issuance / sum_total_inverse_issuances
	pub weight: Option<LedgerBalance>,
	/// weight_adjusted_price = weighting_per_currency * price_in_base_currency
	pub weight_adjusted_price: Option<LedgerBalance>,
	/// uoa_per_unit_currency = price_in_base_currency / (100_000 * unit_of_account)
	pub unit_of_account: Option<LedgerBalance>
}
