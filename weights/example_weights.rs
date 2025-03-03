
//! Autogenerated weights for `pallet_balances_totem`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-21, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Chriss-iMac.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/totem-parachain-collator
// benchmark
// pallet
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_balances_totem
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --output=./pallets/weights/pallet_balances_totem-new-test.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_balances_totem`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances_totem::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Accounting PostingNumber (r:1 w:1)
	// Storage: Accounting BalanceByLedger (r:4 w:4)
	// Storage: Accounting GlobalLedger (r:2 w:2)
	// Storage: Accounting PostingDetail (r:0 w:4)
	fn transfer() -> Weight {
		// Minimum execution time: 88_000 nanoseconds.
		Weight::from_ref_time(89_000_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Accounting PostingNumber (r:1 w:1)
	// Storage: Accounting BalanceByLedger (r:4 w:4)
	// Storage: Accounting GlobalLedger (r:2 w:2)
	// Storage: Accounting PostingDetail (r:0 w:4)
	fn transfer_keep_alive() -> Weight {
		// Minimum execution time: 83_000 nanoseconds.
		Weight::from_ref_time(83_000_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_creating() -> Weight {
		// Minimum execution time: 21_000 nanoseconds.
		Weight::from_ref_time(22_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_killing() -> Weight {
		// Minimum execution time: 25_000 nanoseconds.
		Weight::from_ref_time(25_000_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Accounting PostingNumber (r:1 w:1)
	// Storage: Accounting BalanceByLedger (r:4 w:4)
	// Storage: Accounting GlobalLedger (r:2 w:2)
	// Storage: Accounting PostingDetail (r:0 w:4)
	fn force_transfer() -> Weight {
		// Minimum execution time: 86_000 nanoseconds.
		Weight::from_ref_time(87_000_000)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Accounting PostingNumber (r:1 w:1)
	// Storage: Accounting BalanceByLedger (r:4 w:4)
	// Storage: Accounting GlobalLedger (r:2 w:2)
	// Storage: Accounting PostingDetail (r:0 w:4)
	fn transfer_all() -> Weight {
		// Minimum execution time: 90_000 nanoseconds.
		Weight::from_ref_time(91_000_000)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: System Account (r:1 w:1)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	// Storage: Accounting PostingNumber (r:1 w:1)
	// Storage: Accounting BalanceByLedger (r:2 w:2)
	// Storage: Accounting GlobalLedger (r:2 w:2)
	// Storage: Accounting PostingDetail (r:0 w:2)
	fn force_unreserve() -> Weight {
		// Minimum execution time: 56_000 nanoseconds.
		Weight::from_ref_time(59_000_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
}
