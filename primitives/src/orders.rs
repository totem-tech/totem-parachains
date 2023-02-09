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

use frame_support::{dispatch::EncodeLike, pallet_prelude::*};
use sp_std::prelude::*;
use scale_info::TypeInfo;

pub trait Validating<AccountId, Hash> {
    fn is_order_party(o: AccountId, r: Hash) -> bool;
}

// Module Types
pub type OrderStatus = u16; // Generic Status for whatever the HashReference refers to

#[derive(Debug, Decode, Encode, Clone, Copy, PartialEq, Eq, TypeInfo)]
pub enum ApprovalStatus {
    Submitted = 0,
    Accepted = 1,
    Rejected = 2,
}

/// The order header: contains common values for all items.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct OrderHeader<AccountId> {
    pub commander: AccountId,
    pub fulfiller: AccountId,
    pub approver: AccountId,
    pub order_status: u16,
    pub approval_status: ApprovalStatus,
    pub buy_or_sell: u16,
    pub amount: i128,
    pub market_order: bool,
    pub order_type: u16,
    pub deadline: u32,
    pub due_date: u32,
}

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct OrderItem<Hash> {
    pub product: Hash,
    pub unit_price: i128,
    pub quantity: u128,
    pub unit_of_measure: u16,
}

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct TxKeysL<Hash> {
    pub record_id: Hash,
    pub parent_id: Hash,
    pub bonsai_token: Hash,
    pub tx_uid: Hash,
}

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct TxKeysM<Hash> {
    pub record_id: Hash,
    pub bonsai_token: Hash,
    pub tx_uid: Hash,
}

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct TxKeysS<Hash> {
    pub bonsai_token: Hash,
    pub tx_uid: Hash,
}

// Implementations

impl EncodeLike<ApprovalStatus> for u8 {}

impl Default for ApprovalStatus {
    fn default() -> Self {
        ApprovalStatus::Submitted
    }
}
