use super::*;
use totem_common::converter::Converter;

impl pallet_accounting::Config for Runtime {
    type Event = Event;
    type AccountingConverter = Converter;
    type Currency = Balances;
}

impl pallet_archive::Config for Runtime {
    type Event = Event;
    type Timekeeping = Timekeeping;
}

impl pallet_bonsai::Config for Runtime {
    type Event = Event;
    type Orders = Orders;
    type Projects = Teams;
    type Timekeeping = Timekeeping;
    type BonsaiConverter = Converter;
}

impl pallet_escrow::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type EscrowConverter = Converter;
}

// impl pallet_funding::Config for Runtime {
//     type Event = Event;
// }

impl pallet_orders::Config for Runtime {
    type Event = Event;
    type Accounting = Accounting;
    type Prefunding = Prefunding;
    type Currency = Balances;
    type Bonsai = Bonsai;
    type OrdersConverter = Converter;
}

impl pallet_prefunding::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type PrefundingConverter = Converter;
    type Accounting = Accounting;
    type Escrowable = Escrow;
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
    type Currency = Balances;
    type Bonsai = Bonsai;
}
