#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-orders

use super::*;

#[allow(unused)]
use crate::{Pallet as PrefundingPallet, PrefundingHashOwner };
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use totem_primitives::{escrow::EscrowableCurrency, accounting::*};
use frame_benchmarking::vec::Vec;
use sp_core::H256;
use sp_runtime::traits::{Hash, StaticLookup};
use totem_primitives::orders::{ApprovalStatus, OrderHeader, OrderItem, TxKeysL, TxKeysM};
use totem_primitives::prefunding::LockStatus;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	where_clause {
		where T: pallet_balances_totem::Config,
		T: pallet_escrow::Config
	}

	prefund_someone {
		let commander: T::AccountId = whitelisted_caller();
		let someone: T::AccountId = account("", 1, SEED);
		let tx_uid: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let amount: u32 = 1000000;
		let deadline: u32 = 200000;

		let commander_lookup = T::Lookup::unlookup(commander.clone());
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), commander_lookup, 1000000000u32.into(), 1000000000u32.into());

	}: _(RawOrigin::Signed(commander), someone, amount.into(), deadline.into(), tx_uid.clone())
	verify {
		assert_last_event::<T>(Event::PrefundingCompleted(tx_uid).into());
	}

	invoice_prefunded_order {
		let commander: T::AccountId = whitelisted_caller();
		let someone: T::AccountId = account("", 1, SEED);
		let tx_uid: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let amount: u32 = 1000000;
		let deadline: u32 = 200000;

		let commander_lookup = T::Lookup::unlookup(commander.clone());
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), commander_lookup, 1000000000u32.into(), 1000000000u32.into());

		PrefundingPallet::<T>::prefund_someone(RawOrigin::Signed(commander.clone()).into(), someone.clone(), amount.clone().into(), deadline.clone().into(), tx_uid.clone() );

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<T>::iter().next().map(|(key, _)| key);

	}: _(RawOrigin::Signed(someone.clone()), someone.clone(), amount.into(), latest_prefunding_hash_owner_key.unwrap(), tx_uid.clone())
	verify {
		assert_last_event::<T>(Event::InvoiceIssued(tx_uid).into());
	}

	pay_prefunded_invoice {
		let commander: T::AccountId = whitelisted_caller();
		let someone: T::AccountId = account("", 1, SEED);
		let tx_uid: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let amount: u32 = 1000000;
		let deadline: u32 = 200000;

		let commander_lookup = T::Lookup::unlookup(commander.clone());
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), commander_lookup, 1000000000u32.into(), 1000000000u32.into());

		PrefundingPallet::<T>::prefund_someone(RawOrigin::Signed(commander.clone()).into(), someone.clone(), amount.clone().into(), deadline.clone().into(), tx_uid.clone() );

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<T>::iter().next().map(|(key, _)| key);

		PrefundingPallet::<T>::invoice_prefunded_order(RawOrigin::Signed(someone.clone()).into(), someone.clone(), amount.into(), latest_prefunding_hash_owner_key.unwrap().clone(), tx_uid.clone() );

		PrefundingHashOwner::<T>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((commander.clone(), LockStatus::Locked, someone.clone(), LockStatus::Locked));
		});

		let escrow_account = pallet_escrow::Pallet::<T>::escrow_account();
		let escrow_lookup = T::Lookup::unlookup(escrow_account);
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), escrow_lookup, 1000000000u32.into(), 1000000000u32.into());

	}: _(RawOrigin::Signed(commander.clone()), latest_prefunding_hash_owner_key.unwrap(), tx_uid.clone())
	verify {
		assert_last_event::<T>(Event::InvoiceSettled(tx_uid).into());
	}

	cancel_prefunded_closed_order {
		let commander: T::AccountId = whitelisted_caller();
		let someone: T::AccountId = account("", 1, SEED);
		let tx_uid: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let amount: u32 = 1000000;
		let deadline: u32 = 200000;

		let commander_lookup = T::Lookup::unlookup(commander.clone());
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), commander_lookup, 1000000000u32.into(), 1000000000u32.into());

		PrefundingPallet::<T>::prefund_someone(RawOrigin::Signed(commander.clone()).into(), someone.clone(), amount.clone().into(), deadline.clone().into(), tx_uid.clone() );

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<T>::iter().next().map(|(key, _)| key);

		PrefundingPallet::<T>::invoice_prefunded_order(RawOrigin::Signed(someone.clone()).into(), someone.clone(), amount.into(), latest_prefunding_hash_owner_key.unwrap().clone(), tx_uid.clone() );

		PrefundingHashOwner::<T>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((commander.clone(), LockStatus::Unlocked, someone.clone(), LockStatus::Unlocked));
		});

		let escrow_account = pallet_escrow::Pallet::<T>::escrow_account();
		let escrow_lookup = T::Lookup::unlookup(escrow_account);
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), escrow_lookup, 1000000000u32.into(), 1000000000u32.into());

	}: _(RawOrigin::Signed(commander.clone()), latest_prefunding_hash_owner_key.unwrap(), tx_uid.clone())
	verify {
		assert_last_event::<T>(Event::InvoiceSettled(tx_uid).into());
	}


}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
