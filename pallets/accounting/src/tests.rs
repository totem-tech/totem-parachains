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

		let ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let adjustment_details_assets = construct_adjustment_details(ledger,1_000_000u64, 1_000_000u64);

		let ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(ledger,1_000_000u64, 1_000_000u64);

		let mut adjustment_details = vec![];
		adjustment_details_assets.iter().map(|a| adjustment_details.push(a.clone()));
		adjustment_details_liabilities.iter().map(|a| adjustment_details.push(a.clone()));
		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_ok!(res);
	});
}

#[test]
fn set_opening_balance_fails_when_too_many_open_entries() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 2, 77);

		let adjustment_details = construct_adjustment_details_for_too_many_entries(1_000_000u64);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [12, 0, 0, 0],
				message: Some("TooManyOpeningEntries"),
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

		let ledger = Ledger::ProfitLoss(P::Income(I::Sales(Sales::SalesOfServices)));
		let adjustment_details = construct_adjustment_details(ledger, 1_000_000u64, 1_000_000u64);

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

		let ledger = Ledger::ControlAccounts(ControlAccounts::BorrowingsControl);
		let adjustment_details = construct_adjustment_details(ledger, 1_000_000u64, 1_000_000u64);

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

#[test]
fn set_opening_balance_fails_when_accounting_equation_error() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let adjustment_details = construct_adjustment_details(ledger,2_000_000u64, 1_000_000u64);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [8, 0, 0, 0],
				message: Some("AccountingEquationError"),
			})
		);
	});
}

#[test]
fn set_opening_balance_fails_when_debit_credit_mismatch() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);


		let adjustment_details = construct_adjustment_details_for_credit_debit_mismatch(2_000_000u64, 1_000_000u64);

		dbg!(adjustment_details.clone());
		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [11, 0, 0, 0],
				message: Some("DebitCreditMismatch"),
			})
		);
	});
}

#[test]
fn set_opening_balance_fails_when_balance_value_overflow() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);


		let adjustment_details = construct_adjustment_details_for_credit_debit_mismatch(u64::MAX, u64::MAX);

		dbg!(adjustment_details.clone());
		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [1, 0, 0, 0],
				message: Some("BalanceValueOverflow"),
			})
		);
	});
}
