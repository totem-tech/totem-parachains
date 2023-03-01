use super::*;
use crate::mock::*;
use frame_benchmarking::account;
use frame_support::assert_ok;

#[test]
fn should_add_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		assert_eq!(PalletUnitOfAccount::whitelisted_account_exists(account), Some(true));
	});
}

#[test]
fn should_remove_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let res = PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), account.clone());

		assert_eq!(PalletUnitOfAccount::whitelisted_account_exists(account), Some(false));
	});
}

#[test]
fn should_add_currency_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let currency_symbol_1: Vec<u8> = b"cny".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_1), Some(true));

		let currency_symbol_2: Vec<u8> = b"usd".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_2), Some(true));

		let currency_symbol_3: Vec<u8> = b"eur".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_3.clone()), Some(true));

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(unit_of_account);

		let currency_basket = PalletUnitOfAccount::currency_basket();
		//dbg!(currency_basket);
	});
}

#[test]
fn should_remove_currency_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let currency_symbol_1: Vec<u8> = b"cny".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_1), Some(true));

		let currency_symbol_2: Vec<u8> = b"usd".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_2), Some(true));

		let currency_symbol_3: Vec<u8> = b"eur".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_3.clone()), Some(true));

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(unit_of_account);

		let res = PalletUnitOfAccount::remove_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_3), Some(false));

		let unit_of_account_after_removal = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, unit_of_account_after_removal);
		dbg!(unit_of_account_after_removal);
	});
}

#[test]
fn should_update_currency_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let currency_symbol_1: Vec<u8> = b"cny".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_1), Some(true));

		let currency_symbol_2: Vec<u8> = b"usd".to_vec().into();
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_2.clone()), Some(true));
		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(unit_of_account);

		let res = PalletUnitOfAccount::update_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			Some(13_141_252_300_000),
			Some(158000000000000000000),
		);
		assert_ok!(res);

		let new_unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, new_unit_of_account);
		dbg!(new_unit_of_account);
	});
}
