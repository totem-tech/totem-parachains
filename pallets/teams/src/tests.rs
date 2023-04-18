use crate::{
	TeamHashOwner,
	mock::{new_test_ext, RuntimeOrigin, Teams, Test}
};
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;

#[test]
fn should_add_new_team_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);
	});
}

#[test]
fn should_add_new_team_should_fail_when_team_already_exists() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
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
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::remove_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [7, 0, 0, 0],
				message: Some("AlreadyDeleted"),
			})
		);
	});
}

#[test]
fn should_remove_new_team_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::remove_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);
	});
}

#[test]
fn should_remove_team_should_fail_when_team_does_not_exist() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		<TeamHashOwner<Test>>::remove(&team_hash);

		let res = Teams::remove_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [8, 0, 0, 0],
				message: Some("CannotFetchTeamOwner"),
			})
		);
	});
}

#[test]
fn should_reassign_team_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::reassign_team(RuntimeOrigin::signed(1), 2, team_hash.into());
		assert_ok!(res);
	});
}

#[test]
fn should_reassign_team_should_fail_when_team_does_not_exist() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		let res = Teams::reassign_team(RuntimeOrigin::signed(1), 2, team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [6, 0, 0, 0],
				message: Some("TeamDoesNotExist"),
			})
		);
	});
}

#[test]
fn should_reassign_team_should_fail_when_cannot_fetch_team_owner() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		<TeamHashOwner<Test>>::remove(&team_hash);

		let res = Teams::reassign_team(RuntimeOrigin::signed(1), 2, team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [8, 0, 0, 0],
				message: Some("CannotFetchTeamOwner"),
			})
		);
	});
}

#[test]
fn should_reassign_team_should_fail_when_cannot_reassign_not_owned() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::reassign_team(RuntimeOrigin::signed(2), 1, team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [9, 0, 0, 0],
				message: Some("CannotReassignNotOwned"),
			})
		);
	});
}

#[test]
fn should_close_team_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::close_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);
	});
}

#[test]
fn should_close_team_should_fail_when_team_does_not_exist() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		let res = Teams::close_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [6, 0, 0, 0],
				message: Some("TeamDoesNotExist"),
			})
		);
	});
}

#[test]
fn should_close_team_should_fail_when_cannot_fetch_team_owner() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		<TeamHashOwner<Test>>::remove(&team_hash);

		let res = Teams::close_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [8, 0, 0, 0],
				message: Some("CannotFetchTeamOwner"),
			})
		);
	});
}

#[test]
fn should_reassign_team_should_fail_when_cannot_close_not_owned() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::close_team(RuntimeOrigin::signed(2), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [10, 0, 0, 0],
				message: Some("CannotCloseNotOwned"),
			})
		);
	});
}

#[test]
fn should_reopen_team_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::close_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::reopen_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);
	});
}

#[test]
fn should_reopen_team_should_fail_when_team_does_not_exist() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		let res = Teams::reopen_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [6, 0, 0, 0],
				message: Some("TeamDoesNotExist"),
			})
		);
	});
}


#[test]
fn should_reopen_team_should_fail_when_cannot_fetch_team_owner() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::close_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		<TeamHashOwner<Test>>::remove(&team_hash);

		let res = Teams::reopen_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [8, 0, 0, 0],
				message: Some("CannotFetchTeamOwner"),
			})
		);
	});
}

#[test]
fn should_reopen_team_should_fail_when_cannot_change_not_owned() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::close_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::reopen_team(RuntimeOrigin::signed(2), team_hash.into());
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [11, 0, 0, 0],
				message: Some("TeamCannotChangeNotOwned"),
			})
		);
	});
}

#[test]
fn should_set_status_team_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::set_status_team(RuntimeOrigin::signed(1), team_hash.into(), 200);
		assert_ok!(res);
	});
}

#[test]
fn should_set_status_team_should_fail_when_team_does_not_exist() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		let res = Teams::set_status_team(RuntimeOrigin::signed(1), team_hash.into(), 200);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [6, 0, 0, 0],
				message: Some("TeamDoesNotExist"),
			})
		);
	});
}

#[test]
fn should_set_status_team_should_fail_when_cannot_fetch_team_owner() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		<TeamHashOwner<Test>>::remove(&team_hash);

		let res = Teams::set_status_team(RuntimeOrigin::signed(1), team_hash.into(), 200);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [8, 0, 0, 0],
				message: Some("CannotFetchTeamOwner"),
			})
		);
	});
}

#[test]
fn should_set_status_team_should_fail_when_cannot_change_not_owned() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::set_status_team(RuntimeOrigin::signed(2), team_hash.into(), 200);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [11, 0, 0, 0],
				message: Some("TeamCannotChangeNotOwned"),
			})
		);
	});
}

#[test]
fn should_set_status_team_should_fail_when_status_is_wrong() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::set_status_team(RuntimeOrigin::signed(1), team_hash.into(), 100);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [0, 0, 0, 0],
				message: Some("StatusWrong"),
			})
		);
	});
}

#[test]
fn should_set_status_team_should_fail_when_status_cannot_apply() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);
		let res = Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());
		assert_ok!(res);

		let res = Teams::set_status_team(RuntimeOrigin::signed(1), team_hash.into(), 800);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [2, 0, 0, 0],
				message: Some("StatusCannotApply"),
			})
		);
	});
}



