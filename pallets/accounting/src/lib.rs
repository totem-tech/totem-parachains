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

//! The main Totem Global Accounting Ledger
//!
//! # Totem Live Accounting Primitives
//!
//! * All entities operating on the Totem Live Accounting network have KAPEX as the Functional Currency. This cannot be changed.
//! * All accounting is carried out on Accrual basis.
//! * Accounting periods will close every block, although entities are free to choose a specific block for longer periods (month/year close is a nominated block number)
//! * Periods are defined by block number ranges
//! * In order to facilitate expense recognistion for example the period in which the transaction is recorded, may not necessrily be the period in which the
//! transaction is recognised) adjustments must specify the period(block number or block range) to which they relate. By default the transaction block number and the period block number are identical on first posting.
//!
//! # Curency Types
//!
//! The UI provides spot rate for live results for Period close reporting (also known as Reporting Currency or Presentation Currency), which is supported byt the exchange rates module.
//! General rules for Currency conversion at Period Close follow IFRS rules and are carried out as follows:
//! * Revenue recognition in the period when they occur,
//! * Expenses recognised (including asset consumption) in the same period as the revenue to which they relate.
//! * All other expenses are recognised in the period in which they occur.
//! * Therefore the currency conversion for revenue and related expenses is calculated at the spot rate for the period (block number) in which they are recognised.
//! * All other currency conversions are made at the rate for the period close. The UI can therefore present the correct conversions for any given value at any point in time.

#![cfg_attr(not(feature = "std"), no_std)]

