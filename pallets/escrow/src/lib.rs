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

//! Allows to locks amount thanks to an escrow account.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use codec::{Decode, Encode, MaxEncodedLen};
    use scale_info::TypeInfo;
    use frame_support::{
        fail,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement::KeepAlive, LockIdentifier, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    use sp_runtime::traits::{Convert, Zero};
    use sp_std::{fmt::Debug, prelude::*, result};

    use totem_primitives::escrow::{EscrowableCurrency, Reason, TotemLocksError};

    type EscrowableBalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn escrowed)]
    pub type Escrowed<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        LockIdentifier,
        EscrowedAmount<EscrowableBalanceOf<T>, T::BlockNumber>,
    >;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Currency: Currency<Self::AccountId>;
        type EscrowConverter: Convert<[u8; 32], Self::AccountId>;
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::event]
    //#[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {}

    /// The Totem version of single lock on a balance.
    /// There can be many of these on an account and they "overlap",
    /// so the same balance is frozen by multiple locks.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo)]
    pub struct EscrowedAmount<Balance, BlockNumber> {
        /// The amount which the free balance may not drop below when this lock is in effect.
        pub amount: Balance,
        /// If true, then the lock remains in effect even for payment of transaction fees.
        pub reason: Reason,
        /// The lock deadline.
        pub until: BlockNumber,
    }

    const ESCROW: WithdrawReasons = unsafe { WithdrawReasons::from_bits_unchecked(0b1000_0000) };

    impl<T: Config> EscrowableCurrency<T::AccountId> for Pallet<T> {
        type Moment = T::BlockNumber;

        type Currency = T::Currency;

        fn escrow_account() -> T::AccountId {
            let escrow_account: [u8; 32] = *b"TotemsEscrowAddress4LockingFunds";

            T::EscrowConverter::convert(escrow_account)
        }

        // Set a lock on the balance of `who`.
        // Is a no-op if lock amount is zero or `reasons` `is_none()`.
        fn set_lock(
            id: LockIdentifier,
            who: &T::AccountId,
            amount: EscrowableBalanceOf<T>,
            until: T::BlockNumber,
            reason: Reason,
        ) -> Result<(), TotemLocksError> {
            if amount.is_zero() {
                fail!(TotemLocksError::ZeroAmount)
            }

            let now = frame_system::Pallet::<T>::block_number();
            if now > until {
                fail!(TotemLocksError::InvalidDeadline)
            }

            Escrowed::<T>::try_mutate(who, id, |maybe_escrowed| match maybe_escrowed {
                Some(_) => return Err(TotemLocksError::IdAlreadyExists),
                slot @ &mut None => {
                    Self::transfer_to_the_escrow(who, amount)?;

                    *slot = Some(EscrowedAmount {
                        amount,
                        reason,
                        until,
                    });

                    Ok(())
                }
            })
        }

        fn remove_lock(id: LockIdentifier, who: &T::AccountId) -> Result<(), TotemLocksError> {
            Escrowed::<T>::try_mutate_exists(who, id, |maybe_escrowed| match maybe_escrowed {
                Some(escrowed) => {
                    Self::transfer_from_the_escrow(who, escrowed.amount)?;

                    *maybe_escrowed = None;

                    Ok(())
                }
                None => return Err(TotemLocksError::IdDoesNotExist),
            })
        }
    }

    impl<T: Config> Pallet<T> {
        fn transfer_to_the_escrow(
            who: &T::AccountId,
            amount: EscrowableBalanceOf<T>,
        ) -> result::Result<(), TotemLocksError> {
            let imba = <T as Config>::Currency::withdraw(who, amount, ESCROW, KeepAlive)?;

            <T as Config>::Currency::make_free_balance_be(&Self::escrow_account(), amount);
            let _imba_resolved = imba;

            Ok(())
        }

        fn transfer_from_the_escrow(
            who: &T::AccountId,
            amount: EscrowableBalanceOf<T>,
        ) -> result::Result<(), TotemLocksError> {
            let imba = <T as Config>::Currency::withdraw(
                &Self::escrow_account(),
                amount,
                ESCROW,
                KeepAlive,
            )?;

            <T as Config>::Currency::make_free_balance_be(who, amount);
            let _imba_resolved = imba;

            Ok(())
        }
    }
}
