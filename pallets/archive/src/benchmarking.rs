#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-template

#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-orders

use super::*;

#[allow(unused)]
use crate::Pallet as OrderPallet;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_benchmarking::vec::Vec;
use sp_core::H256;
use sp_runtime::traits::Hash;
use totem_primitives::RecordType;
use totem_primitives::timekeeping::{ReasonCodeStruct, StatusOfTimeRecord};

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	where_clause {
		where T: pallet_timekeeping::Config,
		T: pallet_teams::Config
	}
	archive_record {
		let owner: T::AccountId = whitelisted_caller();
		let worker: T::AccountId = account("", 1, SEED);

		let team_hash: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		pallet_teams::Pallet::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
		pallet_timekeeping::Pallet::<T>::notify_team_worker(RawOrigin::Signed(owner.clone()).into(), worker.clone(), team_hash.clone());
		pallet_timekeeping::Pallet::<T>::worker_acceptance_team(RawOrigin::Signed(worker.clone()).into(), team_hash.clone(), true);

		let default_time_hash = pallet_timekeeping::Pallet::<T>::get_default_hash();

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		pallet_timekeeping::Pallet::<T>::submit_time(RawOrigin::Signed(worker.clone()).into(), team_hash.clone(), default_time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter);

		let worker_time_records_hash_list_values = pallet_timekeeping::WorkerTimeRecordsHashList::<T>::iter().last().unwrap();
		let time_hash = worker_time_records_hash_list_values.1.last().unwrap();

		pallet_teams::Pallet::<T>::reassign_team(RawOrigin::Signed(owner.clone()).into(), worker.clone(), team_hash.clone());

	}: _(RawOrigin::Signed(worker), RecordType::Timekeeping, *time_hash, true)
	verify {
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