#![cfg_attr(not(feature = "std"), no_std)]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{
        fail,
        pallet_prelude::*,
        traits::{
            Currency,
            StorageVersion,
            Randomness,
        },
        dispatch::{
            DispatchResult,
            DispatchResultWithPostInfo,
        },
        sp_runtime::traits::{
            Convert,
            Hash,
            Zero,
            BadOrigin,
            CheckedAdd,
            // CheckedSub,
        },
    };
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    use totem_common::TryConvert;

    use totem_primitives::{
        accounting::*,
        LedgerBalance,
        PostingIndex,
    };

    type CurrencyBalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// The posting index is used to identify a group of related accounting entries
    /// It provides unicity when used in combination with AccountId and Ledger for obtaining a detailed accounting entry
    /// It is also a counter for the number of accounting entries made in the entire system
    #[pallet::storage]
    #[pallet::getter(fn posting_number)]
    pub type PostingNumber<T: Config> = StorageValue<
    _,
    PostingIndex,
    ValueQuery
    >;

    /// Accounting Balances for each account by the ledgers that it uses.
    /// Keys: AccountId, Ledger
    /// Will select all ledgers and balances used by an AccountId when
    /// only the AccountId is used to query storage
    #[pallet::storage]
    #[pallet::getter(fn balance_by_ledger)]
    pub type BalanceByLedger<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, T::AccountId,
    Blake2_128Concat, Ledger,
    LedgerBalance,
    >;

    /// Detail of the accounting entry.
    /// Keys: AccountId, Ledger, Posting Index
    /// Will select all accounting entries for a given AccountId and Ledger combination when
    /// only these keys are used to query storage
    #[pallet::storage]
    #[pallet::getter(fn posting_detail)]
    pub type PostingDetail<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, (T::AccountId, Ledger),
    Blake2_128Concat, PostingIndex,
    Detail<T::AccountId, T::Hash, T::BlockNumber>,
    >;

    /// Yay! Totem!
    /// A global ledger providing a global overview of the entire state of the accounting
    /// across all ledgers. It does not provide the detail of the posting, just the balances.
    #[pallet::storage]
    #[pallet::getter(fn global_ledger)]
    pub type GlobalLedger<T: Config> = StorageMap<
    _,
    Blake2_128Concat, Ledger,
    LedgerBalance,
    ValueQuery,
    >;

    /// Accounting Reference [Date]
    /// This is the point in time from which the accounting periods are calculated.
    /// It is applicable across all ledgers for the identity and can only be set. It cannot be changed once set.
    /// It is usually some point in the future, and from then onwards twelve months will be counted for the fiscal year.
    /// However it can also be used for monthly period close activities as well as quarterly closes and so on.
    /// The calculations for FE engineers should be :
    /// 1 Month = 30 days = 216_000 blocks
    /// 1 Quarter = 3 months = 657_000 blocks
    /// 1 Half Year = 6 months = 1_314_000 blocks
    /// 1 Year = 12 months = 2_628_000 blocks
    #[pallet::storage]
    #[pallet::getter(fn accounting_ref_date)]
    pub type AccountingRefDate<T: Config> = StorageMap<
    _,
    Blake2_128Concat, T::AccountId,
    T::BlockNumber,
    OptionQuery,
    >;

    /// Opening Balance
    /// When taking over a legacy accounting system opening balances need to be added.
    /// This is not a mandatory requirement for new accounting, but conditions must be met.
    /// 1) Opening Balance is dependent on the accounting reference date being completed.
    /// 2) If a balance already exists for any ledger associated with the account,
    ///    we must use the earliest block number of all ledgers as the opening balance date.
    /// 3) Opening balance cannot be in the future by definition
    /// 4) If no balances exists in any ledger then the earliest it can be is the current block.
    /// 5) Some ledgers may have a zero opening balance. For this reason this storage simply indicates if
    ///    an opening balance has been specifically set. It also prevents it being set twice.
    #[pallet::storage]
    #[pallet::getter(fn opening_balance)]
    pub type OpeningBalance<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, T::AccountId,
    Blake2_128Concat, Ledger,
    bool,
    >;

    /// Opening Balance Date
    /// This is the block number at which the opening balance was set.
    /// It is global and not dependent on the Ledger account.
    #[pallet::storage]
    #[pallet::getter(fn opening_balance_date)]
    pub type OpeningBalanceDate<T: Config> = StorageMap<
    _,
    Blake2_128Concat, T::AccountId,
    T::BlockNumber,
    ValueQuery,
    >;

    // The genesis config type.
    // The Balances here should be exactly the same as configured in the Balances Pallet to set the opening balances correctly
    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub opening_balances: Vec<(T::AccountId, LedgerBalance)>,
    }

    // The default value for the genesis config type.
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { opening_balances: Default::default() }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {

            let input = *b"TotemsOpeningBalancesGenesisHash";
            let reference_hash: T::Hash = T::Hashing::hash(input.encode().as_slice());
            let block_number: T::BlockNumber = 0u32.into();
            let posting_index: PostingIndex = 0;
            // Reserves is a Debit Balance Account
            let mint_to: Ledger = Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance)));
            // Reserves is a Credit Balance Account
            let account_for: Ledger = Ledger::BalanceSheet(B::Equity(E::NetworkReserves));
            let total = self.opening_balances.iter().fold(Zero::zero(), |account_balance: LedgerBalance, &(_, n)| account_balance + n);

            <PostingNumber<T>>::put(&posting_index);
            <GlobalLedger::<T>>::insert(&mint_to, total.clone());
            <GlobalLedger::<T>>::insert(&account_for, total);

            for (address, balance) in &self.opening_balances {
                let account_balance_key_debit = (address.clone(), mint_to.clone());
                let account_balance_key_credit = (address.clone(), account_for.clone());

                let detail_debit = Detail {
                    counterparty: address.clone(),
                    amount: balance.clone(),
                    debit_credit: Indicator::Debit,
                    reference_hash: reference_hash,
                    changed_on_blocknumber: block_number,
                    applicable_period_blocknumber: block_number,
                };

                let detail_credit = Detail {
                    counterparty: address.clone(),
                    amount: balance.clone(),
                    debit_credit: Indicator::Credit,
                    reference_hash: reference_hash,
                    changed_on_blocknumber: block_number,
                    applicable_period_blocknumber: block_number,
                };

                <BalanceByLedger::<T>>::insert(&address, &mint_to, balance.clone());
                <BalanceByLedger::<T>>::insert(&address, &account_for, balance);
                <PostingDetail::<T>>::insert(&account_balance_key_debit, &posting_index, detail_debit);
                <PostingDetail::<T>>::insert(&account_balance_key_credit, &posting_index, detail_credit);
            }
        }
    }

    #[pallet::config]
    // pub trait Config: frame_system::Config + pallet_timestamp::Config {
    pub trait Config: frame_system::Config {
        // type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type AccountingConverter: TryConvert<CurrencyBalanceOf<Self>, LedgerBalance>
        + Convert<[u8; 32], Self::AccountId>
        + Convert<CurrencyBalanceOf<Self>, LedgerBalance>
        + Convert<u32, Self::BlockNumber>;
        type Currency: Currency<Self::AccountId>;
        type RandomThing: Randomness<Self::Hash, Self::BlockNumber>;
        type Acc: Posting<
            Self::AccountId,
            Self::Hash,
            Self::BlockNumber,
            CurrencyBalanceOf<Self>,
        >;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Posting index overflowed.
        PostingIndexOverflow,
        /// Balance Value overflowed.
        BalanceValueOverflow,
        /// System failure in Account Posting.
        SystemFailure,
        /// Overflow error, amount too big.
        AmountOverflow,
        /// The accounting reference has not yet been set
        AccountingRefDateNotSet,
        /// The accounting reference date has already been set
        AccountingRefDateAlreadySet,
        /// Minimum of 62 days until first possible accounting reference date
        AccountingRefDateTooSoon,
        /// Maximum of 2 years until first accounting reference date
        AccountingRefDateTooLate,
        /// The Accounting Equation is not balanced
        AccountingEquationError,
        /// Cannot set the opening balance for this ledger
        InvalidOpeningLedgerPL,
        /// Cannot set the opening balance for this ledger
        InvalidOpeningLedgerCtrl,
        /// Debits and credits do not balance
        DebitCreditMismatch,
        /// Too many accounting entries
        TooManyOpeningEntries,
        /// The opening balance has already been set
        OpeningBalanceAlreadySet,
        /// The submitted details are not valid and cannot be adjusted
        AdjustmentNotPossible,
        /// You can only make adjustments to existing records
        IndexNotFound,
        /// You cannot make adjustments in the future
        ApplicablePeriodNotValid,
        /// You cannot make adjustments to these ledgers
        IllegalAdjustment,
		/// No Earliest block number
		NoEarliestBlockNumber
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// This is the point in time from which the accounting periods are calculated.
        /// It is applicable across all ledgers for the identity and can only be set. It cannot be changed once set.
        /// It is usually some point in the future, and from then onwards twelve months will be counted for the fiscal year.
        #[pallet::call_index(0)]
        #[pallet::weight(0/*TODO*/)]
        pub fn set_accounting_ref_date(
            origin: OriginFor<T>,
            block_number: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            // check that the block number is not already set
            ensure!(<AccountingRefDate<T>>::get(&who).is_none(), Error::<T>::AccountingRefDateAlreadySet);

            let current_block = frame_system::Pallet::<T>::block_number();
            let minimum_reference_date = current_block.clone() + 446_400u32.into();
            let maximum_block_number = current_block + 5_256_000u32.into();

            // // check that the block number is in the future. This also confirms that it is not zero
            // // There should be a minimum of 62 Days in the future from the current block (446,400).
            // // This allows for the current month or anoy month in progress
            ensure!(block_number > minimum_reference_date, Error::<T>::AccountingRefDateTooSoon);

            // // check that the block number is not too far in the future.
            // // Maximum period is two years from now (5,256,000)
            ensure!(block_number < maximum_block_number, Error::<T>::AccountingRefDateTooLate);

            // // set the block number
            <AccountingRefDate<T>>::insert(who.clone(), block_number.clone());

            // // emit event
            Self::deposit_event(Event::<T>::AccountingRefDateSet { who, at_blocknumber: block_number });

            Ok(().into())
        }

        /// The opening balances can be set for each ledger in the Balance Sheet.
        /// A date for the opening balance needs to be set however and all accounting will conform to this date.
        /// As opening balances can be added after accounting entries have been made, we should take the earliest possible entry in all the ledgers as the opening balance date.
        /// If no previous entries have been made then we assume that the opening balance date is that supplied by the extrinsic.
        /// This extrinsic does not perform check against the validity of the debit and credit status for the entire ledger, only the incoming entries.
        /// The input Vec should be bounded to 166 entries which is the current size of the Balance Sheet
        #[pallet::call_index(1)]
        #[pallet::weight(0/*TODO*/)]
        pub fn set_opening_balance(
            origin: OriginFor<T>,
            entries: Vec<AdjustmentDetail<CurrencyBalanceOf<T>>>,
            block_number: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            // Check that the number of entries in the entries array is not greater than 166
            // TODO This should be a parameter in the runtime and a BoundedVec
            if entries.len() > 166 {
                return Err(Error::<T>::TooManyOpeningEntries.into())
            }

            // you should not be allowed to set an opening balance until the accounting reference date has been set
            ensure!(!<AccountingRefDate<T>>::get(&who).is_none(), Error::<T>::AccountingRefDateNotSet);

            // Check that the entries conform to the accounting equation, and that all debits equal all credits. Also performs a (redundant?) check that the ledger does not already have an opening balance.
            Self::combined_sanity_checks(&who, &entries)?;

            // If entries exist in any ledger in the PostingDetails storage, then the earliest blocknumber is taken from there and used, in preference to the one in the arguments to this extrinsic.
            // This is to ensure that the accounting reference date is not set before the earliest entry in the ledger.
            // the default value however is the block number in the arguments to this extrinsic.
            let earliest_block_number = <BalanceByLedger<T>>::iter_prefix(&who)
                .flat_map(|(ledger, _)| {
                    <PostingDetail<T>>::iter_prefix_values((who.clone(), ledger))
                    .map(|d| d.applicable_period_blocknumber.min(block_number))
                    .min()
                    .into_iter()
                })
                .min();

			if earliest_block_number.is_none() {
				return Err(Error::<T>::NoEarliestBlockNumber.into());
			}

			let earliest_block_number = earliest_block_number.unwrap();

            // Since all checks have passed, update the storage with the new opening balance entries. This requires building a vec of entries record and sending to multiposting, but the status is also updated here for optimisation.
            let reference_hash = T::Acc::get_pseudo_random_hash(who.clone(), who.clone());
            let current_block = frame_system::Pallet::<T>::block_number();
            let mut keys = Vec::new();
            entries
                .iter()
                .for_each(|e| {
                    // Implement a check on the balance type for each ledger and adjust the posting amount to be negative or positive accordingly. No need to consider if the entered amount is negative because it is a primitive u128 and therefore will elilminate errors.
                    let posting_amount = Self::increase_or_decrease(&e.ledger, e.amount, &e.debit_credit);

                    let record = Record {
                        primary_party: who.clone(),
                        counterparty: who.clone(),
                        ledger: e.ledger,
                        amount: posting_amount,
                        debit_credit: e.debit_credit,
                        reference_hash: reference_hash.clone(),
                        changed_on_blocknumber: current_block.clone(),
                        applicable_period_blocknumber: earliest_block_number.clone(),
                };
                // set the opening balance status
                <OpeningBalance<T>>::insert(who.clone(), e.ledger, true);
                keys.push(record);
            });

            T::Acc::handle_multiposting_amounts(&keys, None)?;

            <OpeningBalanceDate<T>>::insert(who.clone(), earliest_block_number);

            // emit event
            Self::deposit_event(Event::<T>::OpeningBalanceSet);

            Ok(().into())
        }

        /// This function is intended for advanced book-keepers and accountants.
        /// It does not check the logical corrrectness of the entries but allows a limited number entries to be made.
        /// It is used to make minor adjustments but cannot be used to make standalone journal postings as it requires an existing posting index.
        /// There is a check for debit and credit correctness and the debits and credits are checked against the account type to determine increase or decrease in values.
        /// It is not intended for monetary movements so entries relating to those ledgers will be prevented.
        /// The number of entries should be bounded to 10 as it is not expected that a large number of corrections should be made at once.
        #[pallet::call_index(2)]
        #[pallet::weight(0/*TODO*/)]
        pub fn adjustment(
            origin: OriginFor<T>,
            adjustments: Vec<AdjustmentDetail<CurrencyBalanceOf<T>>>,
            index: PostingIndex,
            applicable_period: T::BlockNumber,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            // Check that the number of adjustments in the adjustments array is not greater than 10
            // TODO This should be a parameter in the runtime and a BoundedVec
            if adjustments.len() > 10 {
                return Err(Error::<T>::TooManyOpeningEntries.into())
            }

            // Check that the index is not a zero u128 value
            ensure!(index != 0u128, Error::<T>::IndexNotFound);

            // check that the index is less than or equal to the current Posting Index Number
            ensure!(index <= <PostingNumber<T>>::get(), Error::<T>::IndexNotFound);

            let current_block = frame_system::Pallet::<T>::block_number();
            // adjustment period must be at least 7200 blocks in the past (1 day)
            let threshold_block = current_block.clone() - T::AccountingConverter::convert(7200u32);
            ensure!(applicable_period <= threshold_block, Error::<T>::ApplicablePeriodNotValid);

            // check that the debits match the credits
            Self::debit_credit_matches(&adjustments)?;

            // Find at least one instance where the ledger and the index match for the account.
            // this confirms that this adjustment is plausible.
            // Also reject any adjutment to internal native coin accounting.
            let mut matched_index = false;
            for i in 0..adjustments.len() {
                let adjustment = &adjustments[i];
                // checks to reject illegal native coin adjustments
                if adjustment.ledger ==
                Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance)))
                || adjustment.ledger ==
                Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalReservedBalance)))
                || adjustment.ledger ==
                Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::Admin(AdminCosts::Blockchain(InternalAccounting::NetworkTransactionFees)))))
                || adjustment.ledger ==
                Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::TaxFinesPenalties(TFP::SlashedCoins))))
                || adjustment.ledger ==
                Ledger::ProfitLoss(P::Income(I::OtherOperatingIncome(OOPIN::BlockchainSlashedFundsIncome)))
                || adjustment.ledger ==
                Ledger::BalanceSheet(B::Equity(E::NetworkReserves)) {
                    return Err(Error::<T>::IllegalAdjustment.into())
                }
                // try to find a match for the ledger and the index
                if <PostingDetail<T>>::contains_key((&who, adjustment.ledger), index) {
                    matched_index = true;
                }
            }
            // ensure matched_index is true
            ensure!(matched_index, Error::<T>::IndexNotFound);

            // At this point we passed all the checks and can proceed with the adjustment
            let reference_hash = T::Acc::get_pseudo_random_hash(who.clone(), who.clone());
            let mut keys = Vec::new();
            adjustments
                .iter()
                .for_each(|a| {
                    let posting_amount = Self::increase_or_decrease(
                        &a.ledger,
                        a.amount,
                        &a.debit_credit
                    );
                    let record = Record {
                        primary_party: who.clone(),
                        counterparty: who.clone(),
                        ledger: a.ledger,
                        amount: posting_amount,
                        debit_credit: a.debit_credit,
                        reference_hash: reference_hash.clone(),
                        changed_on_blocknumber: current_block.clone(),
                        applicable_period_blocknumber: applicable_period.clone(),
                    };
                    keys.push(record);
                });

                T::Acc::handle_multiposting_amounts(&keys, Some(index))?;

            Self::deposit_event(Event::<T>::AdjustmentsPosted);

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        LegderUpdate {
            who: T::AccountId,
            ledger: Ledger,
            amount: LedgerBalance,
            index: PostingIndex,
        },
        AccountingRefDateSet {
            who: T::AccountId,
            at_blocknumber: T::BlockNumber,
        },
        OpeningBalanceSet,
        AdjustmentsPosted,
    }

    impl<T: Config> Pallet<T> {
        /// Basic posting function (warning! can cause imbalance if not called with corresponding debit or credit entries)
        /// The reason why this is a simple function is that one debit posting may or may not correspond with one or many credit
        /// postings and vice-versa. For example a debit to Accounts Receivable is the gross invoice amount, which could correspond with
        /// a credit to liabilities for the sales tax amount and a credit to revenue for the net invoice amount. The sum of both credits being
        /// equal to the single debit in accounts receivable, but only one posting needs to be made to that account, and two posting for the others.
        /// The Totem Accounting Recipes are constructed using this simple function.
        /// The applicable_period_blocknumber is for re-targeting the entry in the accounts, i.e. for adjustments prior to or after the current period (generally accruals).
        /// It is also used for reporting purposes.
        /// The changed_on_blocknumber is used to facilitate audit, but is not technically needed as a full archive node could also provide this information from the state
        /// changes apparent in every block.
        fn post_amounts(
            key: Record<T::AccountId, T::Hash, T::BlockNumber>,
            posting_index: PostingIndex,
            is_new_index: bool,
        ) -> DispatchResult {
                let balance_key = (key.primary_party.clone(), key.ledger.clone());
                // let posting_key = (key.primary_party.clone(), key.ledger.clone(), posting_index);
                let abs_amount: LedgerBalance =  key.amount.clone().abs();
                let detail = Detail {
                    counterparty: key.counterparty.clone(),
                    amount: abs_amount,
                    debit_credit: key.debit_credit,
                    reference_hash: key.reference_hash,
                    changed_on_blocknumber: key.changed_on_blocknumber,
                    applicable_period_blocknumber: key.applicable_period_blocknumber,
                };

                // !! Warning !!
                // Values could feasibly overflow, with no visibility on other accounts. In this event this function returns an error.
                // Reversals must occur in the parent function (i.e. that calls this function).
                // As all values passed to this function are already signed +/- we only need to sum to the previous balance and check for overflow
                // Updates are only made to storage once tests below are passed for debits or credits.
                match <BalanceByLedger<T>>::get(&key.primary_party, &key.ledger) {
                    None => BalanceByLedger::<T>::insert(&key.primary_party, &key.ledger, key.amount.clone()),
                    Some(b) => {
                        let new_balance = b.checked_add(key.amount).ok_or(Error::<T>::BalanceValueOverflow)?;
                        BalanceByLedger::<T>::insert(&key.primary_party, &key.ledger, new_balance);
                    },
                };

                if let Some(new_global_balance) = <GlobalLedger<T>>::get(&key.ledger)
                .checked_add(key.amount) {
                    GlobalLedger::<T>::insert(&key.ledger, new_global_balance);
                } else {
                    GlobalLedger::<T>::insert(&key.ledger, key.amount.clone());
                };

                if is_new_index {
                    PostingNumber::<T>::put(posting_index);
                }
                PostingDetail::<T>::insert(&balance_key, &posting_index, detail);

                Ok(())
            }

            /// Return a pair of:
            /// - The amount given as a parameter, but signed.
            /// - The opposite of that amount.
            fn increase_decrease_amounts(
                amount: CurrencyBalanceOf<T>,
            ) -> Result<(LedgerBalance, LedgerBalance), Error<T>> {
                let increase_amount: LedgerBalance =
                T::AccountingConverter::try_convert(amount).ok_or(Error::<T>::AmountOverflow)?;
                let decrease_amount = increase_amount
                .checked_neg()
                .ok_or(Error::<T>::AmountOverflow)?;

                Ok((increase_amount, decrease_amount))
            }

            fn increase_or_decrease(
                ledger: &Ledger,
                amount: CurrencyBalanceOf<T>,
                debit_credit: &Indicator,
            ) -> LedgerBalance {
                let converted_amount: LedgerBalance = T::AccountingConverter::convert(amount);
                // T::AccountingConverter::try_convert(amount).ok_or(Error::<T>::AmountOverflow)?;

                // check that the ledger type is_credit_balance and correct the sign on the amount for posting.
                let final_amount: LedgerBalance = match ledger.is_credit_balance() {
                    true => match debit_credit {
                        Indicator::Debit => -converted_amount,
                        Indicator::Credit => converted_amount,
                    },
                    false => match debit_credit {
                        Indicator::Debit => converted_amount,
                        Indicator::Credit => -converted_amount,
                    }
                };

                final_amount
            }

            fn debit_credit_matches(
                entries: &Vec<AdjustmentDetail<CurrencyBalanceOf<T>>>,
            ) -> DispatchResult {
                let debit_total: CurrencyBalanceOf<T> = Zero::zero();
                let credit_total: CurrencyBalanceOf<T> = Zero::zero();

                for entry in entries {
                    match entry.debit_credit {
                        Indicator::Debit => {
                            debit_total.checked_add(&entry.amount)
                                .ok_or(Error::<T>::BalanceValueOverflow)?;
                        },
                        Indicator::Credit => {
                            credit_total.checked_add(&entry.amount)
                                .ok_or(Error::<T>::BalanceValueOverflow)?;
                        },
                    }
                }

                if debit_total != credit_total {
                    return Err(Error::<T>::DebitCreditMismatch.into());
                }

                Ok(())
            }

        // fn accounting_equation_check(
        //     entries: &Vec<AdjustmentDetail<CurrencyBalanceOf<T>>>,
        // ) -> DispatchResult {
        //     let mut assets_total: LedgerBalance = 0;
        //     let mut liabilities_total: LedgerBalance = 0;
        //     let mut equity_total: LedgerBalance = 0;

        //     for entry in entries {
        //         match entry.ledger {
        //             Ledger::BalanceSheet(B::Assets(_)) => {
        //                 assets_total.checked_add(&entry.amount)
        //                     .ok_or(Error::<T>::BalanceValueOverflow)?;
        //             },
        //             Ledger::BalanceSheet(B::Liabilities(_)) => {
        //                 liabilities_total.checked_add(&entry.amount)
        //                     .ok_or(Error::<T>::BalanceValueOverflow)?;
        //             },
        //             Ledger::BalanceSheet(B::Equity(_)) => {
        //                 equity_total.checked_add(&entry.amount)
        //                     .ok_or(Error::<T>::BalanceValueOverflow)?;
        //             },
        //         }
        //     }

        //     if assets_total != liabilities_total + equity_total {
        //         return Err(Error::<T>::AccountingEquationError.into());
        //     }

        //     Ok(())
        // }

        fn combined_sanity_checks(
            who : &T::AccountId,
            entries: &Vec<AdjustmentDetail<CurrencyBalanceOf<T>>>,
        ) -> DispatchResult {
            let assets_total: CurrencyBalanceOf<T> = Zero::zero();
            let liabilities_total: CurrencyBalanceOf<T> = Zero::zero();
            let equity_total: CurrencyBalanceOf<T> = Zero::zero();
            let debit_total: CurrencyBalanceOf<T> = Zero::zero();
            let credit_total: CurrencyBalanceOf<T> = Zero::zero();

            for entry in entries {
                // This should fail if _any_ ledger has an opening balance already set
                // this should not happen as the check is performed before this is called
                ensure!(<OpeningBalance<T>>::get(&who, &entry.ledger).is_none(), Error::<T>::OpeningBalanceAlreadySet);
                match entry.ledger {
                    Ledger::BalanceSheet(B::Assets(_)) => {
                        assets_total.checked_add(&entry.amount)
                        .ok_or(Error::<T>::BalanceValueOverflow)?;
                    },
                    Ledger::BalanceSheet(B::Liabilities(_)) => {
                        liabilities_total.checked_add(&entry.amount)
                        .ok_or(Error::<T>::BalanceValueOverflow)?;
                    },
                    Ledger::BalanceSheet(B::Equity(_)) => {
                        equity_total.checked_add(&entry.amount)
                        .ok_or(Error::<T>::BalanceValueOverflow)?;
                    },
                    Ledger::ProfitLoss(_) => {
                        // Profit and Loss accounts are not valid for opening balances
                        return Err(Error::<T>::InvalidOpeningLedgerPL.into());
                    },
                    Ledger::ControlAccounts(_) => {
                        // Control accounts are not valid for opening balances
                        return Err(Error::<T>::InvalidOpeningLedgerCtrl.into());
                    },
                }
                match entry.debit_credit {
                    Indicator::Debit => {
                        debit_total.checked_add(&entry.amount)
                        .ok_or(Error::<T>::BalanceValueOverflow)?;
                    },
                    Indicator::Credit => {
                        credit_total.checked_add(&entry.amount)
                        .ok_or(Error::<T>::BalanceValueOverflow)?;
                    },
                }
            }

            if assets_total != liabilities_total + equity_total {
                return Err(Error::<T>::AccountingEquationError.into());
            }

            if debit_total != credit_total {
                return Err(Error::<T>::DebitCreditMismatch.into());
            }

            Ok(())
        }
    }

    impl<T: Config> Posting<T::AccountId, T::Hash, T::BlockNumber, CurrencyBalanceOf<T>> for Pallet<T>
    where
        T: pallet_timestamp::Config,
    {
        type PostingIndex = PostingIndex;

        /// The Totem Accounting Recipes are constructed using this function which handles posting to multiple accounts.
        /// It is exposed to other modules as a trait
        /// If for whatever reason an error occurs during the storage processing which is sequential
        /// this function also handles reversing out the prior accounting entries
        /// Therefore the recipes that are passed as arguments need to be be accompanied with a reversal
        /// Obviously the last posting does not need a reversal for if it errors, then it was not posted in the first place.
        fn handle_multiposting_amounts(
            keys: &[Record<T::AccountId, T::Hash, T::BlockNumber>],
            index: Option<PostingIndex>,
        ) -> DispatchResult {
            let mut posting_index: PostingIndex = 1;
            let mut new_index: bool = true;
            // check that the PostingIndex has been supplied
            if index.is_none() {
                // Set initial value for posting index
                // Only need to increment, if it exists, else this the the very first record (with value 1).
                if <PostingNumber<T>>::exists() {
                    posting_index = match Self::posting_number().checked_add(1) {
                        Some(i) => i,
                        None => {
                            fail!(Error::<T>::PostingIndexOverflow)
                        },
                    }
                };
            } else {
                // the posting index has been supplied externally, therefor this is likely to be an adjustment of some kind, and we need to keep the original posting index.
                posting_index = index.unwrap();
                new_index = false;
            }

            // Iterate over forward keys. If error, then reverse out prior postings.
            for (idx, key) in keys.iter().cloned().enumerate() {
                if let Err(e) = Self::post_amounts(key, posting_index, new_index) {
                    // (Un)likely error before the value was updated.
                    // Need to reverse-out the earlier postings to storage just in case
                    // it has changed in storage already
                    if idx > 1 {
                        for key in keys.iter().cloned().take(idx - 1) {
                            let reversed = Record {
                                // Reversal should never cause an overflow - check nevertheless
                                amount: key.amount.checked_neg().ok_or(Error::<T>::AmountOverflow)?,
                                debit_credit: key.debit_credit.reverse(),
                                ..key
                            };
                            // If this fails it would be serious. This is not expected to fail.
                            Self::post_amounts(reversed, posting_index, new_index)
                            .or(Err(Error::<T>::SystemFailure))?;
                        }
                    }
                    fail!(e)
                }
            }

            Ok(())
        }

        /// This function simply returns the Totem escrow account address
        fn get_escrow_account() -> T::AccountId {
            let escrow_account: [u8; 32] = *b"TotemsEscrowAddress4LockingFunds";

            T::AccountingConverter::convert(escrow_account)
        }

        /// This function simply returns the Totem network fees account address
        fn get_netfees_account() -> T::AccountId {
            let netfees_account: [u8; 32] = *b"TotemAccountingNetworkFeeAddress";

            T::AccountingConverter::convert(netfees_account)
        }

        /// Adds a new accounting entry in the ledger in case of a transfer
        /// Reduce the Internal balance, and reduce the equity from the sender with the reverse for the receiver
        fn account_for_simple_transfer(
            from: T::AccountId,
            to: T::AccountId,
            amount: CurrencyBalanceOf<T>,
        ) -> DispatchResult {
            let reference_hash = Self::get_pseudo_random_hash(from.clone(), to.clone());
            let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
            let current_block_dupe = current_block; // Applicable period for accounting
            let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(amount)?;
            let keys = [
            Record {
                primary_party: from.clone(),
                counterparty: to.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
                amount: decrease_amount,
                debit_credit: Indicator::Credit,
                reference_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: from.clone(),
                counterparty: to.clone(),
                ledger: Ledger::BalanceSheet(B::Equity(E::NetworkReserves)),
                amount: decrease_amount,
                debit_credit: Indicator::Debit,
                reference_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: to.clone(),
                counterparty: from.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
                amount: increase_amount,
                debit_credit: Indicator::Debit,
                reference_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: to.clone(),
                counterparty: from.clone(),
                ledger: Ledger::BalanceSheet(B::Equity(E::NetworkReserves)),
                amount: increase_amount,
                debit_credit: Indicator::Credit,
                reference_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            ];

            Self::handle_multiposting_amounts(&keys, None)?;

            Ok(())
        }

        /// This function takes the transaction fee and prepares to account for it in accounting.
        /// This is one of the few functions that will set the ledger accounts to be updated here. Fees
        /// are native to the Substrate Framework, and there may be other use cases.
        fn account_for_fees(
            fee: CurrencyBalanceOf<T>,
            payer: T::AccountId,
        ) -> DispatchResult {
            // Take the fee amount and convert for use with accounting. Fee is of type T::Balance which is u128.
            // As amount will always be positive, convert for use in accounting
            let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(fee)?;
            // This sets the change block and the applicable posting period. For this context they will always be
            // the same.
            let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
            let current_block_dupe = current_block; // Applicable period for accounting

            // Generate dummy Hash reference (it has no real bearing but allows posting to happen)
            let fee_hash: T::Hash = Self::get_pseudo_random_hash(payer.clone(), payer.clone());

            // Get the dummy address for fees. Note this does not identify the receipients of fees (validators)
            // It is used just for generic self-referential accounting
            let netfee_address: T::AccountId = Self::get_netfees_account();

            let keys = [
            Record {
                primary_party: payer.clone(),
                counterparty: netfee_address.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
                amount: decrease_amount,
                debit_credit: Indicator::Credit,
                reference_hash: fee_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: payer.clone(),
                counterparty: netfee_address.clone(),
                ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::Admin(AdminCosts::Blockchain(InternalAccounting::NetworkTransactionFees))))),
                amount: increase_amount,
                debit_credit: Indicator::Debit,
                reference_hash: fee_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            ];

            Self::handle_multiposting_amounts(&keys, None)?;

            Ok(())
        }

        /// This function takes an amount to be reserved for the user and prepares to account for it.
        /// It is called from totem balances pallet after checks, and should not require further balance checks
        fn set_reserve_amount(
            beneficiary: T::AccountId,
            amount: CurrencyBalanceOf<T>,
        ) -> DispatchResult {
            // Take the amount and convert for use with accounting. Amount is of type T::Balance which is u128.
            // As amount will always be positive, convert for use in accounting
            let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(amount)?;
            // This sets the change block and the applicable posting period. For this context they will always be
            // the same.
            let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
            let current_block_dupe = current_block; // Applicable period for accounting

            // Generate dummy Hash reference (it has no real bearing but allows posting to happen)
            let ref_hash: T::Hash = Self::get_pseudo_random_hash(beneficiary.clone(), beneficiary.clone());

            let keys = [
            Record {
                primary_party: beneficiary.clone(),
                counterparty: beneficiary.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
                amount: decrease_amount,
                debit_credit: Indicator::Credit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: beneficiary.clone(),
                counterparty: beneficiary.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalReservedBalance))),
                amount: increase_amount,
                debit_credit: Indicator::Debit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            ];

            Self::handle_multiposting_amounts(&keys, None)?;

            Ok(())
        }

        /// This function takes an amount to be reserved for the user and prepares to account for it.
        /// It is called from totem balances pallet after checks, and should not require further balance checks
        fn unreserve_amount(
            beneficiary: T::AccountId,
            amount: CurrencyBalanceOf<T>,
        ) -> DispatchResult {
            // Take the amount and convert for use with accounting. Amount is of type T::Balance which is u128.
            // As amount will always be positive, convert for use in accounting
            let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(amount)?;
            // This sets the change block and the applicable posting period. For this context they will always be
            // the same.
            let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
            let current_block_dupe = current_block; // Applicable period for accounting

            // Generate dummy Hash reference (it has no real bearing in this use case, but allows posting to happen)
            let ref_hash: T::Hash = Self::get_pseudo_random_hash(beneficiary.clone(), beneficiary.clone());

            let keys = [
            Record {
                primary_party: beneficiary.clone(),
                counterparty: beneficiary.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalReservedBalance))),
                amount: decrease_amount,
                debit_credit: Indicator::Credit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: beneficiary.clone(),
                counterparty: beneficiary.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
                amount: increase_amount,
                debit_credit: Indicator::Debit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            ];

            Self::handle_multiposting_amounts(&keys, None)?;

            Ok(())
        }

        /// This function takes slashes the reserved amount. It causes the coin supply to be reduced, which is handled
        /// by the balances pallet, however the accounting must book this direct to expenses.
        /// It is called from totem balances pallet after checks, and should not require further balance checks
        fn slash_reserve(
            beneficiary: T::AccountId,
            amount: CurrencyBalanceOf<T>,
        ) -> DispatchResult {
            // Take the amount and convert for use with accounting. Amount is of type T::Balance which is u128.
            // As amount will always be positive, convert for use in accounting
            let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(amount)?;
            // This sets the change block and the applicable posting period. For this context they will always be
            // the same.
            let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
            let current_block_dupe = current_block; // Applicable period for accounting

            // Generate dummy Hash reference (it has no real bearing in this use case, but allows posting to happen)
            let ref_hash: T::Hash = Self::get_pseudo_random_hash(beneficiary.clone(), beneficiary.clone());

            let keys = [
            Record {
                primary_party: beneficiary.clone(),
                counterparty: beneficiary.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalReservedBalance))),
                amount: decrease_amount,
                debit_credit: Indicator::Credit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: beneficiary.clone(),
                counterparty: beneficiary.clone(),
                ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::TaxFinesPenalties(TFP::SlashedCoins)))),
                amount: increase_amount,
                debit_credit: Indicator::Debit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            ];

            Self::handle_multiposting_amounts(&keys, None)?;

            Ok(())
        }

        /// This function reassigns a reserve amount to another beneficiary. Usually used in the context of slashing.
        /// It is called from the balances pallet and all previous checks on balances have already been performed before this is called.
        fn reassign_reserve(
            slashed: T::AccountId,
            beneficiary: T::AccountId,
            amount: CurrencyBalanceOf<T>,
            is_free_balance: bool,
        ) -> DispatchResult {
            // Take the amount and convert for use with accounting. Amount is of type T::Balance which is u128.
            // As amount will always be positive, convert for use in accounting
            let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(amount)?;
            // This sets the change block and the applicable posting period. For this context they will always be
            // the same.
            let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
            let current_block_dupe = current_block; // Applicable period for accounting

            // Generate dummy Hash reference (it has no real bearing in this use case, but allows posting to happen)
            let ref_hash: T::Hash = Self::get_pseudo_random_hash(slashed.clone(), beneficiary.clone());

            // Select the account ledger to update
            let beneficiary_ledger = match is_free_balance {
                // the funds will be moved to the free balance of the beneficiary
                true => Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
                // the funds will be moved to the reserved balance of the beneficiary
                false => Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalReservedBalance))),
            };

            // First handle the slashing on the slashed account, then handle the addition of the funds to the new account
            let keys = [
            Record {
                primary_party: slashed.clone(),
                counterparty: slashed.clone(),
                ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalReservedBalance))),
                amount: decrease_amount,
                debit_credit: Indicator::Credit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: slashed.clone(),
                counterparty: slashed.clone(),
                ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::TaxFinesPenalties(TFP::SlashedCoins)))),
                amount: increase_amount,
                debit_credit: Indicator::Debit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: beneficiary.clone(),
                counterparty: slashed.clone(),
                ledger: beneficiary_ledger,
                amount: increase_amount,
                debit_credit: Indicator::Debit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            Record {
                primary_party: beneficiary.clone(),
                counterparty: slashed.clone(),
                ledger: Ledger::ProfitLoss(P::Income(I::OtherOperatingIncome(OOPIN::BlockchainSlashedFundsIncome))),
                amount: increase_amount,
                debit_credit: Indicator::Credit,
                reference_hash: ref_hash,
                changed_on_blocknumber: current_block,
                applicable_period_blocknumber: current_block_dupe,
            },
            ];

            Self::handle_multiposting_amounts(&keys, None)?;

            Ok(())
        }

        // SUSPENDED until the ASSET_TX_PAYMENT Pallet introduced.
        // /// This function handles burnt fee amounts when the fee rewards distribution fails.
        // /// Related to the asset_tx_payment pallet HandleCredit
        // fn account_for_burnt_fees(
        //     fee: CurrencyBalanceOf<T>,
        //     loser: T::AccountId,
        // ) -> DispatchResult {
        //     let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(fee)?;
        //     let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
        //     let current_block_dupe = current_block; // Applicable period for accounting

        //     let fee_hash: T::Hash = Self::get_pseudo_random_hash(loser.clone(), loser.clone());

        //     let netfee_address: T::AccountId = Self::get_netfees_account();

        //     // this is a single adjustment on the network fees account keeping the current balance correct,
        //     // but also indicating
        //     let keys = [
        //         Record {
        //             primary_party: netfee_address.clone(),
        //             counterparty: loser.clone(),
        //             ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
        //             amount: decrease_amount,
        //             debit_credit: Indicator::Credit,
        //             reference_hash: fee_hash,
        //             changed_on_blocknumber: current_block,
        //             applicable_period_blocknumber: current_block_dupe,
        //         },
        //         Record {
        //             primary_party: netfee_address.clone(),
        //             counterparty: loser.clone(),
        //             ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::CostOfGoodsSold(COGS::CryptoBurnWriteDown)))),
        //             amount: increase_amount,
        //             debit_credit: Indicator::Debit,
        //             reference_hash: fee_hash,
        //             changed_on_blocknumber: current_block,
        //             applicable_period_blocknumber: current_block_dupe,
        //         },
        //     ];

        //     Self::handle_multiposting_amounts(&keys, None)?;

        //     Ok(())
        // }

        // /// This function takes is used to payout validators and account for their gains.
        // /// Related to the asset_tx_payment pallet
        // fn distribute_fees_rewards(
        //     fee: CurrencyBalanceOf<T>,
        //     author: T::AccountId,
        // ) -> DispatchResult {
        //     let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(fee)?;
        //     let current_block = frame_system::Pallet::<T>::block_number(); // For audit on change
        //     let current_block_dupe = current_block; // Applicable period for accounting

        //     let fee_hash: T::Hash = Self::get_pseudo_random_hash(author.clone(), author.clone());

        //     let netfee_address: T::AccountId = Self::get_netfees_account();

        //     // This handles the payout to the block author
        //     let keys = [
        //         Record {
        //             primary_party: netfee_address.clone(),
        //             counterparty: author.clone(),
        //             ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
        //             amount: decrease_amount,
        //             debit_credit: Indicator::Credit,
        //             reference_hash: fee_hash,
        //             changed_on_blocknumber: current_block,
        //             applicable_period_blocknumber: current_block_dupe,
        //         },
        //         Record {
        //             primary_party: netfee_address.clone(),
        //             counterparty: author.clone(),
        //             ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::Admin(AdminCosts::Blockchain(InternalAccounting::NetworkValidationReward))))),
        //             amount: increase_amount,
        //             debit_credit: Indicator::Debit,
        //             reference_hash: fee_hash,
        //             changed_on_blocknumber: current_block,
        //             applicable_period_blocknumber: current_block_dupe,
        //         },
        //         Record {
        //             primary_party: author.clone(),
        //             counterparty: netfee_address.clone(),
        //             ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
        //             amount: increase_amount,
        //             debit_credit: Indicator::Debit,
        //             reference_hash: fee_hash,
        //             changed_on_blocknumber: current_block,
        //             applicable_period_blocknumber: current_block_dupe,
        //         },
        //         Record {
        //             primary_party: author.clone(),
        //             counterparty: netfee_address.clone(),
        //             ledger: Ledger::ProfitLoss(P::Income(I::Sales(Sales::Blockchain(InternalIncome::NetworkValidationIncome)))),
        //             amount: increase_amount,
        //             debit_credit: Indicator::Credit,
        //             reference_hash: fee_hash,
        //             changed_on_blocknumber: current_block,
        //             applicable_period_blocknumber: current_block_dupe,
        //         },
        //     ];

        //     Self::handle_multiposting_amounts(&keys, None)?;

        //     Ok(())
        // }

        fn get_pseudo_random_hash(sender: T::AccountId, recipient: T::AccountId) -> T::Hash {
            let tuple = (sender.clone(), recipient);
            let sender_encoded = sender.encode();
            let (random_value, _) = T::RandomThing::random(&sender_encoded);
            let input = (
                tuple,
                pallet_timestamp::Pallet::<T>::get(),
                random_value,
                frame_system::Pallet::<T>::extrinsic_index(),
                frame_system::Pallet::<T>::block_number(),
            );

            T::Hashing::hash(input.encode().as_slice()) // default hash BlakeTwo256
        }
    }
}
