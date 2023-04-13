#![cfg(feature = "runtime-benchmarks")]

//! Benchmarking setup for pallet-teams

use super::*;

#[allow(unused)]
use crate::Pallet as Teams;
use frame_benchmarking::{account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use totem_primitives::accounting::*;
use frame_benchmarking::vec::Vec;
use sp_core::H256;
use sp_runtime::traits::Hash;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	add_new_team {
		let account: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);
	}: _(RawOrigin::Signed(account.clone()), team_hash.clone())
	verify {
		assert_last_event::<T>(Event::TeamRegistered(team_hash, account).into());
	}

	remove_team {
		let account: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let _ = Teams::<T>::add_new_team(RawOrigin::Signed(account.clone()).into(), team_hash.clone());
	}: _(RawOrigin::Signed(account.clone()), team_hash.clone())
	verify {
		assert_last_event::<T>(Event::TeamDeleted(team_hash, account.clone(), account, 999).into());
	}

	reassign_team {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);
		let new_owner : T::AccountId = account("", 0, SEED);

		let _ = Teams::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
	}: _(RawOrigin::Signed(owner.clone()), new_owner.clone(), team_hash.clone())
	verify {
		assert_last_event::<T>(Event::TeamReassigned(team_hash, new_owner, owner).into());
	}

	close_team {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let _ = Teams::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
	}: _(RawOrigin::Signed(owner.clone()), team_hash.clone())
	verify {
		assert_last_event::<T>(Event::TeamChanged(team_hash, owner, 500).into());
	}

	reopen_team {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let _ = Teams::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
		let _ = Teams::<T>::close_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
	}: _(RawOrigin::Signed(owner.clone()), team_hash.clone())
	verify {
		assert_last_event::<T>(Event::TeamChanged(team_hash, owner, 100).into());
	}

	set_status_team {
		let owner: T::AccountId = whitelisted_caller();
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash: T::Hash = T::Hashing::hash(bytes);

		let _ = Teams::<T>::add_new_team(RawOrigin::Signed(owner.clone()).into(), team_hash.clone());
	}: _(RawOrigin::Signed(owner.clone()), team_hash.clone(), 200)
	verify {
		assert_last_event::<T>(Event::TeamChanged(team_hash, owner, 200).into());
	}
}

impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test,);
