#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as BonsaiPallet;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_benchmarking::vec::Vec;
use sp_core::H256;
use sp_runtime::traits::Hash;
use totem_primitives::RecordType;
use totem_primitives::timekeeping::{ReasonCodeStruct, StatusOfTimeRecord};

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	where_clause {
		where T: pallet_teams::Config
	}
	update_record {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();

		let bonsai_token: T::Hash = T::Hashing::hash("01234567890123456789012345678901".as_bytes());

		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let _ = pallet_teams::Pallet::<T>::add_new_team(RawOrigin::Signed(caller.clone()).into(), team_hash.clone());
	}: _(RawOrigin::Signed(caller), RecordType::Teams, team_hash,  bonsai_token)
	verify {
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
