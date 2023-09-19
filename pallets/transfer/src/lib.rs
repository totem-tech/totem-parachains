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

//! # Replacement for the transfer mechanism in balances module.
//!
//! This module essentially replaces the existing
//! `transfer` function in the balances module by adding an additional tracking
//! mechanism for when the user is offline. It also allows us to manage distribution of funds
//! from the faucet so that funds are not resent to users when there is a network failure.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, StorageVersion},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{BadOrigin};
    use totem_primitives::bonsai::Storing;

    // Other trait types
    type CurrencyBalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Currency: Currency<Self::AccountId>;
        type Bonsai: Storing<Self::Hash>;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO Totem should also ask for the accounting entries for both parties or include some form of reference 
        // to execute accounting recipes.
        /// Transfers funds!
        #[pallet::call_index(0)]
        #[pallet::weight(0/*TODO*/)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            #[pallet::compact] payment_amount: CurrencyBalanceOf<T>,
            tx_uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let from = ensure_signed(origin)?;
            match T::Bonsai::start_tx(tx_uid) {
                Ok(_) => (),
                Err(_) => return Err(Error::<T>::ErrorSettingStartTx.into()),
            };
            let amount = payment_amount;
            if let Err(_) = T::Currency::transfer(&from, &to, amount, ExistenceRequirement::KeepAlive) {
                return Err(Error::<T>::ErrorDuringTransfer.into());
            }
            match T::Bonsai::end_tx(tx_uid) {
                Ok(_) => (),
                Err(_) => return Err(Error::<T>::ErrorSettingEndTx.into()),
            };
            Ok(().into())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// There was an error calling the transfer function in balances.
        ErrorDuringTransfer,
        /// There was an error calling the start_tx function in bonsai.
        ErrorSettingStartTx,
        /// There was an error calling the end_tx function in bonsai.
        ErrorSettingEndTx,
    }

    #[pallet::event]
    pub enum Event<T: Config> {}
}
