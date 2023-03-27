
use super::*;
use crate::{mock::*, *};
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok, traits::ConstU32};
use sp_runtime::ModuleError;
use sp_runtime::DispatchError;

#[test]
fn set_accounting_ref_date_works() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let res = Accounting::set_accounting_ref_date(RuntimeOrigin::signed(account.clone()), 500400);

		assert_ok!(res);
    });
}
