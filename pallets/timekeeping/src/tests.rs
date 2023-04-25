#![cfg(test)]
use crate::{mock::{new_test_ext, RuntimeOrigin, Timekeeping, Test, Teams}, TeamWorkersBanList, TimeRecord};
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use totem_primitives::timekeeping::{BannedStruct, ReasonCodeStruct, StatusOfTimeRecord, Timekeeper};

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

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

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

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));
	});
}

#[test]
fn worker_acceptance_team_should_execute_successfully_when_false() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

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

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

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

#[test]
fn submit_time_should_execute_successfully_for_time_hash() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		assert_ok!(Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status.clone(),
			reason_for_change.clone(), number_of_blocks.clone(), posting_period.clone(),
			start_block_number.clone(), end_block_number.clone(), break_counter.clone()
		));

	});
}

#[test]
fn submit_time_should_fail_when_team_is_inactive() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

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

		let res = Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [5, 0, 0, 0],
				message: Some("TeamInactive"),
			})
		);

	});
}

#[test]
fn submit_time_should_fail_when_worker_banned() {
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

		let ban_status = true;
		let banned_struct = BannedStruct(ban_status, reason_for_change.clone());

		TeamWorkersBanList::<Test>::insert((team_hash.clone(), 2), banned_struct);

		let res = Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [3, 0, 0, 0],
				message: Some("WorkerBanned"),
			})
		);

	});
}


#[test]
fn submit_time_should_fail_when_worker_not_assigned() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

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

		let res = Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status,
										   reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [2, 0, 0, 0],
				message: Some("WorkerNotAssigned"),
			})
		);

	});
}

#[test]
fn submit_time_should_fail_when_time_record_not_from_worker() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let res = Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status.clone(),
			reason_for_change.clone(), number_of_blocks.clone(), posting_period.clone(),
			start_block_number.clone(), end_block_number.clone(), break_counter.clone()
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [8, 0, 0, 0],
				message: Some("TimeRecordNotFromWorker"),
			})
		);

	});
}

#[test]
fn submit_time_should_fail_when_time_record_locked() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: true,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status.clone(),
			reason_for_change.clone(), number_of_blocks.clone(), posting_period.clone(),
			start_block_number.clone(), end_block_number.clone(), break_counter.clone()
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [9, 0, 0, 0],
				message: Some("TimeRecordLocked"),
			})
		);

	});
}


#[test]
fn submit_time_should_fail_when_time_record_not_owned() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 3,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status.clone(),
										   reason_for_change.clone(), number_of_blocks.clone(), posting_period.clone(),
										   start_block_number.clone(), end_block_number.clone(), break_counter.clone()
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [10, 0, 0, 0],
				message: Some("TimeRecordNotOwned"),
			})
		);

	});
}

#[test]
fn submit_time_should_fail_when_time_status_cannot_be_set() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, StatusOfTimeRecord::Invoiced,
										   reason_for_change.clone(), number_of_blocks.clone(), posting_period.clone(),
										   start_block_number.clone(), end_block_number.clone(), break_counter.clone()
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [16, 0, 0, 0],
				message: Some("StatusNotImplementedOr"),
			})
		);

	});
}


#[test]
fn authorize_time_should_execute_successfully() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Submitted;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		assert_ok!(Timekeeping::authorise_time(RuntimeOrigin::signed(1), 2, team_hash, time_hash, StatusOfTimeRecord::Disputed, reason_for_change
		));
	});
}

#[test]
fn authorize_time_should_fail_when_invalid_team_owner() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Submitted;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::authorise_time(RuntimeOrigin::signed(2), 2, team_hash, time_hash, StatusOfTimeRecord::Disputed, reason_for_change);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [4, 0, 0, 0],
				message: Some("InvalidTeamOrOwner"),
			})
		);
	});
}

#[test]
fn authorize_time_should_fail_when_time_record_does_not_exist() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);


		let res = Timekeeping::authorise_time(RuntimeOrigin::signed(1), 2, team_hash, time_hash, StatusOfTimeRecord::Disputed, reason_for_change);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [15, 0, 0, 0],
				message: Some("TimeRecordDoesNotExist"),
			})
		);
	});
}

#[test]
fn authorize_time_should_fail_when_time_record_locked() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Submitted;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: true,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::authorise_time(RuntimeOrigin::signed(1), 2, team_hash, time_hash, StatusOfTimeRecord::Disputed, reason_for_change);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [9, 0, 0, 0],
				message: Some("TimeRecordLocked"),
			})
		);
	});
}

#[test]
fn authorize_time_should_fail_when_time_record_not_finalized() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::authorise_time(RuntimeOrigin::signed(1), 2, team_hash, time_hash, StatusOfTimeRecord::Disputed, reason_for_change);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [13, 0, 0, 0],
				message: Some("TimeRecordNotFinalised"),
			})
		);
	});
}

#[test]
fn authorize_time_should_fail_when_status_not_implemented() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Submitted;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::authorise_time(RuntimeOrigin::signed(1), 2, team_hash, time_hash, StatusOfTimeRecord::Submitted, reason_for_change);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [17, 0, 0, 0],
				message: Some("StatusNotImplemented"),
			})
		);
	});
}

#[test]
fn authorize_time_should_fail_when_team_cannot_be_changed() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let bytes = [1u8; 32].as_slice();
		let time_hash = H256::from_slice(&bytes);

		let submit_status = StatusOfTimeRecord::Disputed;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let new_time_data = Timekeeper {
			worker: 2,
			team_hash: team_hash.clone(),
			total_blocks: number_of_blocks.clone(),
			locked_status: false,
			locked_reason: reason_for_change.clone(),
			submit_status: submit_status.clone(),
			reason_code: reason_for_change.clone(),
			posting_period: 0,
			start_block: start_block_number.clone(),
			end_block: end_block_number.clone(),
			nr_of_breaks: break_counter.clone(),
		};

		TimeRecord::<Test>::insert(time_hash.clone(), new_time_data);

		let res = Timekeeping::authorise_time(RuntimeOrigin::signed(1), 2, team_hash, time_hash, StatusOfTimeRecord::Submitted, reason_for_change);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [7, 0, 0, 0],
				message: Some("TeamCannotBeChanged"),
			})
		);
	});
}

