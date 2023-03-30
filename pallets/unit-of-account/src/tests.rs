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
			1_400_000_000_00, // 0.14
			(1_400_000_000_00, 2_400_000_000_00),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
		);
		assert_ok!(res);

		let unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_ne!(unit_of_account, 0);
		assert_eq!(unit_of_account, 101557108542190140499497658089472);

		let total_inverse_issuance = PalletUnitOfAccount::total_inverse_issuance();
		assert_ne!(total_inverse_issuance, 0);
		assert_eq!(total_inverse_issuance, 15119831);
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
			1_400_000_000_00, // 0.14
			(1_400_000_000_00, 2_400_000_000_00),
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
			1_400_000_000_00, // 0.14
			(1_400_000_000_00, 2_400_000_000_00),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
		);
		assert_ok!(res);

		let currency_symbol_4 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_4,
			1_381_664_000_000_000,
			1_000_000_000_00, // 0.1
			(1_000_000_000_00, 2_000_000_000_00),
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
			1_400_000_000_00, // 0.14
			(1_400_000_000_00, 2_400_000_000_00),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			1_381_664_000_000_000,
			1_000_000_000_00, // 0.1
			(1_000_000_000_00, 2_000_000_000_00),
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
			1_400_000_000_00, // 0.14
			(1_400_000_000_00, 2_400_000_000_00),
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
			0, // 0.14
			(1_400_000_000_00, 2_400_000_000_00),
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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
		);
		assert_ok!(res);

		let old_asset_symbol = PalletUnitOfAccount::asset_symbol();
		assert_eq!(old_asset_symbol.len(), 3);

		let old_unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_eq!(old_unit_of_account, 101557108542190140499497658089472);

		let old_total_inverse_issuance = PalletUnitOfAccount::total_inverse_issuance();
		assert_eq!(old_total_inverse_issuance, 15119831);

		let res =
			PalletUnitOfAccount::remove_asset(RuntimeOrigin::root(), currency_symbol_2);
		assert_ok!(res);

		// check that the length has changed
		let new_asset_symbol = PalletUnitOfAccount::asset_symbol();
		assert_eq!(new_asset_symbol.len(), 2);
		assert_ne!(old_asset_symbol, new_asset_symbol);

		// check that the unit of account has changed
		let new_unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_eq!(new_unit_of_account, 102697188572208678029509772967936);
		assert_ne!(old_unit_of_account, new_unit_of_account);

		// check that the total inverse issuance has changed
		let new_total_inverse_issuance = PalletUnitOfAccount::total_inverse_issuance();
		assert_eq!(new_total_inverse_issuance, 8728799);
		assert_ne!(old_total_inverse_issuance, new_total_inverse_issuance);

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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
		);
		assert_ok!(res);

		let old_unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_eq!(old_unit_of_account, 101557108542190140499497658089472);

		let old_total_inverse_issuance = PalletUnitOfAccount::total_inverse_issuance();
		assert_eq!(old_total_inverse_issuance, 15119831);

		// update price for currency_symbol_2
		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_2, 1_880_000_000_000);
		assert_ok!(res);


		// check that the unit of account has changed
		let new_unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_eq!(new_unit_of_account, 138754004478715014101672447180800);
		assert_ne!(old_unit_of_account, new_unit_of_account);

		// check that the total inverse issuance has changed
		let new_total_inverse_issuance = PalletUnitOfAccount::total_inverse_issuance();
		assert_eq!(new_total_inverse_issuance, 15119831);
		assert_eq!(old_total_inverse_issuance, new_total_inverse_issuance);
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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 1_880_000_000_000);
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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
		);
		assert_ok!(res);

		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 980_000_000_000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [15, 0, 0, 0],
				message: Some("InvalidMinimumThresholdPriceValue"),
			})
		);
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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
		);
		assert_ok!(res);

		let res =
			PalletUnitOfAccount::update_asset_price(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 3_080_000_000_000);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [16, 0, 0, 0],
				message: Some("InvalidMaximumThresholdPriceValue"),
			})
		);
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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
		);
		assert_ok!(res);

		let old_unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_eq!(old_unit_of_account, 101557108542190140499497658089472);

		let old_total_inverse_issuance = PalletUnitOfAccount::total_inverse_issuance();
		assert_eq!(old_total_inverse_issuance, 15119831);

		let res =
			PalletUnitOfAccount::update_asset_issuance(RuntimeOrigin::signed(account.clone()), currency_symbol_2, 19_646_926_171_000);
		assert_ok!(res);

		// check that the unit of account has changed
		let new_unit_of_account = PalletUnitOfAccount::unit_of_account();
		assert_eq!(new_unit_of_account, 101703727282439271142265324568576);
		assert_ne!(old_unit_of_account, new_unit_of_account);

		// check that the total inverse issuance has changed
		let new_total_inverse_issuance = PalletUnitOfAccount::total_inverse_issuance();
		assert_eq!(new_total_inverse_issuance, 13818654);
		assert_ne!(old_total_inverse_issuance, new_total_inverse_issuance);
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
			1_400_000_000_00, // 0.14
			(1_400_000_000_00, 2_400_000_000_00),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::AVA));
		let res =
			PalletUnitOfAccount::update_asset_issuance(RuntimeOrigin::signed(account.clone()), currency_symbol_3, 1_000_000_000_000);
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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
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
			1_400_000_000_00, // 0.14
			(140000000000, 240000000000),
			(203_080_000_000_000, 503_080_000_000_000),
		);
		assert_ok!(res);

		let currency_symbol_2 = Assets::Crypto(CoinType::Coin(COIN::ADA));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_2,
			15_646_926_171_000,
			1_000_000_000_000, // 1.00
			(1_000_000_000_000, 2_000_000_000_000),
			(15_646_926_171_000, 20_646_926_171_000),
		);
		assert_ok!(res);

		let currency_symbol_3 = Assets::Crypto(CoinType::Coin(COIN::ASTR));
		let res = PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(account.clone()),
			currency_symbol_3,
			12_141_252_300_000,
			1_080_000_000_000, // 1.08
			(1_080_000_000_000, 2_080_000_000_000),
			(12_141_252_300_000, 20_141_252_300_000),
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
	});
}
