use super::*;
use mock::{
	new_test_ext, 
	Balances, 
	PalletUnitOfAccount,
	RuntimeOrigin,
	System,
	Test,
};
use frame_support::{
	assert_err, 
	assert_ok,
	assert_noop,
};
use sp_runtime::DispatchError::BadOrigin;

use totem_primitives::unit_of_account::*; 

// Bad Origin Tests - MANDATORY FOR ALL EXTRINISCS
#[test]
fn x_1_should_fail_bad_origin() {
	new_test_ext().execute_with(|| {
        assert_noop!(
            PalletUnitOfAccount::whitelist_account(RuntimeOrigin::none()),
            BadOrigin,
        );
		assert_noop!(
			PalletUnitOfAccount::remove_account(RuntimeOrigin::none(), None),
			BadOrigin,
		);
		assert_noop!(
			PalletUnitOfAccount::add_new_asset(RuntimeOrigin::none(), 
			Tickers::Forex(FIAT::CNY), 
			20308000000000000, 
			140000000000000000, 
			2
		),
			BadOrigin,
		);
		assert_noop!(
			PalletUnitOfAccount::remove_asset(RuntimeOrigin::none(), 
			Tickers::Forex(FIAT::CNY), 
		),
			BadOrigin,
		);
	});
}

// Other Extrinsic Tests
#[test]
fn x_2_should_add_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		// Explicitly setting the block number is a shim
		// it solves a problem in RandomnessCollectiveFlip where 1 is subtracted
		// from the block number (which is zero for testing) causing a panic
		// Create a new block with block number gt 81
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// main testing logic
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
	});
}

#[test]
fn x_3_whitelisted_account_add_should_fail_insufficient_balance() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 100, 0));
		assert_err!(
			PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)),
			Error::<Test>::InsufficientBalance,
		);
		assert_eq!(Balances::free_balance(1), 100);
	});
}

#[test]
fn x_4_whitelisted_account_should_fail_when_max_bound_is_reached() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 2000, 0));
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 2, 2000, 0));
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 3, 2000, 0));
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(2)));
		assert_err!(
			PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(3)),
			Error::<Test>::MaxWhitelistedAccounts,
		);
		assert_eq!(Balances::free_balance(1), 1000);
		assert_eq!(Balances::free_balance(2), 1000);
		assert_eq!(Balances::free_balance(3), 2000);
	});
}

#[test]
fn x_5_whitelisted_account_should_fail_when_account_is_already_whitelisted() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 3000, 0));
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		assert_err!(
			PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)),
			Error::<Test>::AlreadyWhitelistedAccount,
		);
		assert_eq!(Balances::free_balance(1), 2000);
	});
}

#[test]
fn x_6_should_remove_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 2000, 0));
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		assert_eq!(Balances::free_balance(1), 1000);
		assert_ok!(PalletUnitOfAccount::remove_account(RuntimeOrigin::signed(1), None));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_eq!(PalletUnitOfAccount::whitelisted_accounts(1), None);
	});
}

#[test]
fn x_7_sudo_should_remove_a_whitelisted_account_successfully() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 2000, 0));
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		assert_eq!(Balances::free_balance(1), 1000);
		assert_ok!(PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), Some(1)));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_eq!(PalletUnitOfAccount::whitelisted_accounts(1), None);
	});
}

#[test]
fn x_8_remove_account_should_fail_when_account_is_not_whitelisted() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 2000, 0));
		assert_err!(
			PalletUnitOfAccount::remove_account(RuntimeOrigin::signed(1), None),
			Error::<Test>::UnknownWhitelistedAccount,
		);
		assert_eq!(Balances::free_balance(1), 2000);
	});
}

#[test]
fn x_9_remove_account_should_fail_when_account_is_not_valid() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 2000, 0));
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 2, 2000, 0));
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		assert_err!(
			PalletUnitOfAccount::remove_account(RuntimeOrigin::signed(1), Some(2)),
			Error::<Test>::InvalidAccountToUnlist,
		);
		// Balance of 1 should be reduced and not equal to the original balance
		assert_ne!(Balances::free_balance(1), 2000);
		assert_eq!(Balances::free_balance(2), 2000);
	});
}

