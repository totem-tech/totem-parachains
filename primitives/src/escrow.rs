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

use codec::MaxEncodedLen;
use frame_support::{
    pallet_prelude::*,
    traits::{Currency, LockIdentifier},
};
use scale_info::TypeInfo;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo)]
pub enum Reason {
    Escrowing,
}

/// A currency whose accounts can have liquidity restrictions.
pub trait EscrowableCurrency<AccountId> {
    /// The quantity used to denote time; usually just a `BlockNumber`.
    type Moment;

    /// The maximum number of locks a user should have on their account.
    type Currency: Currency<AccountId>;

    /// This function simply returns the Totem escrow account address.
    fn escrow_account() -> AccountId;

    /// Create a new balance lock on account `who`.
    ///
    /// If the new lock is valid (i.e. not already expired), it will push the struct to
    /// the `Locks` vec in storage.
    fn set_lock(
        id: LockIdentifier,
        who: &AccountId,
        amount: <Self::Currency as Currency<AccountId>>::Balance,
        until: Self::Moment,
        reason: Reason,
    ) -> Result<(), TotemLocksError>;

    /// Remove an existing lock.
    fn remove_lock(id: LockIdentifier, who: &AccountId) -> Result<(), TotemLocksError>;
}

pub enum TotemLocksError {
    /// There is no enough place remaining to add a lock for this account.
    NoPlaceRemaining,
    /// The amount to be locked is zero.
    ZeroAmount,
    /// The ID already exists, thus the locks already exists.
    IdAlreadyExists,
    /// There is no escrowed value at this ID.
    IdDoesNotExist,
    /// The deadline must be in the future.
    InvalidDeadline,
    /// The fund cannot be tranfered to the escrow.
    CannotTransferToTheEscrow(DispatchError),
}

// Implementations

impl From<DispatchError> for TotemLocksError {
    fn from(e: DispatchError) -> TotemLocksError {
        TotemLocksError::CannotTransferToTheEscrow(e)
    }
}
