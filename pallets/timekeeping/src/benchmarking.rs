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
use totem_primitives::accounting::*;

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
}

impl_benchmark_test_suite!(Timekeeping, crate::mock::new_test_ext(), crate::mock::Test,);
