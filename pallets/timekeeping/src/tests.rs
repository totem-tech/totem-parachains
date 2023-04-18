#![cfg(test)]
use crate::{
	mock::{new_test_ext, RuntimeOrigin, Timekeeping, Test, Teams}
};
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        assert_ok!(Timekeeping::invoice_time(
            RuntimeOrigin::signed(1),
            H256([0; 32]),
            H256([0; 32]),
        ));
    });
}

#[test]
fn notify_team_worker_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),2,team_hash));
	});
}

#[test]
fn notify_team_worker_should_fail_when_invalid_team_owner() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_err!(
			Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),2,team_hash),
			DispatchError::Module(ModuleError {
				index: 1,
				error: [4, 0, 0, 0],
				message: Some("InvalidTeamOrOwner"),
			})
		);
	});
}
