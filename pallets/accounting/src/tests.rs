use crate::mock::*;
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok};
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

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut adjustment_details = construct_adjustment_details(asset_ledger,1_000_000u64, 1_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,1_000_000u64, 1_000_000u64);

		adjustment_details.extend(adjustment_details_liabilities);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details, 100);
		assert_ok!(res);

		let posting_number = Accounting::posting_number();

		let asset_posting_details = Accounting::posting_detail((account.clone(), asset_ledger), posting_number).unwrap();
		assert_eq!(asset_posting_details.amount, 1000000);

		let liabilities_posting_details = Accounting::posting_detail((account.clone(), liabilities_ledger), posting_number).unwrap();
		assert_eq!(liabilities_posting_details.amount, 1000000);

		let balance_by_ledger = Accounting::balance_by_ledger(&account.clone(), &liabilities_ledger).unwrap();
		assert_eq!(balance_by_ledger, 0);

		let global_ledger = Accounting::global_ledger(&liabilities_ledger);
		assert_eq!(global_ledger, 0);
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

#[test]
fn set_adjustment_works() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut adjustment_details = construct_adjustment_details(asset_ledger,1_000_000u64, 1_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,1_000_000u64, 1_000_000u64);

		adjustment_details.extend(adjustment_details_liabilities);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details.clone(), 100);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut updated_adjustment_details = construct_adjustment_details(asset_ledger,2_000_000u64, 2_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let updated_adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,2_000_000u64, 2_000_000u64);

		updated_adjustment_details.extend(updated_adjustment_details_liabilities);

		let posting_number = Accounting::posting_number();

		let res = Accounting::adjustment(RuntimeOrigin::signed(account.clone()), updated_adjustment_details, posting_number, 100);
		assert_ok!(res);

		let asset_posting_details = Accounting::posting_detail((account.clone(), asset_ledger), posting_number).unwrap();
		assert_eq!(asset_posting_details.amount, 2000000);

		let liabilities_posting_details = Accounting::posting_detail((account.clone(), liabilities_ledger), posting_number).unwrap();
		assert_eq!(liabilities_posting_details.amount, 2000000);

		let balance_by_ledger = Accounting::balance_by_ledger(&account.clone(), &liabilities_ledger).unwrap();
		assert_eq!(balance_by_ledger, 0);

		let global_ledger = Accounting::global_ledger(&liabilities_ledger);
		assert_eq!(global_ledger, 0);
	});
}

#[test]
fn set_adjustment_fails_when_index_is_zero() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut adjustment_details = construct_adjustment_details(asset_ledger,1_000_000u64, 1_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,1_000_000u64, 1_000_000u64);

		adjustment_details.extend(adjustment_details_liabilities);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details.clone(), 100);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut updated_adjustment_details = construct_adjustment_details(asset_ledger,2_000_000u64, 2_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let updated_adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,2_000_000u64, 2_000_000u64);

		updated_adjustment_details.extend(updated_adjustment_details_liabilities);

		let posting_number = 0;

		let res = Accounting::adjustment(RuntimeOrigin::signed(account.clone()), updated_adjustment_details, posting_number, 100);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [15, 0, 0, 0],
				message: Some("IndexNotFound"),
			})
		);
	});
}

#[test]
fn set_adjustment_fails_when_applicable_period_is_not_valid() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut adjustment_details = construct_adjustment_details(asset_ledger,1_000_000u64, 1_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,1_000_000u64, 1_000_000u64);

		adjustment_details.extend(adjustment_details_liabilities);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details.clone(), 100);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut updated_adjustment_details = construct_adjustment_details(asset_ledger,2_000_000u64, 2_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let updated_adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,2_000_000u64, 2_000_000u64);

		updated_adjustment_details.extend(updated_adjustment_details_liabilities);

		let posting_number = Accounting::posting_number();

		let res = Accounting::adjustment(RuntimeOrigin::signed(account.clone()), updated_adjustment_details, posting_number, 1500000);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [16, 0, 0, 0],
				message: Some("ApplicablePeriodNotValid"),
			})
		);
	});
}

#[test]
fn set_adjustment_fails_when_debit_credit_mismatch() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut adjustment_details = construct_adjustment_details(asset_ledger,1_000_000u64, 1_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,1_000_000u64, 1_000_000u64);

		adjustment_details.extend(adjustment_details_liabilities);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details.clone(), 100);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut updated_adjustment_details = construct_adjustment_details(asset_ledger,2_000_000u64, 2_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let updated_adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,4_000_000u64, 9_000_000u64);

		updated_adjustment_details.extend(updated_adjustment_details_liabilities);

		let posting_number = Accounting::posting_number();

		let res = Accounting::adjustment(RuntimeOrigin::signed(account.clone()), updated_adjustment_details, posting_number, 100);

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
fn set_adjustment_fails_when_balance_value_overflow() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut adjustment_details = construct_adjustment_details(asset_ledger,1_000_000u64, 1_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,1_000_000u64, 1_000_000u64);

		adjustment_details.extend(adjustment_details_liabilities);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details.clone(), 100);
		assert_ok!(res);

		let updated_adjustment_details = construct_adjustment_details_for_credit_debit_mismatch(u64::MAX, u64::MAX);

		let posting_number = Accounting::posting_number();

		let res = Accounting::adjustment(RuntimeOrigin::signed(account.clone()), updated_adjustment_details, posting_number, 100);

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

#[test]
fn set_adjustment_fails_when_illegal_adjustment() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount)));
		let mut adjustment_details = construct_adjustment_details(asset_ledger,1_000_000u64, 1_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,1_000_000u64, 1_000_000u64);

		adjustment_details.extend(adjustment_details_liabilities);

		let res = Accounting::set_opening_balance(RuntimeOrigin::signed(account.clone()), adjustment_details.clone(), 100);
		assert_ok!(res);

		let asset_ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance)));
		let mut updated_adjustment_details = construct_adjustment_details(asset_ledger,2_000_000u64, 2_000_000u64);

		let liabilities_ledger = Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities)));
		let updated_adjustment_details_liabilities = construct_adjustment_details(liabilities_ledger,2_000_000u64, 2_000_000u64);

		updated_adjustment_details.extend(updated_adjustment_details_liabilities);

		let posting_number = Accounting::posting_number();

		let res = Accounting::adjustment(RuntimeOrigin::signed(account.clone()), updated_adjustment_details, posting_number, 100);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [17, 0, 0, 0],
				message: Some("IllegalAdjustment"),
			})
		);
	});
}


