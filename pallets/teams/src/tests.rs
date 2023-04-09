use super::*;
use crate::mock::{AccountId, new_test_ext, RuntimeOrigin, Teams};
use sp_runtime::DispatchError;
use frame_benchmarking::account;
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use sp_runtime::ModuleError;
#[test]
fn should_add_new_team_successfully() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(account), team_hash.into());
		assert_ok!(res);
	});
}

#[test]
fn should_add_new_team_should_fail_when_team_already_exists() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(account.clone()), team_hash.into());
		assert_ok!(res);

		let res = Teams::add_new_team(RuntimeOrigin::signed(account), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [5, 0, 0, 0],
				message: Some("TeamAlreadyExists"),
			})
		);
	});
}

#[test]
fn should_add_new_team_should_fail_when_team_already_deleted() {
	new_test_ext().execute_with(|| {
		let account = account::<AccountId>("", 0, 0);
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(account.clone()), team_hash.into());
		assert_ok!(res);

		let res = Teams::remove_team(RuntimeOrigin::signed(account.clone()), team_hash.into());
		assert_ok!(res);

		let res = Teams::add_new_team(RuntimeOrigin::signed(account), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [5, 0, 0, 0],
				message: Some("TeamAlreadyExists"),
			})
		);
	});
}
