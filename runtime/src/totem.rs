use super::*;
use totem_common::converter::Converter;

impl pallet_accounting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AccountingConverter = Converter;
    type Currency = Balances;
    type RandomThing = RandomnessCollectiveFlip;
}

impl pallet_unit_of_account::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = pallet_balances_totem::Pallet<Runtime>;
    type MaxWhitelistedAccounts: u32 = 50;
    type MaxAssetsInBasket: u32 = 1000;
    type SymbolMaxChars = ConstU32<7>;
    type WhitelistDeposit: u128 = 1_000_000_000_000_000;
    type WeightInfo = pallet_unit_of_account::weights::TotemWeight<Runtime>;
}

// impl pallet_archive::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Timekeeping = pallet_timekeeping::Pallet<Self>;
// }

// impl pallet_bonsai::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Orders = pallet_orders::Pallet<Self>;
//     type Projects = pallet_teams::Pallet<Self>;
//     type Timekeeping = pallet_timekeeping::Pallet<Self>;
//     type BonsaiConverter = Converter;
// }

// impl pallet_escrow::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = Balances;
//     type EscrowConverter = Converter;
// }

// impl pallet_funding::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
// }

// impl pallet_orders::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Accounting = pallet_accounting::Pallet<Self>;
//     type Prefunding = pallet_prefunding::Pallet<Self>;
//     type Currency = Balances;
//     type Bonsai = pallet_bonsai::Pallet<Self>;
//     type OrdersConverter = Converter;
// }

// impl pallet_prefunding::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = pallet_balances::Pallet<Self>;
//     type PrefundingConverter = Converter;
//     type Accounting = pallet_accounting::Pallet<Self>;
//     type Escrowable = pallet_escrow::Pallet<Self>;
// }

// impl pallet_teams::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
// }

// impl pallet_timekeeping::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Projects = Teams;
// }

// impl pallet_transfer::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = pallet_balances::Pallet<Self>;
//     type Bonsai = pallet_bonsai::Pallet<Self>;
// }