#[test]
fn x_10_remove_account_should_fail_using_sudo_when_account_is_not_whitelisted() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 2000, 0));
		assert_err!(
			PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), Some(1)),
			Error::<Test>::UnknownWhitelistedAccount,
		);
		assert_eq!(Balances::free_balance(1), 2000);
	});
}

#[test]
fn x_11_remove_account_should_fail_using_sudo_when_account_is_not_valid() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 2000, 0));
		assert_err!(
			PalletUnitOfAccount::remove_account(RuntimeOrigin::root(), None),
			Error::<Test>::InvalidAccountToUnlist,
		);
		assert_eq!(Balances::free_balance(1), 2000);
	});
}

// #[test]
// fn x_1_x_name_stating_what_should_happen() {
// 	new_test_ext().execute_with(|| {
// 		System::set_block_number(100);
// 		assert_eq!(System::block_number(), 100);
// 		// setup state
// 		// perform tests
// 		...
// 	});
// }

#[test]
fn x_12_should_add_new_single_asset_successfully() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		// main testing logic
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let mut ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let mut issuance: u64 = 20308000000000000;
		let mut price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let mut decimals: u8 = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 49);
		assert_eq!(PalletUnitOfAccount::financial_index(), 140000000000000000);
		// now basket exists add more records
		ticker = Tickers::Forex(FIAT::USD);
		issuance = 1564692617100000;
		price = PalletUnitOfAccount::convert_float_to_int(1.0);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);
		
		ticker = Tickers::Forex(FIAT::EUR);
		issuance = 1214125230000000;
		price = PalletUnitOfAccount::convert_float_to_int(1.08);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::JPY);
		issuance = 138166400000000000;
		price = PalletUnitOfAccount::convert_float_to_int(0.01);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::GBP);
		issuance = 288957500000000;
		price = PalletUnitOfAccount::convert_float_to_int(1.23);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 4980);
		assert_eq!(PalletUnitOfAccount::financial_index(), 1163123087857170176);
	});
}

#[test]
fn x_13_add_new_asset_should_fail_when_using_account_not_whitelisted() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		// Basket does not yet exist. This is the population of the first record
		let ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let issuance: u64 = 123;
		let price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let decimals: u8 = 2;
		// perform tests
		assert_err!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		),
		Error::<Test>::NotWhitelistedAccount, 
		);
	});
}

#[test]
fn x_14_add_new_asset_should_fail_when_issuance_is_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let issuance: u64 = 0;
		let price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let decimals: u8 = 2;
		// perform tests
		assert_err!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		),
		Error::<Test>::InvalidIssuanceValue, 
		);
	});
}

#[test]
fn x_15_add_new_asset_should_fail_when_price_is_zero() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let issuance: u64 = 123;
		let price: u64 = PalletUnitOfAccount::convert_float_to_int(0.0);
		let decimals: u8 = 2;
		// perform tests
		assert_err!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		),
		Error::<Test>::InvalidPriceValue, 
		);
	});
}

#[test]
fn x_16_sudo_should_remove_single_asset_successfully() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		// main testing logic
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let mut ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let mut issuance: u64 = 20308000000000000;
		let mut price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let mut decimals: u8 = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::USD);
		issuance = 1564692617100000;
		price = PalletUnitOfAccount::convert_float_to_int(1.0);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);
		
		ticker = Tickers::Forex(FIAT::EUR);
		issuance = 1214125230000000;
		price = PalletUnitOfAccount::convert_float_to_int(1.08);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::JPY);
		issuance = 138166400000000000;
		price = PalletUnitOfAccount::convert_float_to_int(0.01);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::GBP);
		issuance = 288957500000000;
		price = PalletUnitOfAccount::convert_float_to_int(1.23);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 4980);
		assert_eq!(PalletUnitOfAccount::financial_index(), 1163123087857170176);

		assert_ok!(PalletUnitOfAccount::remove_asset(
			RuntimeOrigin::root(), 
			Tickers::Forex(FIAT::GBP),
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 1519);
		assert_eq!(PalletUnitOfAccount::financial_index(), 1010780490503248640);
	});
}

