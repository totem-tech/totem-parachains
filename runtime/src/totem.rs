use super::*;
use totem_common::converter::Converter;

impl pallet_accounting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AccountingConverter = Converter;
    type Currency = pallet_balances_totem::Pallet<Self>;
    type RandomThing = RandomnessCollectiveFlip;
    type Acc = pallet_accounting::Pallet<Self>;
    // type WeightInfo = ();
}


parameter_types! {
    pub const MaxWhitelistedAccounts: u32 = 50;
    pub const MaxTickersInBasket: u32 = 1000;
    pub const MaxTickersInput: u32 = 100;
    pub const SymbolMaxChars: u32 = 7;
    pub const WhitelistDeposit: Balance = WHITELIST_DEPOSIT;
    pub const AccountBytes: [u8; 32] = *b"totems/whitelist/deposit/account";
}

impl pallet_unit_of_account::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = pallet_balances_totem::Pallet<Runtime>;
    type WeightInfo = pallet_unit_of_account::weights::TotemWeight<Runtime>;
    type MaxWhitelistedAccounts = MaxWhitelistedAccounts;
    type MaxTickersInBasket = MaxTickersInBasket;
    type MaxTickersInput = MaxTickersInput;
    type AccountBytes = AccountBytes;
    type UnitOfAccountConverter = Converter;
    type WhitelistDeposit = WhitelistDeposit;
}

impl pallet_archive::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Timekeeping = pallet_timekeeping::Pallet<Self>;
}

impl pallet_bonsai::Config for Runtime {
    // type RuntimeEvent = RuntimeEvent; // Event<T> temporarily removed and commented out in pallet_bonsai
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
