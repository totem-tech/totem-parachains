use super::*;
use crate::{mock::*, *};
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok};
use frame_support::traits::ConstU32;
use sp_runtime::ModuleError;

const balance_to_use: u64 = 1_000_000_000_000u64;

#[test]
fn should_add_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);

		Balances::set_balance(
			RuntimeOrigin::root(),
			account.clone(),
			balance_to_use,
			balance_to_use,
		);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);
	});
}

#[test]
fn whitelisted_account_should_fail_when_max_bound_is_reached() {
	new_test_ext().execute_with(|| {
		let account_0 = account::<AccountId>("", 0, 0);
		Balances::set_balance(
			RuntimeOrigin::root(),
			account_0.clone(),
			balance_to_use,
			balance_to_use,
		);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account_0.clone()));
		assert_ok!(res);

		let account_1 = account::<AccountId>("", 1, 0);
		Balances::set_balance(
			RuntimeOrigin::root(),
			account_1.clone(),
			balance_to_use,
			balance_to_use,
		);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account_1.clone()));
		assert_ok!(res);

		let account_2 = account::<AccountId>("", 2, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account_2.clone()));
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [0, 0, 0, 0],
				message:  Some("MaxWhitelistedAccountOutOfBounds"),
			})
		);
	});
}


#[test]
fn whitelisted_account_should_fail_when_account_is_already_whitelisted() {
	new_test_ext().execute_with(|| {
		let account_0 = account::<AccountId>("", 0, 0);
		Balances::set_balance(
			RuntimeOrigin::root(),
			account_0.clone(),
			balance_to_use,
			balance_to_use,
		);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account_0.clone()));
		assert_ok!(res);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account_0.clone()));

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [1, 0, 0, 0],
				message: Some("AlreadyWhitelistedAccount"),
			})
		);
	});
}


#[test]
fn should_remove_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		let account_0 = account::<AccountId>("", 0, 0);
		Balances::set_balance(
			RuntimeOrigin::root(),
			account_0.clone(),
			balance_to_use,
			balance_to_use,
		);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account_0.clone()));
		assert_ok!(res);

		let res = PalletUnitOfAccount::remove_account(RuntimeOrigin::signed(account_0.clone()), None);
		assert_ok!(res);

		assert_eq!(PalletUnitOfAccount::whitelisted_accounts(account_0), None);
	});
}


#[test]
fn remove_account_should_fail_when_account_is_not_whitelisted() {
	new_test_ext().execute_with(|| {
		let account_0 = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::remove_account(RuntimeOrigin::signed(account_0.clone()), None);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [2, 0, 0, 0],
				message: Some("UnknownWhitelistedAccount"),
			})
		);
	});
}


#[test]
fn should_add_new_asset_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);

		let currency_symbol_2: BoundedVec<u8, ConstU32<7>> = b"usd".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);

		let currency_symbol_3: BoundedVec<u8, ConstU32<7>> = b"eur".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn add_currency_should_fail_when_account_is_not_whitelisted() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);

		let currency_symbol_1: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [3, 0, 0, 0],
				message: Some("NotWhitelistedAccount"),
			})
		);
	});
}

#[test]
fn add_currency_should_fail_when_asset_basket_is_out_of_bound() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);

		let currency_symbol_2: BoundedVec<u8, ConstU32<7>> = b"usd".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);

		let currency_symbol_3: BoundedVec<u8, ConstU32<7>> = b"eur".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);

		let currency_symbol_4: BoundedVec<u8, ConstU32<7>> = b"jpy".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_4.clone(),
			1_381_664_000_000_000,
			1000000000000000000, // 0.1
		);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [7, 0, 0, 0],
				message: Some("AssetCannotBeAddedToBasket"),
			})
		);
	});
}


#[test]
fn add_asset_should_fail_when_asset_symbol_already_exists() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);

		let currency_symbol_2: BoundedVec<u8, ConstU32<7>> = b"usd".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);

		let currency_symbol_3: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
			1_381_664_000_000_000,
			1000000000000000000, // 0.1
		);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [6, 0, 0, 0],
				message: Some("SymbolAlreadyExists"),
			})
		);
	});
}


#[test]
fn add_asset_should_fail_with_invalid_issuance_value() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			0,
			14000000000000002000, // 0.14
		);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [9, 0, 0, 0],
				message: Some("InvalidIssuanceValue"),
			})
		);
	});
}


#[test]
fn add_asset_should_fail_with_invalid_price_value() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			0,
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [10, 0, 0, 0],
				message: Some("InvalidPriceValue"),
			})
		);
	});
}


#[test]
fn should_remove_asset_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1.clone(),
			203_080_000_000_000,
			14000000000000002000, // 0.14
		);
		assert_ok!(res);

		let currency_symbol_2: BoundedVec<u8, ConstU32<7>> = b"usd".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
		);
		assert_ok!(res);

		let currency_symbol_3: BoundedVec<u8, ConstU32<7>> = b"eur".to_vec().try_into().unwrap();
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3.clone(),
			12_141_252_300_000,
			108000000000000000000, // 1.08
		);
		assert_ok!(res);

		let asset_symbol = PalletUnitOfAccount::asset_symbol();
		assert_eq!(asset_symbol.len(), 3);

		let res = PalletUnitOfAccount::remove_asset(
			RuntimeOrigin::root(),
			currency_symbol_2.clone(),
		);
		assert_ok!(res);

		let asset_symbol = PalletUnitOfAccount::asset_symbol();
		assert_eq!(asset_symbol.len(), 2);

	});
}


#[test]
fn should_remove_asset_should_fail_when_asset_is_not_in_basket() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol: BoundedVec<u8, ConstU32<7>> = b"cny".to_vec().try_into().unwrap();

		let res = PalletUnitOfAccount::remove_asset(
			RuntimeOrigin::root(),
			currency_symbol.clone(),
		);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [8, 0, 0, 0],
				message: Some("AssetNotFound"),
			})
		);
	});
}

/*
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
*/
