#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::Pallet as UnitOfAccount;
use frame_benchmarking::{
	benchmarks, whitelisted_caller,
};
use frame_system::RawOrigin;
use totem_primitives::unit_of_account::{COIN, CoinType};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	whitelist_account {
		let account: T::AccountId = whitelisted_caller();

		let balance_to_use = T::Currency::minimum_balance() * 3_000_000_000u32.into() * 3_000_000_000u32.into();
		let _ = T::Currency::make_free_balance_be(&account, balance_to_use);

	}: _(RawOrigin::Signed(account.clone()))
	verify {
		assert_last_event::<T>(Event::AccountWhitelisted(account).into());
	}


	remove_account {
		let account: T::AccountId = whitelisted_caller();

		let balance_to_use = T::Currency::minimum_balance() * 3_000_000_000u32.into() * 3_000_000_000u32.into();
		let deposit_acocunt = UnitOfAccount::<T>::get_deposit_account();
		let _ = T::Currency::make_free_balance_be(&deposit_acocunt, balance_to_use);

		WhitelistedAccounts::<T>::set(account.clone(), Some(()));

	}: _(RawOrigin::Root, Some(account.clone()))
	verify {
		assert_last_event::<T>(Event::AccountRemoved(account).into());
	}

	add_new_asset {
		let account: T::AccountId = whitelisted_caller();

		WhitelistedAccounts::<T>::set(account.clone(), Some(()));

		let aca_symbol = Tickers::Crypto(CoinType::Coin(COIN::ACA));
		let aca_issuance = 203_080_000_000_000;
		let aca_price = 1_400_000_000_00;
		let aca_price_threshold = (1_400_000_000_00, 2_400_000_000_00);
		let aca_issuance_threshold = (203_080_000_000_000, 503_080_000_000_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), aca_symbol, aca_issuance, aca_price, aca_price_threshold, aca_issuance_threshold);

		let ada_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ADA));
		let ada_issuance = 15_646_926_171_000;
		let ada_price =  1_000_000_000_000;
		let ada_price_threshold =  (1_000_000_000_000, 2_000_000_000_000);
		let ada_issuance_threshold = (15_646_926_171_000, 20_646_926_171_000);

	}: _(RawOrigin::Signed(account), ada_symbol.clone(), ada_issuance, ada_price, ada_price_threshold, ada_issuance_threshold)
	verify {
		assert_last_event::<T>(Event::AssetAddedToBasket(ada_symbol).into());
	}


	remove_asset {
		let account: T::AccountId = whitelisted_caller();

		WhitelistedAccounts::<T>::set(account.clone(), Some(()));

		let aca_symbol = Tickers::Crypto(CoinType::Coin(COIN::ACA));
		let aca_issuance = 203_080_000_000_000;
		let aca_price = 1_400_000_000_00;
		let aca_price_threshold = (1_400_000_000_00, 2_400_000_000_00);
		let aca_issuance_threshold = (203_080_000_000_000, 503_080_000_000_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), aca_symbol, aca_issuance, aca_price, aca_price_threshold, aca_issuance_threshold);

		let ada_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ADA));
		let ada_issuance = 15_646_926_171_000;
		let ada_price =  1_000_000_000_000;
		let ada_price_threshold =  (1_000_000_000_000, 2_000_000_000_000);
		let ada_issuance_threshold = (15_646_926_171_000, 20_646_926_171_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), ada_symbol, ada_issuance, ada_price, ada_price_threshold, ada_issuance_threshold);

		let a_str_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ASTR));
		let a_str_issuance = 12_141_252_300_000;
		let a_str_price =  1_080_000_000_000;
		let a_str_price_threshold = (1_080_000_000_000, 2_080_000_000_000);
		let a_str_issuance_threshold = (12_141_252_300_000, 20_141_252_300_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), a_str_symbol, a_str_issuance, a_str_price, a_str_price_threshold, a_str_issuance_threshold);

	}: _(RawOrigin::Root, a_str_symbol.clone())
	verify {
		assert_last_event::<T>(Event::AssetRemoved(a_str_symbol).into());
	}

	update_asset_price {
		let account: T::AccountId = whitelisted_caller();

		WhitelistedAccounts::<T>::set(account.clone(), Some(()));

		let aca_symbol = Tickers::Crypto(CoinType::Coin(COIN::ACA));
		let aca_issuance = 203_080_000_000_000;
		let aca_price = 1_400_000_000_00;
		let aca_price_threshold = (1_400_000_000_00, 2_400_000_000_00);
		let aca_issuance_threshold = (203_080_000_000_000, 503_080_000_000_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), aca_symbol, aca_issuance, aca_price, aca_price_threshold,  aca_issuance_threshold);

		let ada_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ADA));
		let ada_issuance = 15_646_926_171_000;
		let ada_price =  1_000_000_000_000;
		let ada_price_threshold =  (1_000_000_000_000, 2_000_000_000_000);
		let ada_issuance_threshold = (15_646_926_171_000, 20_646_926_171_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), ada_symbol, ada_issuance, ada_price, ada_price_threshold, ada_issuance_threshold);

		let a_str_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ASTR));
		let a_str_issuance = 12_141_252_300_000;
		let a_str_price =  1_080_000_000_000;
		let a_str_price_threshold = (1_080_000_000_000, 2_080_000_000_000);
		let a_str_issuance_threshold = (12_141_252_300_000, 20_141_252_300_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), a_str_symbol, a_str_issuance, a_str_price, a_str_price_threshold, a_str_issuance_threshold);

		let new_ada_price = 1_800_000_000_000;
	}: _(RawOrigin::Signed(account.clone()), ada_symbol.clone(), new_ada_price)
	verify {
		assert_last_event::<T>(Event::AssetPriceUpdated(ada_symbol).into());
	}

	update_asset_issuance {
		let account: T::AccountId = whitelisted_caller();

		WhitelistedAccounts::<T>::set(account.clone(), Some(()));

		let aca_symbol = Tickers::Crypto(CoinType::Coin(COIN::ACA));
		let aca_issuance = 203_080_000_000_000;
		let aca_price = 1_400_000_000_00;
		let aca_price_threshold = (1_400_000_000_00, 2_400_000_000_00);
		let aca_issuance_threshold = (203_080_000_000_000, 503_080_000_000_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), aca_symbol, aca_issuance, aca_price, aca_price_threshold,  aca_issuance_threshold);

		let ada_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ADA));
		let ada_issuance = 15_646_926_171_000;
		let ada_price =  1_000_000_000_000;
		let ada_price_threshold =  (1_000_000_000_000, 2_000_000_000_000);
		let ada_issuance_threshold = (15_646_926_171_000, 20_646_926_171_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), ada_symbol, ada_issuance, ada_price, ada_price_threshold, ada_issuance_threshold);

		let a_str_symbol =  Tickers::Crypto(CoinType::Coin(COIN::ASTR));
		let a_str_issuance = 12_141_252_300_000;
		let a_str_price =  1_080_000_000_000;
		let a_str_price_threshold = (1_080_000_000_000, 2_080_000_000_000);
		let a_str_issuance_threshold = (12_141_252_300_000, 20_141_252_300_000);

		let _ = UnitOfAccount::<T>::add_new_asset(RawOrigin::Signed(account.clone()).into(), a_str_symbol, a_str_issuance, a_str_price, a_str_price_threshold, a_str_issuance_threshold);

		let new_ada_issuance = 17_646_926_171_000;
	}: _(RawOrigin::Signed(account.clone()), ada_symbol.clone(), new_ada_issuance)
	verify {
		assert_last_event::<T>(Event::AssetIssuanceUpdated(ada_symbol).into());
	}
}
