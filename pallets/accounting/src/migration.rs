use super::*;
use frame_support::{
	pallet_prelude::*,
	traits::{OnRuntimeUpgrade, PalletInfoAccess},
	weights::Weight,
};

fn migrate_v1_to_v2<T: Config<I>, I: 'static>(accounts: &[T::AccountId]) -> Weight {
	let onchain_version = Pallet::<T, I>::on_chain_storage_version();

	if onchain_version == 1 {
		// Just for reference will cleanup later

		// #[pallet::storage]
		// #[pallet::getter(fn posting_detail)]
		// pub type PostingDetail<T: Config> = StorageDoubleMap<
		// _,
		// Blake2_128Concat, (T::AccountId, Ledger),
		// Blake2_128Concat, PostingIndex,
		// Detail<T::AccountId, T::Hash, T::BlockNumber>,
		// >;

		// The following should read the entries in 
		// and then write them back to the same storage with the new Detail struct
		for ((account_id, ledger), posting_index, old_detail_field) in PostingDetail::<T>::drain() {
			// Transform the old value to the new format
			let expanded_detail = Detail {
				counterparty: old_detail_field.counterparty,
				amount: old_detail_field.amount,
				debit_credit: old_detail_field.debit_credit,
				reference_hash: old_detail_field.reference_hash,
				changed_on_blocknumber: old_detail_field.changed_on_blocknumber,
				applicable_period_blocknumber: old_detail_field.applicable_period_blocknumber,
				// Initialize new fields
				// This is the exchange rate KPX/Transaction Currency (e.g. USD)
				// exchange_rate: Default::default(), 
				// functional_currency_ticker: Default::default(), // This is by default KPX
				// transaction_ticker: Default::default(), // This is by default KPX
			};

			// Insert the new expanded detail into the storage
			PostingDetail::<T>::insert((account_id, ledger), posting_index, expanded_detail);
		}

		// Remove the old `StorageVersion` type.
		frame_support::storage::unhashed::kill(&frame_support::storage::storage_prefix(
			Pallet::<T, I>::name().as_bytes(),
			"StorageVersion".as_bytes(),
		));

		// Set storage version to `2`.
		StorageVersion::new(2).put::<Pallet<T, I>>();

		log::info!(target: "runtime::accounting", "Storage to version 2");
		T::DbWeight::get().reads_writes(2 + accounts.len() as u64, 3)
	} else {
		log::info!(target: "runtime::accounting",  "Migration did not execute. This probably should be removed");
		T::DbWeight::get().reads(1)
	}
}

// New struct to call your migration function on runtime upgrade
pub struct MigrateAccounting<T, A, I = ()>(PhantomData<(T, A, I)>);
impl<T: Config<I>, A: Get<T::AccountId>, I: 'static> OnRuntimeUpgrade
	for MigrateAccounting<T, A, I>
{
	fn on_runtime_upgrade() -> Weight {
		migrate_v1_to_v2::<T, I>(&[A::get()])
	}
}