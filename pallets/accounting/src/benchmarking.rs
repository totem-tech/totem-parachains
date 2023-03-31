#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Accounting;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use totem_primitives::accounting::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn construct_adjustment_details<Balance: Clone>(ledger: Ledger, credit_amount: Balance, debit_amount: Balance) -> Vec<AdjustmentDetail<Balance>>{
	let mut adjustment_details = vec![];

	let adjustment_detail_credit = AdjustmentDetail {
		ledger,
		debit_credit: Indicator::Credit,
		amount: credit_amount
	};

	let adjustment_detail_debit = AdjustmentDetail {
		ledger,
		debit_credit: Indicator::Debit,
		amount: debit_amount
	};

	adjustment_details.push(adjustment_detail_credit);
	adjustment_details.push(adjustment_detail_debit);

	adjustment_details
}

benchmarks! {
	set_accounting_ref_date {
		let account: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(account.clone()), 500400u32.into())
	verify {
		assert!(AccountingRefDate::<T>::get(&account).is_some());
	}
}

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test,);
