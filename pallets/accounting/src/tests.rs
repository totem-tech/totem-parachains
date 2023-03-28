use super::*;
use crate::mock::*;
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok, traits::ConstU32};
use sp_runtime::{ModuleError, DispatchError};
use totem_primitives::accounting::*;

#[test]
fn set_accounting_ref_date_works() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);

		assert_ok!(res);
    });
}

#[test]
fn set_accounting_ref_date_fails_when_account_date_is_already_set() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [5, 0, 0, 0],
				message: Some("AccountingRefDateAlreadySet"),
			})
		);
	});
}

#[test]
fn set_accounting_ref_date_fails_when_account_ref_date_is_too_soon() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 100);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [6, 0, 0, 0],
				message: Some("AccountingRefDateTooSoon"),
			})
		);
	});
}

#[test]
fn set_accounting_ref_date_fails_when_account_ref_date_is_too_late() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 6_256_000);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [7, 0, 0, 0],
				message: Some("AccountingRefDateTooLate"),
			})
		);
	});
}

#[test]
fn set_opening_balance_works() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let adjustment_details = construct_adjustment_details_for_debit_credit(1_000_000u64);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_ok!(res);
	});
}

#[test]
fn set_opening_balance_fails_when_accounting_reference_is_not_set() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);

		let adjustment_details = construct_adjustment_details_for_control_accounts(1_000_000u64);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [4, 0, 0, 0],
				message: Some("AccountingRefDateNotSet"),
			})
		);
	});
}

#[test]
fn set_opening_balance_fails_when_ledger_is_profit_and_loss() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let adjustment_details = construct_adjustment_details_for_profit_and_loss_ledger(1_000_000u64);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [9, 0, 0, 0],
				message: Some("InvalidOpeningLedgerCtrl"),
			})
		);
	});
}

#[test]
fn set_opening_balance_fails_when_ledger_is_control_accounts() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let adjustment_details = construct_adjustment_details_for_control_accounts(1_000_000u64);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [10, 0, 0, 0],
				message: Some("InvalidOpeningLedgerPL"),
			})
		);
	});
}
