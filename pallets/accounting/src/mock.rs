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
use crate::{self as pallet_accounting};
use frame_support::{parameter_types, traits::{ConstU32, ConstU64, GenesisBuild}};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup, IdentifyAccount, Verify},
	MultiSignature,
};
use sp_std::convert::{TryFrom, TryInto};
use totem_common::converter::Converter;
use frame_benchmarking::account;
use totem_primitives::accounting::*;

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
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		RandomnessCollectiveFlip: pallet_randomness::{Pallet, Storage},
		Accounting: pallet_accounting::{Pallet, Storage, Event<T>},
		Balances: pallet_balances_totem::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u16 = 2007;
}

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl system::Config for Test {
	type AccountData = pallet_balances_totem::AccountData<u64>;
	type AccountId = AccountId;
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
    pub static ExistentialDeposit: u64 = 0;
}

impl pallet_randomness::Config for Test {}

parameter_types! {
    pub const MaxReserves: u32 = 2;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<100>;
	type WeightInfo = ();
}

impl pallet_accounting::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AccountingConverter = Converter;
	type Currency = Balances;
	type RandomThing = RandomnessCollectiveFlip;
	type Acc = pallet_accounting::Pallet<Test>;
	type MaxOpeningBalanceAdjustmentDetailsEntry = ConstU32<166>;
	type MaxAdjustmentDetailsEntry = ConstU32<10>;
	type WeightInfo = ();
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

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut endowed_accounts = vec![];
	let account_1 = account::<AccountId>("", 0, 0);
	let account_2 = account::<AccountId>("", 0, 0);

	endowed_accounts.push(account_1);
	endowed_accounts.push(account_2);

	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_accounting::GenesisConfig::<Test> { opening_balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect() }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(10000));
	ext
}

pub fn construct_adjustment_details<Balance: Clone>(ledger: Ledger, credit_amount: Balance, debit_amount: Balance) -> Vec<AdjustmentDetail<Balance>>{
	let mut adjustment_details = vec![];

	let adjustment_detail_credit = AdjustmentDetail {
		ledger,
		debit_credit: Indicator::Credit,
		amount: credit_amount
	};

	let adjustment_detail_debit = AdjustmentDetail {
		ledger,
		debit_credit: Indicator::Debit,
		amount: debit_amount
	};

	adjustment_details.push(adjustment_detail_credit);
	adjustment_details.push(adjustment_detail_debit);

	adjustment_details
}

pub fn construct_adjustment_details_for_too_many_entries<Balance: Clone>(credit_amount: Balance) -> Vec<AdjustmentDetail<Balance>>{
	let mut adjustment_details = vec![];

	let mut length = 200;

	while length != 0 {
		let adjustment_detail_asset = AdjustmentDetail {
			ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount))),
			debit_credit: Indicator::Credit,
			amount: credit_amount.clone()
		};
		adjustment_details.push(adjustment_detail_asset);

		length -= 1;
	}

	adjustment_details
}

pub fn construct_adjustment_details_for_credit_debit_mismatch<Balance: Clone>(credit_amount: Balance, debit_amount: Balance) -> Vec<AdjustmentDetail<Balance>>{
	let mut adjustment_details = vec![];

	let adjustment_detail_asset_1 = AdjustmentDetail {
		ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount))),
		debit_credit: Indicator::Credit,
		amount: credit_amount.clone()
	};

	let adjustment_detail_asset_2 = AdjustmentDetail {
		ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::BankCurrentAccount))),
		debit_credit: Indicator::Credit,
		amount: credit_amount.clone()
	};
	adjustment_details.push(adjustment_detail_asset_1);
	adjustment_details.push(adjustment_detail_asset_2);


	let adjustment_detail_liabilities_1 = AdjustmentDetail {
		ledger: Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities))),
		debit_credit: Indicator::Debit,
		amount: debit_amount.clone()
	};

	let adjustment_detail_liabilities_2 = AdjustmentDetail {
		ledger: Ledger::BalanceSheet(B::Liabilities(L::CurrentLiabilities(CurrentLiabilities::ContractLiabilities))),
		debit_credit: Indicator::Debit,
		amount: debit_amount.clone()
	};

	adjustment_details.push(adjustment_detail_liabilities_1);
	adjustment_details.push(adjustment_detail_liabilities_2);

	let adjustment_detail_equity_1 = AdjustmentDetail {
		ledger: Ledger::BalanceSheet(B::Equity(E::CapitalStock(CapitalStock::OrdinaryShares))),
		debit_credit: Indicator::Credit,
		amount: debit_amount.clone()
	};

	let adjustment_detail_equity_2 = AdjustmentDetail {
		ledger: Ledger::BalanceSheet(B::Equity(E::CapitalStock(CapitalStock::OrdinaryShares))),
		debit_credit: Indicator::Credit,
		amount: debit_amount.clone()
	};

	adjustment_details.push(adjustment_detail_equity_1);
	adjustment_details.push(adjustment_detail_equity_2);

	adjustment_details
}

