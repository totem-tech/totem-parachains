#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::Pallet as UnitOfAccount;
use frame_benchmarking::{
	benchmarks_instance_pallet, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_system::RawOrigin;
use sp_std::vec;
fn assert_last_event<T: Config<I>, I: 'static>(generic_event: <T as Config<I>>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

const MAX_PARAMETER_LENGTH: u32 = 20000;

benchmarks_instance_pallet! {
	whitelist_account {
		let account: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, account.clone())
	verify {
		assert_last_event::<T, I>(Event::AccountWhitelisted(account).into());
	}

	remove_account {
		let account: T::AccountId = whitelisted_caller();

		let mut whitelisted_accounts = WhitelistedAccounts::<T, I>::get();

		whitelisted_accounts
				.try_push(account.clone())
				.unwrap();

		WhitelistedAccounts::<T, I>::set(whitelisted_accounts);

	}: _(RawOrigin::Root, account.clone())
	verify {
		assert_last_event::<T, I>(Event::AccountRemoved(account).into());
	}

	add_currency {
		let account: T::AccountId = whitelisted_caller();

		let mut whitelisted_accounts = WhitelistedAccounts::<T, I>::get();

		whitelisted_accounts
				.try_push(account.clone())
				.unwrap();

		WhitelistedAccounts::<T, I>::set(whitelisted_accounts);

		let cny_symbol = b"cny".to_vec().into();
		let cny_issuance = 203_080_000_000_000u128 as LedgerBalance;
		let cny_price = 14000000000000002000u128 as LedgerBalance;

		<UnitOfAccount<T, I> as UnitOfAccountInterface>::add_currency(cny_symbol, cny_issuance, cny_price);

		let usd_symbol =  b"usd".to_vec();
		let usd_issuance = 15_646_926_171_000u128 as LedgerBalance;
		let usd_price =  100000000000000000000u128 as LedgerBalance;

	}: _(RawOrigin::Signed(account.clone()), usd_symbol.clone(), usd_issuance, usd_price)
	verify {
		assert_last_event::<T, I>(Event::CurrencyAddedToBasket(usd_symbol).into());
	}

	remove_currency {
		let account: T::AccountId = whitelisted_caller();

		let mut whitelisted_accounts = WhitelistedAccounts::<T, I>::get();

		whitelisted_accounts
				.try_push(account.clone())
				.unwrap();

		WhitelistedAccounts::<T, I>::set(whitelisted_accounts);

		let cny_symbol:Vec<u8> = b"cny".to_vec().into();
		let cny_issuance = 203_080_000_000_000u128 as LedgerBalance;
		let cny_price = 14000000000000002000u128 as LedgerBalance;

		<UnitOfAccount<T, I> as UnitOfAccountInterface>::add_currency(cny_symbol.clone(), cny_issuance, cny_price);

		let usd_symbol =  b"usd".to_vec();
		let usd_issuance = 15_646_926_171_000u128 as LedgerBalance;
		let usd_price =  100000000000000000000u128 as LedgerBalance;

		<UnitOfAccount<T, I> as UnitOfAccountInterface>::add_currency(usd_symbol, usd_issuance, usd_price);

		let eur_symbol =  b"eur".to_vec();
		let eur_issuance = 12_141_252_300_000u128 as LedgerBalance;
		let eur_price =  108000000000000000000u128 as LedgerBalance;

		<UnitOfAccount<T, I> as UnitOfAccountInterface>::add_currency(eur_symbol, eur_issuance, eur_price);

	}: _(RawOrigin::Signed(account.clone()), cny_symbol.clone())
	verify {
		assert_last_event::<T, I>(Event::CurrencyRemovedFromTheBasket(cny_symbol).into());
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
