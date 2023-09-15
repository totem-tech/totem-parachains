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

pub mod benchmarking;
pub mod mock;
pub mod tests;

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::StorageVersion,
    };
    use frame_system::pallet_prelude::*;

    use sp_std::prelude::*;

    use totem_primitives::{timekeeping::Validating as TimeValidating, RecordType};

    type Archival = bool;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Timekeeping: TimeValidating<Self::AccountId, Self::Hash>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Unknown or unimplemented record type. Cannot archive record
        UnknownRecordType,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Archive types
        /// 1000
        /// 2000
        /// 3000 Activities (previously Projects)
        /// 4000 Timekeeping
        /// 5000 Orders
        /// 6000
        /// 7000
        /// 8000
        /// 9000
        #[pallet::call_index(0)]
        #[pallet::weight(0/*TODO*/)]
        pub fn archive_record(
            origin: OriginFor<T>,
            record_type: RecordType,
            bonsai_token: T::Hash,
            archive: bool,
        ) -> DispatchResultWithPostInfo {
            // check signed
            let who = ensure_signed(origin)?;
            // check which type of record
            match record_type {
                RecordType::Timekeeping => {
                    // module specific archive handling
                    if T::Timekeeping::validate_and_archive(who.clone(), bonsai_token, archive) {
                        // issue event
                        Self::deposit_event(Event::RecordArchived(
                            RecordType::Timekeeping,
                            who,
                            bonsai_token,
                            archive,
                        ));
                    }
                }
                _ => return Err(Error::<T>::UnknownRecordType.into()),
            }

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        RecordArchived(RecordType, T::AccountId, T::Hash, Archival),
    }
}