#[test]
fn x_17_whitelisted_account_should_fail_remove_single_asset() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		// main testing logic
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let mut ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let mut issuance: u64 = 20308000000000000;
		let mut price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let mut decimals: u8 = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::USD);
		issuance = 1564692617100000;
		price = PalletUnitOfAccount::convert_float_to_int(1.0);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);
		
		ticker = Tickers::Forex(FIAT::EUR);
		issuance = 1214125230000000;
		price = PalletUnitOfAccount::convert_float_to_int(1.08);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::JPY);
		issuance = 138166400000000000;
		price = PalletUnitOfAccount::convert_float_to_int(0.01);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::GBP);
		issuance = 288957500000000;
		price = PalletUnitOfAccount::convert_float_to_int(1.23);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 4980);
		assert_eq!(PalletUnitOfAccount::financial_index(), 1163123087857170176);

		assert_err!(PalletUnitOfAccount::remove_asset(
			RuntimeOrigin::signed(1), 
			Tickers::Forex(FIAT::GBP),
			//source, // source of data from enum
		), Error::<Test>::UnAuthorisedAccount);
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 4980);
		assert_eq!(PalletUnitOfAccount::financial_index(), 1163123087857170176);
	});
}

#[test]
fn x_18_sudo_should_fail_remove_asset_not_in_basket() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		// main testing logic
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let mut ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let mut issuance: u64 = 20308000000000000;
		let mut price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let mut decimals: u8 = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::USD);
		issuance = 1564692617100000;
		price = PalletUnitOfAccount::convert_float_to_int(1.0);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);		
		assert_err!(PalletUnitOfAccount::remove_asset(
			RuntimeOrigin::root(), 
			Tickers::Forex(FIAT::GBP),
			//source, // source of data from enum
		), Error::<Test>::AssetNotFound);
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);
	});
}

#[test]
fn x_19_whitelisted_account_should_fail_update_asset_price_out_of_bounds() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let mut ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let mut issuance: u64 = 20308000000000000;
		let mut price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let mut decimals: u8 = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::USD);
		issuance = 1564692617100000;
		price = PalletUnitOfAccount::convert_float_to_int(1.0);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);		

		ticker = Tickers::Forex(FIAT::CNY);
		price = PalletUnitOfAccount::convert_float_to_int(0.76);

		assert_err!(PalletUnitOfAccount::update_asset_price(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			price, // price as a u64
			//source, // source of data from enum
		), Error::<Test>::PriceOutOfBounds);
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);	

	});
}

#[test]
fn x_20_whitelisted_account_should_fail_update_issuance_out_of_bounds() {
	new_test_ext().execute_with(|| {
		System::set_block_number(100);
		assert_eq!(System::block_number(), 100);
		// setup state
		assert_ok!(Balances::set_balance(RuntimeOrigin::root(),1, 2000, 0));
		assert_eq!(Balances::free_balance(1), 2000);
		assert_ok!(PalletUnitOfAccount::whitelist_account(RuntimeOrigin::signed(1)));
		// Basket does not yet exist. This is the population of the first record
		let mut ticker: Tickers = Tickers::Forex(FIAT::CNY);
		let mut issuance: u64 = 20308000000000000;
		let mut price: u64 = PalletUnitOfAccount::convert_float_to_int(0.14);
		let mut decimals: u8 = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		ticker = Tickers::Forex(FIAT::USD);
		issuance = 1564692617100000;
		price = PalletUnitOfAccount::convert_float_to_int(1.0);
		decimals = 2;
		// perform tests
		assert_ok!(PalletUnitOfAccount::add_new_asset(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			price, // price as a u64
			decimals, // decimals as u8
			//source, // source of data from enum
		));
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);		

		ticker = Tickers::Forex(FIAT::CNY);
		issuance = 30308000000000000;

		assert_err!(PalletUnitOfAccount::update_asset_issuance(
			RuntimeOrigin::signed(1), // must be whitelisted account
			ticker, // symbol from Tickers enum
			issuance, // issuance as a u64
			//source, // source of data from enum
		), Error::<Test>::IssuanceOutOfBounds);
		assert_eq!(PalletUnitOfAccount::total_inverse_issuance(), 688);
		assert_eq!(PalletUnitOfAccount::financial_index(), 938478738111375232);		
	});
}

