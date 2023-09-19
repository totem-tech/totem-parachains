// #![cfg(feature = "runtime-benchmarks")]

// //! Benchmarking setup for pallet-template

// use super::*;

// #[allow(unused)]
// use crate::Pallet as Template;
// use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
// use frame_system::RawOrigin;

// benchmarks! {
// 	do_something {
// 		let s in 0 .. 100;
// 		let caller: T::AccountId = whitelisted_caller();
// 	}: _(RawOrigin::Signed(caller), s)
// 	verify {
// 		assert_eq!(Something::<T>::get(), Some(s));
// 	}
// }

impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test,);

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    set_accounting_ref_date {
        let caller: T::AccountId = whitelisted_caller();
        let block_number = T::BlockNumber::from(1u32);
    }: _(RawOrigin::Signed(caller), block_number)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}

    set_opening_balance {
        let caller: T::AccountId = whitelisted_caller();
        let entries: Vec<AdjustmentDetail<CurrencyBalanceOf<T>>> = vec![];
        let block_number = T::BlockNumber::from(1u32);
    }: _(RawOrigin::Signed(caller), entries, block_number)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}
    adjustment {
        let caller: T::AccountId = whitelisted_caller();
        let adjustments: Vec<AdjustmentDetail<CurrencyBalanceOf<T>>> = vec![];
        let index: PostingIndex = 1u128;
        let applicable_period = T::BlockNumber::from(1u32);
    }: _(RawOrigin::Signed(caller), adjustments, index, applicable_period)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}
}
