#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-orders

use super::*;

#[allow(unused)]
use crate::Pallet as OrderPallet;
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
		T: pallet_prefunding::Config,
		T: pallet_escrow::Config
	}
	create_order {
		let commander: T::AccountId = whitelisted_caller();
		let fulfiller: T::AccountId = account("", 1, SEED);
		let approver: T::AccountId = account("", 2, SEED);
		let product_hash: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let parent_id = T::Hashing::hash("01234567890123456789012345678902".as_bytes());
		let bonsai_token = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let tx_uid = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id: record_id.clone(),
			parent_id,
			bonsai_token,
			tx_uid: tx_uid.clone()
		};

		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: commander.clone(),
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		Orders::<T>::insert(&parent_id, order_header);


	}: _(RawOrigin::Signed(commander), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large)
	verify {
		assert_last_event::<T>(Event::OrderCreated(tx_uid, record_id).into());
	}

	delete_order {
		let commander: T::AccountId = whitelisted_caller();
		let fulfiller: T::AccountId = account("", 1, SEED);
		let approver: T::AccountId = account("", 2, SEED);
		let product_hash: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let parent_id = T::Hashing::hash("01234567890123456789012345678902".as_bytes());
		let bonsai_token = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let tx_uid = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id: record_id.clone(),
			parent_id,
			bonsai_token,
			tx_uid: tx_uid.clone()
		};

		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: commander.clone(),
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		Orders::<T>::insert(&parent_id, order_header);

		OrderPallet::<T>::create_order(RawOrigin::Signed(commander.clone()).into(), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large);

		let tx_uid = T::Hashing::hash("01234567890123456789012345678902".as_bytes());

		let tx_keys_medium = TxKeysM {
			record_id,
			bonsai_token,
			tx_uid
		};

	}: _(RawOrigin::Signed(commander), tx_keys_medium)
	verify {
		assert_eq!(Orders::<T>::get(&record_id).is_some(),  false);
	}

	create_spfso {
		let commander: T::AccountId = whitelisted_caller();
		let fulfiller: T::AccountId = account("", 1, SEED);
		let approver: T::AccountId = account("", 2, SEED);

		let product_hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let tx_uid = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		let commander_lookup = T::Lookup::unlookup(commander.clone());

		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), commander_lookup, 1000000000u32.into(), 1000000000u32.into());

	}: _(RawOrigin::Signed(commander.clone()), commander.clone(), fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid)
	verify {
	}

	change_spfso {
		let commander: T::AccountId = whitelisted_caller();
		let fulfiller: T::AccountId = account("", 1, SEED);
		let approver: T::AccountId = account("", 2, SEED);

		let product_hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let bonsai_token = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let tx_uid = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		let commander_lookup = T::Lookup::unlookup(commander.clone());
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), commander_lookup, 1000000000u32.into(), 1000000000u32.into());

		OrderPallet::<T>::create_spfso(RawOrigin::Signed(commander.clone()).into(), approver.clone(), fulfiller.clone(), buy_or_sell.clone(), total_amount.clone(), market_order.clone(), order_type.clone(), deadline.clone(), due_date.clone(), order_item.clone(), bonsai_token.clone(), tx_uid.clone() );

		let order_hashes = Orders::<T>::iter().last().unwrap();
		let order_hash = order_hashes.0;

		let tx_uid = T::Hashing::hash("01234567890123456789012345678902".as_bytes());

	}: _(RawOrigin::Signed(approver.clone()), approver.clone(), fulfiller, total_amount, deadline, due_date, order_item, order_hash, bonsai_token, tx_uid.clone())
	verify {
		assert_last_event::<T>(Event::OrderUpdated(tx_uid).into());
	}

	change_approval {
		let commander: T::AccountId = whitelisted_caller();
		let fulfiller: T::AccountId = account("", 1, SEED);
		let approver: T::AccountId = account("", 2, SEED);

		let product_hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let bonsai_token = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let tx_uid = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		let order_hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: commander.clone(),
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};
		Orders::<T>::insert(&order_hash, order_header);

	}: _(RawOrigin::Signed(approver.clone()), order_hash, ApprovalStatus::Accepted, bonsai_token, tx_uid)
	verify {
	}

	handle_spfso {
		let commander: T::AccountId = whitelisted_caller();
		let fulfiller: T::AccountId = account("", 1, SEED);
		let approver: T::AccountId = account("", 2, SEED);

		let product_hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let bonsai_token = T::Hashing::hash("01234567890123456789012345678901".as_bytes());
		let tx_uid = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 5;
		let deadline = 20011520;
		let due_date = 0;

		let commander_lookup = T::Lookup::unlookup(commander.clone());
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), commander_lookup, 1000000000u32.into(), 1000000000u32.into());

		OrderPallet::<T>::create_spfso(RawOrigin::Signed(commander.clone()).into(), commander.clone(), fulfiller.clone(), buy_or_sell.clone(), total_amount.clone(), market_order.clone(), order_type.clone(), deadline.clone(), due_date.clone(), order_item.clone(), bonsai_token.clone(), tx_uid.clone() );

		let order_hashes = Orders::<T>::iter().last().unwrap();
		let order_hash = order_hashes.0;

		let mut stored_order_header = order_hashes.1;

		stored_order_header.order_status = 5;
		Orders::<T>::mutate(&order_hash, |order_header| {
			*order_header = Some(stored_order_header.clone());
		});

		let prefunding_hash_owner_values = pallet_prefunding::PrefundingHashOwner::<T>::iter().last().unwrap();
		let prefunding_hash = prefunding_hash_owner_values.0;

		let mut stored_prefunding_tuple = prefunding_hash_owner_values.1;
		stored_prefunding_tuple = (stored_prefunding_tuple.0, LockStatus::Locked,  stored_prefunding_tuple.2, LockStatus::Locked);
		pallet_prefunding::PrefundingHashOwner::<T>::mutate(&prefunding_hash, |value| {
			*value = Some(stored_prefunding_tuple);
		});

		let reference_status_values = pallet_prefunding::ReferenceStatus::<T>::iter().last().unwrap();
		let reference_hash = reference_status_values.0;

		pallet_prefunding::ReferenceStatus::<T>::mutate(&reference_hash, |value| {
			*value = Some(400);
		});

		let escrow_account = pallet_escrow::Pallet::<T>::escrow_account();
		let escrow_lookup = T::Lookup::unlookup(escrow_account);
		pallet_balances_totem::Pallet::<T>::set_balance(RawOrigin::Root.into(), escrow_lookup, 1000000000u32.into(), 1000000000u32.into());

		let tx_uid = T::Hashing::hash("01234567890123456789012345678902".as_bytes());

	}: _(RawOrigin::Signed(commander.clone()), order_hash, 6, tx_uid)
	verify {
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
