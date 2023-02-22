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

		let whitelisted_account =
			PalletUnitOfAccount::whitelisted_accounts(account.clone()).unwrap();

		assert_eq!(whitelisted_account, ());
	});
}

#[test]
fn should_remove_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let whitelisted_account =
			PalletUnitOfAccount::whitelisted_accounts(account.clone()).unwrap();

		assert_eq!(whitelisted_account, ());

		let res = PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), account.clone());

		assert_ok!(res);
	});
}

#[test]
fn should_add_currency_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::root(), account.clone());
		assert_ok!(res);

		let whitelisted_account =
			PalletUnitOfAccount::whitelisted_accounts(account.clone()).unwrap();

		assert_eq!(whitelisted_account, ());

		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			vec![0, 0, 0, 0, 0, 0, 0],
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);

		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			vec![0, 0, 0, 0, 0, 0, 1],
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);

		let currency_basket = PalletUnitOfAccount::currency_basket();
		//dbg!(currency_basket);

		let res = PalletUnitOfAccount::add_currency(
			RuntimeOrigin::signed(account.clone()),
			vec![0, 0, 0, 0, 0, 0, 2],
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);

		let currency_basket = PalletUnitOfAccount::currency_basket();
		//dbg!(currency_basket);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(unit_of_account);

		let currency_basket = PalletUnitOfAccount::currency_basket();
		dbg!(currency_basket);
	});
}
