use super::*;
use totem_common::converter::Converter;

impl pallet_accounting::Config for Runtime {
    type Event = Event;
    type AccountingConverter = Converter;
    type Currency = Balances;
}

impl pallet_archive::Config for Runtime {
    type Event = Event;
    type Timekeeping = pallet_timekeeping::Pallet<Self>;
}

impl pallet_bonsai::Config for Runtime {
    type Event = Event;
    type Orders = pallet_orders::Pallet<Self>;
    type Projects = pallet_teams::Pallet<Self>;
    type Timekeeping = pallet_timekeeping::Pallet<Self>;
    type BonsaiConverter = Converter;
}

impl pallet_escrow::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type EscrowConverter = Converter;
}

impl pallet_funding::Config for Runtime {
    type Event = Event;
}

impl pallet_orders::Config for Runtime {
    type Event = Event;
    type Accounting = pallet_accounting::Pallet<Self>;
    type Prefunding = pallet_prefunding::Pallet<Self>;
    type Currency = Balances;
    type Bonsai = pallet_bonsai::Pallet<Self>;
    type OrdersConverter = Converter;
}

impl pallet_prefunding::Config for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Pallet<Self>;
    type PrefundingConverter = Converter;
    type Accounting = pallet_accounting::Pallet<Self>;
    type Escrowable = pallet_escrow::Pallet<Self>;
}

impl pallet_teams::Config for Runtime {
    type Event = Event;
}

impl pallet_timekeeping::Config for Runtime {
    type Event = Event;
    type Projects = Teams;
}

impl pallet_transfer::Config for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Pallet<Self>;
    type Bonsai = pallet_bonsai::Pallet<Self>;
}
