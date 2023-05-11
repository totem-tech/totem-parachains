#![cfg(test)]

use crate::{mock::{new_test_ext, RuntimeOrigin, Archive, Teams, Timekeeping, Test}};
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use pallet_timekeeping::WorkerTimeRecordsHashList;
use totem_primitives::RecordType;
use totem_primitives::timekeeping::{ReasonCodeStruct, StatusOfTimeRecord};

#[test]
fn archive_record_works_when_archive_is_true() {
    new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let default_time_hash = <Timekeeping>::get_default_hash();

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		assert_ok!(Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, default_time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		));

		let worker_time_records_hash_list_values = WorkerTimeRecordsHashList::<Test>::iter().last().unwrap();
		let time_hash = worker_time_records_hash_list_values.1.last().unwrap();

		assert_ok!(Teams::reassign_team(RuntimeOrigin::signed(1), 2,  team_hash.into()));

		assert_ok!(Archive::archive_record(
            RuntimeOrigin::signed(2),
            RecordType::Timekeeping,
            *time_hash,
            true,
        ));
    });
}

#[test]
fn archive_record_works_when_archive_is_false() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let default_time_hash = <Timekeeping>::get_default_hash();

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		assert_ok!(Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, default_time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		));

		let worker_time_records_hash_list_values = WorkerTimeRecordsHashList::<Test>::iter().last().unwrap();
		let time_hash = worker_time_records_hash_list_values.1.last().unwrap();

		assert_ok!(Teams::reassign_team(RuntimeOrigin::signed(1), 2,  team_hash.into()));

		assert_ok!(Archive::archive_record(
            RuntimeOrigin::signed(2),
            RecordType::Timekeeping,
            *time_hash,
            true,
        ));

		assert_ok!(Archive::archive_record(
            RuntimeOrigin::signed(2),
            RecordType::Timekeeping,
            *time_hash,
            false,
        ));
	});
}

#[test]
fn archive_record_fails_with_unsupported_record_type() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let default_time_hash = <Timekeeping>::get_default_hash();

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		assert_ok!(Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, default_time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		));

		let worker_time_records_hash_list_values = WorkerTimeRecordsHashList::<Test>::iter().last().unwrap();
		let time_hash = worker_time_records_hash_list_values.1.last().unwrap();

		assert_ok!(Teams::reassign_team(RuntimeOrigin::signed(1), 2,  team_hash.into()));

		let res = Archive::archive_record(
			RuntimeOrigin::signed(2),
			RecordType::Orders,
			*time_hash,
			true,
		);

		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 1,
				error: [0, 0, 0, 0],
				message: Some("UnknownRecordType"),
			})
		);
	});
}
