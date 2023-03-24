use super::*;
use crate::{mock::*, *};
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok, traits::ConstU32};
use sp_runtime::ModuleError;
use totem_primitives::unit_of_account::{COIN, CoinType};

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
				message: Some("MaxWhitelistedAccountOutOfBounds"),
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

		let res =
			PalletUnitOfAccount::remove_account(RuntimeOrigin::signed(account_0.clone()), None);
		assert_ok!(res);

		assert_eq!(PalletUnitOfAccount::whitelisted_accounts(account_0), None);
	});
}

#[test]
fn sudo_should_remove_a_whitelisted_account_successfully() {
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

		let res =
			PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), Some(account_0.clone()));
		assert_ok!(res);

		assert_eq!(PalletUnitOfAccount::whitelisted_accounts(account_0), None);
	});
}

#[test]
fn remove_account_should_fail_when_account_is_not_whitelisted() {
	new_test_ext().execute_with(|| {
		let account_0 = account::<AccountId>("", 0, 0);
		let res =
			PalletUnitOfAccount::remove_account(RuntimeOrigin::signed(account_0.clone()), None);

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
fn remove_account_should_fail_using_sudo_when_account_is_not_whitelisted() {
	new_test_ext().execute_with(|| {
		let account_0 = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), Some(account_0));

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
fn remove_account_should_fail_using_sudo_when_account_is_invalid() {
	new_test_ext().execute_with(|| {
		let res = PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), None);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [11, 0, 0, 0],
				message: Some("InvalidAccountToWhitelist"),
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

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000),
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

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
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

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)
		);
		assert_ok!(res);

		let currency_symbol_4 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_4,
			1_381_664_000_000_000,
			1000000000000000000, // 0.1
			(1000000000000000000, 2000000000000000000),
			(1_381_664_000_000_000, 10_381_664_000_000_000)
		);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [6, 0, 0, 0],
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

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000)
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			1_381_664_000_000_000,
			1000000000000000000, // 0.1
			(1000000000000000000,21000000000000000000),
			(1_381_664_000_000_000, 10_381_664_000_000_000)
		);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [5, 0, 0, 0],
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

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			0,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [8, 0, 0, 0],
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

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			0,
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [9, 0, 0, 0],
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

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)

		);
		assert_ok!(res);

		let asset_symbol = PalletUnitOfAccount::asset_symbol();
		assert_eq!(asset_symbol.len(), 3);

		let res =
			PalletUnitOfAccount::remove_asset(RuntimeOrigin::root(), currency_symbol_2);
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

		let currency_symbol = Assets::Crypto(CoinType::Coin(COIN::ACA));

		let res = PalletUnitOfAccount::remove_asset(RuntimeOrigin::root(), currency_symbol);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [7, 0, 0, 0],
				message: Some("AssetNotFound"),
			})
		);
	});
}

#[test]
fn should_update_asset_price_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)

		);
		assert_ok!(res);

		// update price for currency_symbol_2
		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_2, 180000000000000000000);
		assert_ok!(res);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn update_asset_price_should_fail_when_asset_is_not_in_basket() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 180000000000000000000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [7, 0, 0, 0],
				message: Some("AssetNotFound"),
			})
		);


		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(&unit_of_account);
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn update_asset_price_should_fail_when_asset_price_is_below_price_threshold() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)
		);
		assert_ok!(res);

		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 90000000000000000000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [15, 0, 0, 0],
				message: Some("InvalidMinimumThresholdPriceValue"),
			})
		);


		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(&unit_of_account);
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn update_asset_price_should_fail_when_asset_price_is_above_price_threshold() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)
		);
		assert_ok!(res);

		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 300000000000000000000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [16, 0, 0, 0],
				message: Some("InvalidMaximumThresholdPriceValue"),
			})
		);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn should_update_asset_issuance_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)

		);
		assert_ok!(res);

		let res =
			PalletUnitOfAccount::update_asset_issuance(RuntimeOrigin::signed(account.clone()), currency_symbol_2, 18_646_926_171_000);
		assert_ok!(res);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn update_asset_issuance_should_fail_when_asset_is_not_in_basket() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res =
			PalletUnitOfAccount::update_asset_issuance(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 180000000000000000000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [7, 0, 0, 0],
				message: Some("AssetNotFound"),
			})
		);


		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		dbg!(&unit_of_account);
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn update_asset_issuance_should_fail_when_asset_issuance_is_below_issuance_threshold() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)
		);
		assert_ok!(res);

		let res =
			PalletUnitOfAccount::update_asset_issuance(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 10_141_252_300_000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [19, 0, 0, 0],
				message: Some("InvalidMinimumThresholdIssuanceValue"),
			})
		);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, 0);
	});
}

#[test]
fn update_asset_issuance_should_fail_when_asset_issuance_is_above_price_threshold() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(account.clone()));
		assert_ok!(res);

		let currency_symbol_1 = Assets::Crypto(CoinType::Coin(COIN::ACA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_1,
			203_080_000_000_000,
			14000000000000002000, // 0.14
			(14000000000000002000, 24000000000000002000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2.clone(),
			15_646_926_171_000,
			100000000000000000000, // 1.00
			(100000000000000000000, 200000000000000000000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			108000000000000000000, // 1.08
			(108000000000000000000, 208000000000000000000),
			(12_141_252_300_000, 20_141_252_300_000)
		);
		assert_ok!(res);

		let res =
			PalletUnitOfAccount::update_asset_issuance(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 30_141_252_300_000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [20, 0, 0, 0],
				message: Some("InvalidMaximumThresholdIssuanceValue"),
			})
		);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, 0);
	});
}
