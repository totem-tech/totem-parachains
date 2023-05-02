//                              Næ§@@@ÑÉ©
//                        æ@@@@@@@@@@@@@@@@@@
//                    Ñ@@@@?.?@@@@@@@@@@@@@@@@@@@N
//                 ¶@@@@@?^%@@.=@@@@@@@@@@@@@@@@@@@@
//               N@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^@@@».............?@@@@@@@@@É
//              Ñ@@@@@@@@?^@@@@@@@@@@@@@@@@@@'?@@@@@@@@Ñ
//              @@@@@@@@@?^@@@»..............»@@@@@@@@@@
//              @@@@@@@@@?^@@@»^@@@@@@@@@@@@@@@@@@@@@@@@
//              @@@@@@@@@?^ë@@&.@@@@@@@@@@@@@@@@@@@@@@@@
//               @@@@@@@@?^´@@@o.%@@@@@@@@@@@@@@@@@@@@©
//                @@@@@@@?.´@@@@@ë.........*.±@@@@@@@æ
//                 @@@@@@@@?´.I@@@@@@@@@@@@@@.&@@@@@N
//                  N@@@@@@@@@@ë.*=????????=?@@@@@Ñ
//                    @@@@@@@@@@@@@@@@@@@@@@@@@@@¶
//                        É@@@@@@@@@@@@@@@@Ñ¶
//                             Næ§@@@ÑÉ©

// Copyright 2020 Chris D'Costa
// This file is part of Totem Live Accounting.
// Authors:
// - Félix Daudré-Vignier   email: felix@totemaccounting.com
// - Chris D'Costa          email: chris.dcosta@totemaccounting.com

// Totem is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Totem is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.

#![cfg(test)]

use crate as pallet_orders;

use frame_support::{
	parameter_types,
	traits::{GenesisBuild, OnFinalize, OnInitialize},
};
use frame_support::traits::{ ConstU32, ConstU64 };
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{
		BlakeTwo256,
		IdentityLookup,
	},
};
use totem_common::converter::Converter;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances_totem::{ Pallet, Call, Storage, Event<T> },
		Orders: pallet_orders::{Pallet, Call, Storage, Event<T>},
        Accounting: pallet_accounting::{Pallet, Storage, Event<T>},
		Bonsai: pallet_bonsai::{Pallet, Call, Storage, Event<T>},
		Prefunding: pallet_prefunding::{Pallet, Call, Storage, Event<T>},
		Timekeeping: pallet_timekeeping::{Pallet, Call, Storage, Event<T>},
		Teams: pallet_teams::{Pallet, Call, Storage, Event<T>},
		Escrow: pallet_escrow::{Pallet, Call, Storage, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness::{Pallet, Storage},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u16 = 2007;
}

impl system::Config for Test {
	type AccountData = pallet_balances_totem::AccountData<u64>;
	type AccountId = u64;
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type RuntimeCall = RuntimeCall;
	type DbWeight = ();
	type RuntimeEvent = RuntimeEvent;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type RuntimeOrigin = RuntimeOrigin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_accounting::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AccountingConverter = Converter;
	type Currency = Balances;
	type RandomThing = RandomnessCollectiveFlip;
}

impl pallet_balances_totem::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = system::Pallet<Test>;
	type MaxLocks = ();
	type MaxReserves = ConstU32<2>;
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = pallet_balances_totem::weights::SubstrateWeight<Test>;
	type Accounting = Accounting;
}

impl pallet_bonsai::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Timekeeping = Timekeeping;
	type Teams = Teams;
	type Orders = Orders;
	type BonsaiConverter = Converter;
}

impl pallet_orders::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type OrdersConverter = Converter;
	type Accounting = Accounting;
	type Prefunding = Prefunding;
	type Bonsai = Bonsai;
	type Currency = Balances;
}

impl pallet_teams::Config for Test {
	type RuntimeEvent = RuntimeEvent;
}

impl pallet_timekeeping::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Teams = Teams;
}

impl pallet_escrow::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type EscrowConverter = Converter;
}

impl pallet_randomness::Config for Test {}

impl pallet_prefunding::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Escrowable = Escrow;
	type PrefundingConverter = Converter;
	type Accounting = Accounting;
	type RandomThing = RandomnessCollectiveFlip;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<100>;
	type WeightInfo = ();
}
parameter_types! {
	pub const QueueCount: u32 = 3;
	pub const MaxQueueLen: u32 = 3;
	pub const FifoQueueLen: u32 = 1;
	pub const Period: u64 = 3;
	pub const MinFreeze: u64 = 2;
	pub const IntakePeriod: u64 = 2;
	pub const MaxIntakeBids: u32 = 2;
}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {
	/*let mut endowed_accounts = vec![];

	endowed_accounts.push(1);
	endowed_accounts.push(2);

	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances_totem::GenesisConfig::<Test> { balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect() }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(10000));
	ext*/
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap().into();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(10000000));
	ext
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		Orders::on_finalize(System::block_number());
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
		Orders::on_initialize(System::block_number());
	}
}
