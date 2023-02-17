use frame_support::dispatch;
use sp_std::vec::Vec;
use crate::LedgerBalance;
use frame_support::{dispatch::{ DispatchResult, EncodeLike, TypeInfo }, pallet_prelude::*};

/// UnitOfAccount trait definition to be used in other pallets
pub trait UnitOfAccountInterface {
	/// Registers a new currency
	fn add_currency(currency: Vec<u8>, issuance: LedgerBalance,  price: LedgerBalance) -> Result<(), dispatch::DispatchError>;
	/// Removes a currency
	fn remove_currency(currency: Vec<u8>) -> Result<(), dispatch::DispatchError>;
}

#[derive(MaxEncodedLen, Clone, Decode, Encode, TypeInfo)]
pub struct UnitOfAccountCurrency {
	pub issuance: LedgerBalance,
	pub price: LedgerBalance,
	pub weight: Option<LedgerBalance>,
}
