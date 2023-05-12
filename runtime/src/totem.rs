use super::*;
use totem_common::converter::Converter;

impl pallet_accounting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AccountingConverter = Converter;
    type Currency = pallet_balances_totem::Pallet<Self>;
    type RandomThing = RandomnessCollectiveFlip;
}

impl pallet_archive::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Timekeeping = pallet_timekeeping::Pallet<Self>;
	type WeightInfo = pallet_archive::weights::TotemWeight<Runtime>;
}

impl pallet_bonsai::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Orders = pallet_orders::Pallet<Self>;
    type Teams = pallet_teams::Pallet<Self>;
    type Timekeeping = pallet_timekeeping::Pallet<Self>;
    type BonsaiConverter = Converter;
}

impl pallet_escrow::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = pallet_balances_totem::Pallet<Self>;
    type EscrowConverter = Converter;
}

// impl pallet_funding::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
// }

impl pallet_orders::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Accounting = pallet_accounting::Pallet<Self>;
    type Prefunding = pallet_prefunding::Pallet<Self>;
    type Currency = pallet_balances_totem::Pallet<Self>;
    type Bonsai = pallet_bonsai::Pallet<Self>;
    type OrdersConverter = Converter;
}

impl pallet_prefunding::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = pallet_balances_totem::Pallet<Self>;
    type PrefundingConverter = Converter;
    type Accounting = pallet_accounting::Pallet<Self>;
    type Escrowable = pallet_escrow::Pallet<Self>;
    type RandomThing = RandomnessCollectiveFlip;
}

impl pallet_teams::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}

impl pallet_timekeeping::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Teams = pallet_teams::Pallet<Self>;
}

// impl pallet_transfer::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type Currency = pallet_balances_totem::Pallet<Self>;
//     type Bonsai = pallet_bonsai::Pallet<Self>;
// }
