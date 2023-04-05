#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Accounting;
use CurrencyBalanceOf;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use totem_primitives::accounting::*;
use frame_benchmarking::vec::Vec;


fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn run_to_block<T: Config>(new_block: <T as frame_system::Config>::BlockNumber) {
	frame_system::Pallet::<T>::set_block_number(new_block);
}

fn construct_adjustment_details<Balance: Clone>(ledger: Ledger, credit_amount: Balance, debit_amount: Balance) -> Vec<AdjustmentDetail<Balance>>{
	let mut adjustment_details = Vec::new();

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

	set_opening_balance {
		let account: T::AccountId = whitelisted_caller();

		let _ = Accounting::<T>::set_accounting_ref_date(RawOrigin::Signed(account.clone()).into(), 500400u32.into());

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let credit_amount: CurrencyBalanceOf<T> = 100u32.into();
		let debit_amount: CurrencyBalanceOf<T> = 100u32.into();
		let mut adjustment_details = construct_adjustment_details(asset_ledger, credit_amount, debit_amount);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,credit_amount,  debit_amount);

		adjustment_details.extend(adjustment_details_liabilities);

		run_to_block::<T>(1000000u32.into());

	}: _(RawOrigin::Signed(account.clone()), adjustment_details.into(), 100u32.into())
	verify {
		assert_last_event::<T>(Event::OpeningBalanceSet.into());
	}

	adjustment {
		let account: T::AccountId = whitelisted_caller();

		let _ = Accounting::<T>::set_accounting_ref_date(RawOrigin::Signed(account.clone()).into(), 500400u32.into());

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let credit_amount: CurrencyBalanceOf<T> = 100u32.into();
		let debit_amount: CurrencyBalanceOf<T> = 100u32.into();
		let mut adjustment_details = construct_adjustment_details(asset_ledger, credit_amount, debit_amount);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,credit_amount,  debit_amount);

		adjustment_details.extend(adjustment_details_liabilities);

		run_to_block::<T>(1000000u32.into());

		let _ = Accounting::<T>::set_opening_balance(RawOrigin::Signed(account.clone()).into(), adjustment_details.into(), 100u32.into());
		let posting_number = Accounting::<T>::posting_number();

		let updated_credit_amount: CurrencyBalanceOf<T> = 200u32.into();
		let updated_debit_amount: CurrencyBalanceOf<T> = 200u32.into();
		let updated_liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let updated_adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger, credit_amount,  debit_amount);

	}: _(RawOrigin::Signed(account.clone()), updated_adjustment_details_liabilities.into(), posting_number, 100u32.into())
	verify {
		assert_last_event::<T>(Event::AdjustmentsPosted.into());
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
