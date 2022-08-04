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

//pub mod benchmarking;
//pub mod mock;
//pub mod tests;

pub use pallet::*;

#[frame_support::pallet]
mod pallet {
    
    use frame_support::{
        fail,
        pallet_prelude::*,
        traits::{ Currency, StorageVersion, Randomness },
        dispatch::DispatchResult,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{Convert, Hash, Zero};
    use sp_std::prelude::*;

    use totem_common::TryConvert;
    use totem_primitives::accounting::*;
    
    use totem_primitives::{LedgerBalance, PostingIndex};

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
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type AccountingConverter: TryConvert<CurrencyBalanceOf<Self>, LedgerBalance>
            + Convert<[u8; 32], Self::AccountId>;
        type Currency: Currency<Self::AccountId>;
        type RandomThing: Randomness<Self::Hash, Self::BlockNumber>;
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
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // #[pallet::call]
    // impl<T: Config> Pallet<T> {
    //     #[pallet::weight(0/*TODO*/)]
    //     pub fn opening_balance(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
    //         todo!()
    //     }

    //     #[pallet::weight(0/*TODO*/)]
    //     pub fn adjustment(_origin: OriginFor<T>) -> DispatchResultWithPostInfo {
    //         todo!()
    //     }
    // }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        LegderUpdate(
            <T as frame_system::Config>::AccountId,
            Ledger,
            LedgerBalance,
            PostingIndex,
        ),
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

            PostingNumber::<T>::put(posting_index);
            PostingDetail::<T>::insert(&balance_key, &posting_index, detail);
            
            Self::deposit_event(Event::LegderUpdate(
                key.primary_party,
                key.ledger,
                key.amount,
                posting_index,
            ));

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
        ) -> DispatchResult {
            // Set initial value for posting index
            let mut posting_index: PostingIndex = 1;
            // Only need to increment, if it exists, else this the the very first record (with value 1).
            if <PostingNumber<T>>::exists() {
                posting_index = match Self::posting_number().checked_add(1) {
                    Some(i) => i,
                    None => {
                        fail!(Error::<T>::PostingIndexOverflow)
                    },
                }
            };

            // Iterate over forward keys. If error, then reverse out prior postings.
            for (idx, key) in keys.iter().cloned().enumerate() {
                if let Err(e) = Self::post_amounts(key, posting_index) {
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
                            Self::post_amounts(reversed, posting_index)
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
                    ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::AdminCosts(_0030_::Blockchain(TXOUT::NetworkTransaction))))),
                    amount: increase_amount,
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
                    ledger: Ledger::ProfitLoss(P::Income(I::Sales(Sales::Blockchain(TXIN::TransactionReceipt)))), 
                    amount: increase_amount,
                    debit_credit: Indicator::Credit,
                    reference_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
            ];

            Self::handle_multiposting_amounts(&keys)?;

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
                    ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::AdminCosts(_0030_::Blockchain(TXOUT::NetworkTransactionFees))))),
                    amount: increase_amount,
                    debit_credit: Indicator::Debit,
                    reference_hash: fee_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                Record {
                    primary_party: netfee_address.clone(),
                    counterparty: payer.clone(),
                    ledger: Ledger::BalanceSheet(B::Assets(A::CurrentAssets(CurrentAssets::InternalBalance))),
                    amount: increase_amount,
                    debit_credit: Indicator::Debit,
                    reference_hash: fee_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                Record {
                    primary_party: netfee_address.clone(),
                    counterparty: payer.clone(),
                    ledger: Ledger::ProfitLoss(P::Income(I::Sales(Sales::Blockchain(TXIN::NetworkFeeIncome)))),
                    amount: increase_amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: fee_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
            ];

            Self::handle_multiposting_amounts(&keys)?;
            
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

        //     Self::handle_multiposting_amounts(&keys)?;
            
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
        //             ledger: Ledger::ProfitLoss(P::Expenses(X::OperatingExpenses(OPEX::AdminCosts(_0030_::Blockchain(TXOUT::NetworkValidationReward))))),
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
        //             ledger: Ledger::ProfitLoss(P::Income(I::Sales(Sales::Blockchain(TXIN::NetworkValidationIncome)))), 
        //             amount: increase_amount,
        //             debit_credit: Indicator::Credit,
        //             reference_hash: fee_hash,
        //             changed_on_blocknumber: current_block,
        //             applicable_period_blocknumber: current_block_dupe,
        //         },
        //     ];

        //     Self::handle_multiposting_amounts(&keys)?;

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
                // sp_io::offchain::random_seed(),
                frame_system::Pallet::<T>::extrinsic_index(),
                frame_system::Pallet::<T>::block_number(),
            );

            T::Hashing::hash(input.encode().as_slice()) // default hash BlakeTwo256
        }
    }
}