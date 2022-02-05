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
    fn is_project_owner(o: AccountId, h: Hash) -> bool;

    fn is_project_valid(h: Hash) -> bool;

    fn is_owner_and_project_valid(o: AccountId, h: Hash) -> bool;
}

/// Reference supplied externally.
//TODO make an enum
pub type ProjectStatus = u16;

#[derive(PartialEq, Eq, Clone, Debug, Encode, Decode, TypeInfo)]
pub struct DeletedProject<AccountId, ProjectStatus> {
    pub owned_by: AccountId,
    pub deleted_by: AccountId,
    pub status: ProjectStatus,
}

#[cfg(any(test, feature = "mock"))]
impl<AccountId, Hash> Validating<AccountId, Hash> for () {
    fn is_project_owner(_o: AccountId, _h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn is_project_valid(_h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn is_owner_and_project_valid(_o: AccountId, _h: Hash) -> bool {
        unimplemented!("Used as a mock, shouldn't be called")
    }
}
