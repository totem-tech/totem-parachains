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

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

pub trait Validating<AccountId, Hash> {
    fn is_time_record_owner(o: AccountId, h: Hash) -> bool;

    fn validate_and_archive(o: AccountId, h: Hash, a: bool) -> bool;
}

/// Number of pauses of the timer.
pub type NumberOfBreaks = u16;

/// Quantity of blocks determines the passage of time.
pub type NumberOfBlocks = u64;

pub type StartOrEndBlockNumber = NumberOfBlocks;

/// submitted(0), accepted(1), rejected(2), disputed(3), blocked(4), invoiced(5), reason_code(0), reason text.

/// Not calendar period, but fiscal periods 1-15 (0-14).
pub type PostingPeriod = u16;

//TODO: create an enum: Accepted (true)/Pending (false)
pub type AcceptAssignedStatus = bool;

/// Locked true, unlocked false.
pub type LockStatus = bool;

/// Reason for status change (TODO codes to be defined).
pub type ReasonCode = u16;

/// Category of reason code (TODO categories to be defined).
pub type ReasonCodeType = u16;

/// Reason for status change in text (not on chain!).
// pub type ReasonCodeText = Vec<u8>;

/// Ban status (default is false).
pub type BanStatus = bool;

#[derive(Clone, Copy, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub enum StatusOfTimeRecord {
    Draft,
    Submitted,
    Disputed,
    Rejected,
    Accepted,
    Invoiced,
    Blocked,
}

/// Reason why the code changes.
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct ReasonCodeStruct(pub ReasonCode, pub ReasonCodeType);

/// Status of the code changes.
#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct BannedStruct(BanStatus, ReasonCodeStruct);

/// The individual time record.
#[derive(PartialEq, Eq, Copy, Clone, Debug, Encode, Decode, Default, TypeInfo)]
pub struct Timekeeper<
    AccountId,
    ReferenceHash,
    NumberOfBlocks,
    LockStatus,
    StatusOfTimeRecord,
    ReasonCodeStruct,
    PostingPeriod,
    StartOrEndBlockNumber,
    NumberOfBreaks,
> {
    pub worker: AccountId,
    pub project_hash: ReferenceHash,
    pub total_blocks: NumberOfBlocks,
    pub locked_status: LockStatus,
    pub locked_reason: ReasonCodeStruct,
    pub submit_status: StatusOfTimeRecord,
    pub reason_code: ReasonCodeStruct,
    pub posting_period: PostingPeriod,
    pub start_block: StartOrEndBlockNumber,
    pub end_block: StartOrEndBlockNumber,
    pub nr_of_breaks: NumberOfBreaks,
}

#[cfg(any(test, feature = "mock"))]
impl<AccountId, Hash> Validating<AccountId, Hash> for () {
    fn is_time_record_owner(_o: AccountId, _h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn validate_and_archive(_o: AccountId, _h: Hash, _a: bool) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }
}
