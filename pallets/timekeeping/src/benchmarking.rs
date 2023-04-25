#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Timekeeping;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller, account};
use frame_system::RawOrigin;
use frame_benchmarking::vec::Vec;
use sp_core::H256;
use sp_runtime::traits::Hash;
use totem_primitives::timekeeping::*;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	where_clause {
		where T: pallet_teams::Config
	}

	notify_team_worker {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let worker : T::AccountId = account("", 0, SEED);

		let _ = pallet_teams::Pallet::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
	}: _(RawOrigin::Signed(owner), worker.clone(), team_hash)
	verify {
		assert_last_event::<T>(Event::NotifyTeamWorker(worker, team_hash).into());
	}

	worker_acceptance_team {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let worker : T::AccountId = account("", 0, SEED);

		let _ = pallet_teams::Pallet::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
		let _ = Timekeeping::<T>::notify_team_worker(RawOrigin::Signed(owner.clone()).into(), worker.clone(), team_hash);

	}: _(RawOrigin::Signed(worker.clone()), team_hash.clone(), true)
	verify {
		let team_invites_list = TeamInvitesList::<T>::get(&team_hash);
		assert_eq!(team_invites_list.unwrap().contains(&worker), false);
	}

	submit_time {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let worker : T::AccountId = account("", 0, SEED);

		let _ = pallet_teams::Pallet::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
		let _ = Timekeeping::<T>::notify_team_worker(RawOrigin::Signed(owner.clone()).into(), worker.clone(), team_hash);
		let _ = Timekeeping::<T>::worker_acceptance_team(RawOrigin::Signed(worker.clone()).into(), team_hash.clone(), true);

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let time_bytes = [1u8; 32].as_slice();
		let time_hash = T::Hashing::hash(time_bytes);

		let new_time_data = Timekeeper {
			worker: worker.clone(),
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

		TimeRecord::<T>::insert(time_hash.clone(), new_time_data);

	}: _(RawOrigin::Signed(worker), team_hash, time_hash.into(), submit_status,
		reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter)
	verify {
	}

	authorise_time {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let worker : T::AccountId = account("", 0, SEED);

		let _ = pallet_teams::Pallet::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
		let _ = Timekeeping::<T>::notify_team_worker(RawOrigin::Signed(owner.clone()).into(), worker.clone(), team_hash);
		let _ = Timekeeping::<T>::worker_acceptance_team(RawOrigin::Signed(worker.clone()).into(), team_hash.clone(), true);

		let submit_status = StatusOfTimeRecord::Submitted;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		let time_bytes = [1u8; 32].as_slice();
		let time_hash = T::Hashing::hash(time_bytes);

		let new_time_data = Timekeeper {
			worker: worker.clone(),
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

		TimeRecord::<T>::insert(time_hash.clone(), new_time_data);

	}: _(RawOrigin::Signed(owner), worker, team_hash, time_hash.into(), StatusOfTimeRecord::Disputed,reason_for_change)
	verify {
	}
}

impl_benchmark_test_suite!(Timekeeping, crate::mock::new_test_ext(), crate::mock::Test,);
