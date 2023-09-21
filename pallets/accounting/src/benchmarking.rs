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

// // Structure of a benchmarking function:
// add_new_asset {
//     // Initially create the caller. This us the account that will be used to call the function.
//     // In this example the whitelisted_caller() is simply a helper function that creates an account with a balance.
//     let account: T::AccountId = whitelisted_caller();
//     // Set any storage values that will be used by default.
//     // In this example we are storing the caller in the whitelist
//     WhitelistedAccounts::<T>::set(account.clone(), Some(()));

//     // Set the values that will be used to create the asset.
//     // In this example we are pre-filling storage with some values because we want this test to fail because we are not allowed to add the same asset twice.
//     let aca_symbol = Tickers::Crypto(CoinType::Coin(COIN::ACA));
//     let aca_issuance = 203_080_000_000_000;
//     let aca_price = 1_400_000_000_00;
//     let aca_price_threshold = (1_400_000_000_00, 2_400_000_000_00);
//     let aca_issuance_threshold = (203_080_000_000_000, 503_080_000_000_000);

//     // In this example we actually call another function to add the asset to the basket with values.
//     // This is interesting because it is calling the api from within the benchmarking function in order to add an asset to the basket.
//     let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), aca_symbol, aca_issuance, aca_price, aca_price_threshold, aca_issuance_threshold);

//     // Now that we have set some real values in storage we are going to perform the real benchmark which is to add some values to storage 
//     let ada_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ADA));
//     let ada_issuance = 15_646_926_171_000;
//     let ada_price =  1_000_000_000_000;
//     let ada_price_threshold =  (1_000_000_000_000, 2_000_000_000_000);
//     let ada_issuance_threshold = (15_646_926_171_000, 20_646_926_171_000);

// }: _(RawOrigin::Signed(account), ada_symbol.clone(), ada_issuance, ada_price, ada_price_threshold, ada_issuance_threshold)
// // Here we should verify that the storage has been updated correctly
// // however this test simply checks to see if the call reached the last line of the function and issued an event.
// verify {
//     assert_last_event::<T>(Event::AssetAddedToBasket(ada_symbol).into());
// }

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{account, benchmarks_instance_pallet, whitelisted_caller};
use frame_system::RawOrigin;

use crate::Pallet as Accounting;

benchmarks_instance_pallet! {
    // set_accounting_ref_date should pass unless it has been set previously
    // this benchmark measures the happy path of setting the accounting ref date which is the most expensive operation
    set_accounting_ref_date {
        let caller: T::AccountId = whitelisted_caller();

        // The blocknumber needs to be between the minimum and maximum
        // Assume that the current blocknumber is either 0 or 1
        // let minimum_reference_date = current_block.clone() + 446_400u32.into();
        //let maximum_block_number = current_block + 5_256_000u32.into();
        let block_number = T::BlockNumber::from(446_402u32);
    }: _(RawOrigin::Signed(caller.clone()), block_number)
	verify {
        assert_eq!(AccountingRefDate::<T>::get(&caller), Some(block_number));
        assert_last_event::<T>(Event::<T>::AccountingRefDateSet {
            who: caller,
            at_blocknumber: block_number,
        }.into());
	}
    // set_opening_balance should pass unless it has been set previously
    // this benchmark measures the path where an existing entry exists in posting detail, 
    // and so has to iterate through the existing entries to find the first blocknumber.
    // This is currently limited to 166 entries to set the opneing balances. This is very expensive!
    // TODO
    set_opening_balance {
        let caller: T::AccountId = whitelisted_caller();
        let entries: Vec<AdjustmentDetail<CurrencyBalanceOf<T>>> = vec![];
        let block_number = T::BlockNumber::from(1u32);
    }: _(RawOrigin::Signed(caller), entries, block_number)
	verify {
		assert_eq!(Something::<T>::get(), Some(s));
	}


    // adjust_balance should pass unless the input somehow does not balance 
    // this benchmark measures the happy path of adjusting balances in the accounting system
    // This currently is limited to 10 adjustments of debits and credits.
    // TODO
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
