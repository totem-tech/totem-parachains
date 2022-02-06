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

//! Allows to fund/crowdsale Totem.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{fail, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use totem_primitives::Set;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Defines if the transfer mechanism is open yet.
    #[pallet::storage]
    #[pallet::getter(fn transfer_status)]
    pub type TransferStatus<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// The Maximum Quantity of Coins that can be minted.
    #[pallet::storage]
    #[pallet::getter(fn max_issuance)]
    pub type MaxlIssuance<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Initially 45% of Supply (Reserved Funds).
    #[pallet::storage]
    #[pallet::getter(fn unissued)]
    pub type UnIssued<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Initially 55% of Supply Reduces as funds distributed.
    #[pallet::storage]
    #[pallet::getter(fn issued)]
    pub type Issued<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Controller of funds (Live Accounting Association Account).
    #[pallet::storage]
    #[pallet::getter(fn controller)]
    pub type Controller<T: Config> = StorageValue<_, T::AccountId>;

    /// The number of coins distributed. It should equal the sum in AccountIdBalances.
    #[pallet::storage]
    #[pallet::getter(fn total_distributed)]
    pub type TotalDistributed<T: Config> = StorageValue<_, u128, ValueQuery>;

    /// Place to store investors accountids with balances.
    #[pallet::storage]
    #[pallet::getter(fn account_id_balances)]
    pub type AccountIdBalances<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u128>;

    /// List of account Ids who have tokens (updated when token value is 0).
    #[pallet::storage]
    #[pallet::getter(fn holders_account_ids)]
    pub type HoldersAccountIds<T: Config> = StorageValue<_, Set<T::AccountId>, ValueQuery>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        transfer_status: bool,
        max_issuance: u128,
        unissued: u128,
        issued: u128,
        _marker: PhantomData<T>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            GenesisConfig {
                transfer_status: false,
                max_issuance: 161_803_398_875_u128,
                unissued: 72_811_529_493_u128,
                issued: 88_991_869_382_u128,
                _marker: PhantomData,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            TransferStatus::<T>::put(self.transfer_status);
            MaxlIssuance::<T>::put(self.max_issuance);
            UnIssued::<T>::put(self.unissued);
            Issued::<T>::put(self.issued);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Super User sets the controller account.
        #[pallet::weight(0/*TODO*/)]
        pub fn set_controller_account(
            origin: OriginFor<T>,
            controller: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            // Only Sudo
            let _who = ensure_root(origin)?;

            // abandon if this is the same controller
            if Self::is_controller(&controller) {
                fail!(Error::<T>::SameController);
            }

            // insert new controller
            Controller::<T>::put(controller);

            Ok(().into())
        }

        /// Super User sets the transfers to open or closed.
        #[pallet::weight(0/*TODO*/)]
        pub fn set_transfer_status(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _who = ensure_root(origin)?;

            if Self::transfer_status() {
                TransferStatus::<T>::put(false)
            } else if Self::check_setup() {
                TransferStatus::<T>::put(true)
            } else {
                fail!(Error::<T>::ControllerNotSet);
            }

            Ok(().into())
        }

        /// Super User can only mint coins if transfers are disabled.
        #[pallet::weight(0/*TODO*/)]
        pub fn mint_coins(origin: OriginFor<T>, quantity: u128) -> DispatchResultWithPostInfo {
            let _who = ensure_root(origin)?;

            if Self::transfer_status() {
                fail!(Error::<T>::CannotMintCoins);
            }

            let supply = Self::max_issuance()
                .checked_add(quantity)
                .ok_or(Error::<T>::Overflow)?;
            let unissued = Self::unissued()
                .checked_add(quantity)
                .ok_or(Error::<T>::Overflow)?;

            // Update unissued account with new balance
            UnIssued::<T>::put(unissued);
            // Update Max Supply
            MaxlIssuance::<T>::put(supply);

            Ok(().into())
        }

        /// Super User can move from unissued to issued coins if transfers are disabled.
        #[pallet::weight(0/*TODO*/)]
        pub fn rebalance_issued_coins(
            origin: OriginFor<T>,
            amount: u128,
        ) -> DispatchResultWithPostInfo {
            let _who = ensure_root(origin)?;
            let unissued = Self::unissued();

            // check that the amount is not greater than the available funds
            if amount > unissued {
                fail!(Error::<T>::InsufficientFunds);
            }

            // Those should never error.
            let unissued = unissued.checked_sub(amount).ok_or(Error::<T>::Overflow)?;
            let issued = Self::issued()
                .checked_add(amount)
                .ok_or(Error::<T>::Overflow)?;

            UnIssued::<T>::put(unissued);
            Issued::<T>::put(issued);

            Ok(().into())
        }

        /// Only the controller can do the initial distribution
        #[pallet::weight(0/*TODO*/)]
        pub fn distribute(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u128,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let issued = Self::issued();

            // ensure that this is the controller account
            if Self::is_controller(&who) == false {
                fail!(Error::<T>::NotController);
            }

            if amount > issued {
                fail!(Error::<T>::InsufficientFunds);
            }

            let issued = issued.checked_sub(amount).ok_or(Error::<T>::Overflow)?;
            let new_balance = match Self::account_id_balances(&to) {
                Some(b) => {
                    let new_balance = b.checked_add(amount).ok_or(Error::<T>::Overflow)?;
                    AccountIdBalances::<T>::take(&to);
                    new_balance
                }
                None => 0,
            };
            // Ensure that the amount to send is less the available funds.
            let total_distributed = Self::total_distributed()
                .checked_add(amount)
                .ok_or(Error::<T>::Overflow)?;

            Issued::<T>::put(issued);
            AccountIdBalances::<T>::insert(&to, new_balance);
            TotalDistributed::<T>::put(total_distributed);
            HoldersAccountIds::<T>::mutate(|holders_account_ids| holders_account_ids.insert(to));

            Ok(().into())
        }

        /// This function transfers funds between accounts (only when opened)
        #[pallet::weight(0/*TODO*/)]
        pub fn transfer(
            origin: OriginFor<T>,
            to: T::AccountId,
            amount: u128,
        ) -> DispatchResultWithPostInfo {
            let from = ensure_signed(origin)?;

            // are transfers open?
            if Self::transfer_status() == false {
                fail!(Error::<T>::TransfersNotOpen);
            }

            // Get the balance of sender
            let new_sender_balance =
                Self::account_id_balances(&from).ok_or(Error::<T>::InsufficientFunds)?;
            let new_receiver_balance = Self::account_id_balances(&to).unwrap_or(0);

            match new_sender_balance.cmp(&amount) {
                core::cmp::Ordering::Less => fail!(Error::<T>::InsufficientFunds),
                core::cmp::Ordering::Greater => {
                    // reduce balance on sender
                    let new_sender_balance = new_sender_balance
                        .checked_sub(amount)
                        .ok_or(Error::<T>::Overflow)?;
                    // increase balance on receiver
                    let new_receiver_balance = new_receiver_balance
                        .checked_add(amount)
                        .ok_or(Error::<T>::Overflow)?;

                    AccountIdBalances::<T>::insert(&from, new_sender_balance);
                    AccountIdBalances::<T>::insert(&to, new_receiver_balance);
                    HoldersAccountIds::<T>::mutate(|holders_account_ids| {
                        holders_account_ids.insert(to)
                    });
                }
                core::cmp::Ordering::Equal => {
                    let new_receiver_balance = Self::account_id_balances(&to)
                        .unwrap_or(0)
                        .checked_add(amount)
                        .ok_or(Error::<T>::Overflow)?;
                    // balance of sender will be 0 remove from table
                    AccountIdBalances::<T>::remove(&from);
                    HoldersAccountIds::<T>::mutate(|holders_account_ids| {
                        holders_account_ids.remove(&from)
                    });
                    // increase balance on receiver
                    AccountIdBalances::<T>::insert(&to, new_receiver_balance);
                    HoldersAccountIds::<T>::mutate(|holders_account_ids| {
                        holders_account_ids.insert(to)
                    });
                }
            }

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Checks if the account parameter is the controller.
        fn is_controller(account: &T::AccountId) -> bool {
            Self::controller()
                .map(|controller| account == &controller)
                .unwrap_or(false)
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// You cannot change a controller to the same controller.
        SameController,
        /// You are not the controller.
        NotController,
        /// Cannot open transfers when controller not set.
        ControllerNotSet,
        /// Cannot mint whilst transfers open.
        CannotMintCoins,
        /// Minting Overflowed.
        Overflow,
        /// Insufficient funds to rebalance.
        InsufficientFunds,
        /// Transfers not open.
        TransfersNotOpen,
    }

    #[pallet::event]
    pub enum Event<T: Config> {
        SuccessMessage(T::AccountId),
    }

    impl<T: Config> Pallet<T> {
        /// Checks if all the setup actions have been done.
        fn check_setup() -> bool {
            Controller::<T>::exists()
        }
    }
}
