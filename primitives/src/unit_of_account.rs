//! All the traits exposed to be used in other custom pallets
use codec::Codec;
use frame_support::dispatch;
use sp_std::vec::Vec;
use crate::LedgerBalance;

/// UnitOfAccount trait definition to be used in other pallets
pub trait UnitOfAccountInterface<AccountId: Codec> {
	/// Registers a new currency
	fn add_currency(currency: Vec<u8>, issuance: LedgerBalance) -> Result<(), dispatch::DispatchError>;
	/// Removes a currency
	fn remove_currency(currency: Vec<u8>) -> Result<(), dispatch::DispatchError>;
}
