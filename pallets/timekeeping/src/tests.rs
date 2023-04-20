#![cfg(test)]

use codec::Encode;
use crate::{mock::{new_test_ext, RuntimeOrigin, Timekeeping, Test, Teams}, pallet};
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use totem_primitives::timekeeping::{ReasonCodeStruct, StatusOfTimeRecord};

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

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));
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

#[test]
fn worker_acceptance_team_should_execute_successfully_when_true() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());

		Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash);

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));
	});
}

#[test]
fn worker_acceptance_team_should_execute_successfully_when_false() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());

		Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash);

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, false));
	});
}

#[test]
fn worker_acceptance_team_should_fail_when_team_is_inactive() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_err!(
			Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, false),
			DispatchError::Module(ModuleError {
				index: 1,
				error: [5, 0, 0, 0],
				message: Some("TeamInactive"),
			})
		);
	});
}

#[test]
fn worker_acceptance_team_should_fail_when_worker_is_not_assigned() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into());

		Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash);

		assert_err!(
			Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(3), team_hash, false),
			DispatchError::Module(ModuleError {
				index: 1,
				error: [2, 0, 0, 0],
				message: Some("WorkerNotAssigned"),
			})
		);
	});
}

#[test]
fn worker_acceptance_team_should_fail_when_worker_already_accepted_team() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		assert_err!(
			Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true),
			DispatchError::Module(ModuleError {
				index: 1,
				error: [0, 0, 0, 0],
				message: Some("WorkerAlreadyAcceptedTeam"),
			})
		);
	});
}

#[test]
fn submit_time_should_execute_successfully_for_default_time_hash() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let time_hash = <Timekeeping>::get_default_hash();

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		assert_ok!(Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		));

	});
}
