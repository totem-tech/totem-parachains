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
        fail,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;
    use totem_primitives::bonsai::Storing;

    // Other trait types
    type CurrencyBalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId>;
        type Bonsai: Storing<Self::Hash>;
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Transfers funds!
        #[pallet::weight(0/*TODO*/)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            #[pallet::compact] payment_amount: CurrencyBalanceOf<T>,
            tx_uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let from = ensure_signed(origin)?;
            T::Bonsai::start_tx(tx_uid)?;
            // Convert incoming amount to currency for transfer
            //let amount: CurrencyBalanceOf<T> = T::TransferConverter::convert(payment_amount);
            let amount = payment_amount;

            if let Err(_) =
                T::Currency::transfer(&from, &to, amount, ExistenceRequirement::KeepAlive)
            {
                fail!(Error::<T>::ErrorDuringTransfer);
            }

            T::Bonsai::end_tx(tx_uid)?;

            Ok(().into())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// There was an error calling the transfer function in balances.
        ErrorDuringTransfer,
    }

    #[pallet::event]
    pub enum Event<T: Config> {}
}
