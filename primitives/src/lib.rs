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

#![cfg_attr(not(feature = "std"), no_std)]

pub mod accounting;
pub mod archive;
pub mod bonsai;
pub mod escrow;
pub mod funding;
pub mod orders;
pub mod prefunding;
pub mod teams;
pub mod timekeeping;
pub mod transfer;

use frame_support::{dispatch::EncodeLike, pallet_prelude::*};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

/// Balance on an account can be negative.
pub type LedgerBalance = i128;

/// General ledger account number.
pub type Account = u64;

/// The index number for identifying the posting to ledgers.
pub type PostingIndex = u128;

/// Used for comparisons.
pub type ComparisonAmounts = u128;

#[derive(Decode, Encode, Debug, Clone, Copy, PartialEq, TypeInfo)]
pub enum RecordType {
    Teams,
    Timekeeping,
    Orders,
}

/// A collection based on `Vec` that guarantees that every member is unique.
#[derive(Decode, Encode, TypeInfo)]
pub struct Set<T>(Vec<T>);

// Implementations

impl EncodeLike<RecordType> for u8 {}

impl<T> Default for Set<T> {
    fn default() -> Self {
        Set(Default::default())
    }
}

impl<T> Set<T>
where
    T: Eq,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, elem: T) {
        self.remove(&elem);
        self.0.push(elem)
    }

    pub fn remove(&mut self, elem: &T) {
        self.0.retain(|x| x != elem)
    }
}
