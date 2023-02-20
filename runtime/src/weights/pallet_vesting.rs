
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ubuntu-s-8vcpu-16gb-fra1-01`, CPU: `DO-Regular`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/totem-parachain-collator
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_vesting
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --output=./weights/pallet_vesting-new-test.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_vesting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vesting::WeightInfo for WeightInfo<T> {
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_locked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 106_433 nanoseconds.
		Weight::from_ref_time(127_250_875)
			// Standard Error: 87_498
			.saturating_add(Weight::from_ref_time(53_941).saturating_mul(l.into()))
			// Standard Error: 155_674
			.saturating_add(Weight::from_ref_time(2_279_746).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 111_409 nanoseconds.
		Weight::from_ref_time(146_451_323)
			// Standard Error: 101_124
			.saturating_add(Weight::from_ref_time(1_085_994).saturating_mul(l.into()))
			// Standard Error: 179_917
			.saturating_add(Weight::from_ref_time(877_337).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_locked(_l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 105_574 nanoseconds.
		Weight::from_ref_time(164_370_259)
			// Standard Error: 177_783
			.saturating_add(Weight::from_ref_time(1_470_947).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[1, 28]`.
	fn vest_other_unlocked(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 108_222 nanoseconds.
		Weight::from_ref_time(81_189_239)
			// Standard Error: 104_926
			.saturating_add(Weight::from_ref_time(1_288_548).saturating_mul(l.into()))
			// Standard Error: 186_682
			.saturating_add(Weight::from_ref_time(3_295_726).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Accounting PostingNumber (r:1 w:1)
	// Storage: Accounting BalanceByLedger (r:4 w:4)
	// Storage: Accounting GlobalLedger (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Accounting PostingDetail (r:0 w:4)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn vested_transfer(_l: u32, _s: u32, ) -> Weight {
		// Minimum execution time: 290_826 nanoseconds.
		Weight::from_ref_time(540_821_283)
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(14))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Accounting PostingNumber (r:1 w:1)
	// Storage: Accounting BalanceByLedger (r:4 w:4)
	// Storage: Accounting GlobalLedger (r:2 w:2)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Accounting PostingDetail (r:0 w:4)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[0, 27]`.
	fn force_vested_transfer(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 299_390 nanoseconds.
		Weight::from_ref_time(252_950_070)
			// Standard Error: 253_675
			.saturating_add(Weight::from_ref_time(3_951_098).saturating_mul(l.into()))
			// Standard Error: 451_333
			.saturating_add(Weight::from_ref_time(6_334_926).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(15))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn not_unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 113_076 nanoseconds.
		Weight::from_ref_time(156_638_302)
			// Standard Error: 100_027
			.saturating_add(Weight::from_ref_time(566_415).saturating_mul(l.into()))
			// Standard Error: 184_725
			.saturating_add(Weight::from_ref_time(1_881_253).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Vesting Vesting (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `l` is `[0, 49]`.
	/// The range of component `s` is `[2, 28]`.
	fn unlocking_merge_schedules(l: u32, s: u32, ) -> Weight {
		// Minimum execution time: 112_673 nanoseconds.
		Weight::from_ref_time(107_258_168)
			// Standard Error: 102_794
			.saturating_add(Weight::from_ref_time(693_133).saturating_mul(l.into()))
			// Standard Error: 189_835
			.saturating_add(Weight::from_ref_time(4_308_209).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
