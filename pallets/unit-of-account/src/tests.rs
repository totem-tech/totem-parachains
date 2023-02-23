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

		assert_eq!(PalletUnitOfAccount::whitelisted_account_exists(account), true);
	});
}

#[test]
fn should_remove_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let res = PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), account.clone());

		assert_eq!(PalletUnitOfAccount::whitelisted_account_exists(account), false);
	});
}

#[test]
fn should_add_currency_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let currency_symbol_1 = vec![0, 0, 0, 0, 0, 0, 0];
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_1), true);

		let currency_symbol_2 = vec![0, 0, 0, 0, 0, 0, 1];
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_2), true);

		let currency_symbol_3 = vec![0, 0, 0, 0, 0, 0, 2];
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			vec![0, 0, 0, 0, 0, 0, 2],
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_3), true);

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

		let currency_symbol_1 = vec![0, 0, 0, 0, 0, 0, 0];
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_1), true);

		let currency_symbol_2 = vec![0, 0, 0, 0, 0, 0, 1];
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_2), true);

		let currency_symbol_3 = vec![0, 0, 0, 0, 0, 0, 2];
		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_3.clone()), true);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(unit_of_account);

		let res = PalletUnitOfAccount::remove_currency(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
		);
		assert_ok!(res);
		assert_eq!(PalletUnitOfAccount::symbol_exists(currency_symbol_3), false);

		let unit_of_account_after_removal = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, unit_of_account_after_removal);
		dbg!(unit_of_account_after_removal);
	});
}
