use super::*;
use crate::{self as pallet_unit_of_account};

use frame_support::{parameter_types, traits::ConstU64};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConstU32, IdentifyAccount, IdentityLookup, Verify},
	MultiSignature,
};
use sp_std::convert::{TryFrom, TryInto};
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
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		PalletUnitOfAccount: pallet_unit_of_account::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances_totem::{Pallet, Call, Storage, Event<T>},
		Accounting: pallet_accounting::{Pallet, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
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
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<100>;
	type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

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
	type Accounting = pallet_accounting::Pallet<Test>;
}

parameter_types! {
	pub const WhitelistDeposit: u128 = 0;
	pub const AccountBytes: [u8; 32] = *b"totems/whitelist/deposit/account";
}

impl pallet_unit_of_account::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = pallet_balances_totem::Pallet<Test>;
	type MaxWhitelistedAccounts = ConstU32<5>;
	type MaxAssetsInBasket = ConstU32<3>;
	type MaxAssetsInput = ConstU32<100>;
	type SymbolMaxChars = ConstU32<7>;
	type AccountBytes = AccountBytes;
	type BytesToAccountId = Converter;
	type WhitelistDeposit = WhitelistDeposit;
	type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
