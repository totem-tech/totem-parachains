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

use crate::LedgerBalance;
use frame_support::{dispatch::EncodeLike, pallet_prelude::*};
use scale_info::TypeInfo;
use sp_std::prelude::*;

pub trait Encumbrance<AccountId, Hash, BlockNumber, CoinAmount> {
    fn prefunding_for(
        who: AccountId,
        recipient: AccountId,
        amount: CoinAmount,
        deadline: BlockNumber,
        ref_hash: Hash,
        uid: Hash,
    ) -> DispatchResultWithPostInfo;

    fn send_simple_invoice(
        who: AccountId,
        recipient: AccountId,
        amount: LedgerBalance,
        ref_hash: Hash,
        uid: Hash,
    ) -> DispatchResultWithPostInfo;

    fn settle_prefunded_invoice(
        who: AccountId,
        ref_hash: Hash,
        uid: Hash,
    ) -> DispatchResultWithPostInfo;

    fn set_release_state(
        who: AccountId,
        o_lock: LockStatus,
        ref_hash: Hash,
        uid: Hash,
    ) -> DispatchResultWithPostInfo;

    fn unlock_funds_for_owner(
        who: AccountId,
        ref_hash: Hash,
        uid: Hash,
    ) -> DispatchResultWithPostInfo;

    fn check_ref_owner(who: AccountId, ref_hash: Hash) -> bool;

    fn check_ref_beneficiary(who: AccountId, ref_hash: Hash) -> bool;
}

#[derive(Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, TypeInfo)]
pub enum LockStatus {
    Unlocked = 0,
    Locked = 1,
}

/// Generic Status for whatever the HashReference refers.
//TODO
pub type Status = u16;

// Implementations

impl EncodeLike<LockStatus> for bool {}
