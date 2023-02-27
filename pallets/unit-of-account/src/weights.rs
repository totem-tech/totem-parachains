
//! Autogenerated weights for `pallet_unit_of_account`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ubuntu-s-8vcpu-16gb-fra1-01`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/totem-parachain-collator
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_unit_of_account
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --output=./weights/pallet_unit_of_account_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_unit_of_account.
pub trait WeightInfo {
	fn whitelist_account(_c: u32, ) -> Weight;
	fn remove_account(c: u32, ) -> Weight;
	fn add_currency(c: u32, ) -> Weight;
	fn remove_currency(c: u32, ) -> Weight;

}

/// Weight functions for `pallet_unit_of_account`.
pub struct TotemWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for TotemWeight<T> {
	// Storage: UnitOfAccount WhitelistedAccounts (r:1 w:1)
	/// The range of component `c` is `[0, 20000]`.
	fn whitelist_account(_c: u32, ) -> Weight {
		// Minimum execution time: 44_122 nanoseconds.
		Weight::from_ref_time(70_066_468)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: UnitOfAccount WhitelistedAccounts (r:1 w:1)
	/// The range of component `c` is `[0, 20000]`.
	fn remove_account(c: u32, ) -> Weight {
		// Minimum execution time: 48_347 nanoseconds.
		Weight::from_ref_time(69_382_497)
			// Standard Error: 160
			.saturating_add(Weight::from_ref_time(512).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: UnitOfAccount WhitelistedAccounts (r:1 w:0)
	// Storage: UnitOfAccount CurrencyBasket (r:1 w:1)
	// Storage: UnitOfAccount UnitOfAccount (r:0 w:1)
	/// The range of component `c` is `[0, 20000]`.
	fn add_currency(c: u32, ) -> Weight {
		// Minimum execution time: 76_054 nanoseconds.
		Weight::from_ref_time(83_503_029)
			// Standard Error: 160
			.saturating_add(Weight::from_ref_time(1_049).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: UnitOfAccount WhitelistedAccounts (r:1 w:0)
	// Storage: UnitOfAccount CurrencyBasket (r:1 w:1)
	// Storage: UnitOfAccount UnitOfAccount (r:0 w:1)
	/// The range of component `c` is `[0, 20000]`.
	fn remove_currency(c: u32, ) -> Weight {
		// Minimum execution time: 74_486 nanoseconds.
		Weight::from_ref_time(91_136_343)
			// Standard Error: 195
			.saturating_add(Weight::from_ref_time(1_379).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn whitelist_account(_c: u32, ) -> Weight {
		Weight::from_ref_time(70_066_468)
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn remove_account(c: u32) -> Weight {
		Weight::from_ref_time(69_382_497)
			// Standard Error: 160
			.saturating_add(Weight::from_ref_time(512).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}

	fn add_currency(c: u32) -> Weight {
		Weight::from_ref_time(83_503_029)
			// Standard Error: 160
			.saturating_add(Weight::from_ref_time(1_049).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}

	fn remove_currency(c: u32) -> Weight {
		Weight::from_ref_time(91_136_343)
			// Standard Error: 195
			.saturating_add(Weight::from_ref_time(1_379).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(2))
	}
}

