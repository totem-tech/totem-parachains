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

// Locks prefunded amounts into the runtime.
//
// This module functions as a pseudo-escrow module, holding funds for a specified period of time and or for a specific beneficiary.
// In addition to locking funds until a deadline, this module also updates the accounting ledger showing that the assets have moved.
// There is no automatic release of funds from the locked state so requires that the either the deadline to have past to allow withdrawal
// or the intervention of the permitted party to withdraw the funds.
//
// For the initial use of this prefunding module the intended beneficiary is identified by AccountId.
// In a later version there may be no intended beneficiary (for example for marketplace transactions)
// and therefore the funds may be locked until a cadidate secures the funds.
//
// A further scenario is forseen where a dispute resolution method that relies upon an independent validator
// is required to set the lock-release state.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{
        fail,
        pallet_prelude::*,
        traits::{Currency, ExistenceRequirement, LockIdentifier},
    };
    use frame_system::pallet_prelude::*;

    use sp_runtime::traits::{Convert, Hash};
    use sp_std::prelude::*;

    use totem_common::{StorageMapExt, TryConvert};
    use totem_primitives::{
        accounting::{Indicator, Ledger, Posting, Record as PostingRecord},
        escrow::{EscrowableCurrency, Reason},
        prefunding::*,
        ComparisonAmounts, LedgerBalance,
    };

    type EscrowableBalanceOf<T> = <<<T as Config>::Escrowable as EscrowableCurrency<
        <T as frame_system::Config>::AccountId,
    >>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    type CurrencyBalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Bonsai Storage.
    #[pallet::storage]
    #[pallet::getter(fn prefunding)]
    pub type Prefunding<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, (CurrencyBalanceOf<T>, T::BlockNumber)>;

    /* Hacky workaround for inability of RPC to query transaction by hash */

    /// Maps to current block number allows interrogation of errors.
    #[pallet::storage]
    #[pallet::getter(fn prefunding_hash_owner)]
    pub type PrefundingHashOwner<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        (T::AccountId, LockStatus, T::AccountId, LockStatus),
    >;

    /// Future block number beyond which the Hash should deleted.
    #[pallet::storage]
    #[pallet::getter(fn owner_prefunding_hash_list)]
    pub type OwnerPrefundingHashList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    /// Tracking to ensure that we can perform housekeeping on finalization of block.
    #[pallet::storage]
    #[pallet::getter(fn reference_status)]
    pub type ReferenceStatus<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Status>;

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_timestamp::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId>;
        type Escrowable: EscrowableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
        type PrefundingConverter: TryConvert<LedgerBalance, u128>
            + TryConvert<CurrencyBalanceOf<Self>, LedgerBalance>
            + Convert<Vec<u8>, LockIdentifier>
            + Convert<u32, Self::BlockNumber>
            + Convert<CurrencyBalanceOf<Self>, u128>
            + TryConvert<LedgerBalance, EscrowableBalanceOf<Self>>;
        type Accounting: Posting<
            Self::AccountId,
            Self::Hash,
            Self::BlockNumber,
            CurrencyBalanceOf<Self>,
        >;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// You are not the owner or the beneficiary
        LockNotAllowed1,
        /// You are not the owner or the beneficiary
        LockNotAllowed2,
        /// You are not the owner or the beneficiary
        LockNotAllowed3,
        /// You are not the owner or the beneficiary
        LockNotAllowed4,
        /// You are not the owner or the beneficiary
        LockNotAllowed5,
        /// You are not the owner or the beneficiary
        LockNotAllowed6,
        /// Not enough funds to prefund
        InsufficientPreFunds,
        /// Cannot set this state
        WrongState1,
        /// Cannot set this state
        WrongState2,
        /// Cannot set this state
        WrongState3,
        /// Cannot set this state
        WrongState4,
        /// Cannot set this state
        WrongState5,
        /// Funds already locked for intended purpose by both parties.
        NotAllowed1,
        /// Not the beneficiary
        NotAllowed2,
        /// Not the owner
        NotAllowed3,
        /// This function should not be used for this state
        NotAllowed4,
        /// Funds locked for intended purpose by both parties.
        NotAllowed5,
        /// Funds locked for beneficiary.
        NotAllowed6,
        /// The demander has not approved the work yet!
        NotApproved,
        /// The demander has not approved the work yet!
        NotApproved2,
        /// Deadline not yet passed. Wait a bit longer!
        DeadlineInPlay,
        /// Funds locked for intended purpose by both parties.
        FundsInPlay,
        /// Funds locked for intended purpose by both parties.
        FundsInPlay2,
        /// You are not the owner of the hash!
        NotOwner,
        /// You are not the owner of the hash!
        NotOwner2,
        /// This hash already exists!
        HashExists,
        /// Hash does not exist
        HashDoesNotExist,
        /// Hash does not exist
        HashDoesNotExist2,
        /// Hash does not exist
        HashDoesNotExist3,
        /// Deadline is too short! Must be at least 48 hours
        ShortDeadline,
        /// Deposit was not taken
        PrefundNotSet,
        /// An error occured posting to accounts - prefunding for...
        InAccounting1,
        /// An error occured posting to accounts - send simple invoice
        InAccounting2,
        /// An error occured posting to accounts - settle invoice
        InAccounting3,
        /// Did not set the status - prefunding for...
        SettingStatus1,
        /// Did not set the status - send simple invoice
        SettingStatus2,
        /// Error getting details from hash
        NoDetails,
        /// Error setting release state
        ReleaseState,
        /// Error unlocking for beneficiary
        Unlocking,
        /// Error cancelling prefunding
        CancellingPrefund,
        /// Error getting prefunding details
        NoPrefunding,
        /// Cancelling prefunding failed for some reason
        CancelFailed,
        /// Cancelling prefunding failed for some reason
        CancelFailed2,
        /// Value overflowed during computation.
        Overflow,
        /// Error while locking the funds.
        LockFailed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// This function reserves funds from the buyer for a specific vendor account (Closed Order). It is used when an order is created.
        /// Quantity is not relevant.
        /// The prefunded amount remains as an asset of the buyer until the order is accepted.
        /// Updates only the accounts of the buyer.
        #[pallet::weight(0/*TODO*/)]
        pub fn prefund_someone(
            origin: OriginFor<T>,
            beneficiary: T::AccountId,
            amount: CurrencyBalanceOf<T>,
            deadline: T::BlockNumber,
            tx_uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            // check that the beneficiary is not the sender
            ensure!(who != beneficiary, "Beneficiary must be another account");
            let prefunding_hash: T::Hash =
                Self::get_pseudo_random_hash(who.clone(), beneficiary.clone());

            Self::prefunding_for(who, beneficiary, amount, deadline, prefunding_hash, tx_uid)
        }

        /// Creates a single line simple invoice without taxes, tariffs or commissions.
        /// This invoice is associated with a prefunded order - therefore needs to provide the hash reference of the order.
        /// Updates the accounting for the vendor and the customer.
        #[pallet::weight(0/*TODO*/)]
        pub fn invoice_prefunded_order(
            origin: OriginFor<T>,
            payer: T::AccountId,
            amount: i128,
            reference: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Self::send_simple_invoice(who, payer, amount, reference, uid)
        }

        /// Buyer pays a prefunded order. Needs to supply the correct hash reference.
        /// Updates bother the buyer and the vendor accounts.
        #[pallet::weight(0/*TODO*/)]
        pub fn pay_prefunded_invoice(
            origin: OriginFor<T>,
            reference: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Self::settle_prefunded_invoice(who, reference, uid)
        }

        /// Is used by the buyer to recover funds if the vendor does not accept the order by the deadline.
        #[pallet::weight(0/*TODO*/)]
        pub fn cancel_prefunded_closed_order(
            origin: OriginFor<T>,
            reference: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Self::unlock_funds_for_owner(who, reference, uid)
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        PrefundingCancelled(T::AccountId, T::Hash),
        PrefundingLockSet(T::Hash),
        PrefundingCompleted(T::Hash),
        InvoiceIssued(T::Hash),
        InvoiceSettled(T::Hash),
    }

    impl<T: Config> Pallet<T> {
        /// Reserve the prefunding deposit.
        fn set_prefunding(
            s: T::AccountId,
            c: LedgerBalance,
            d: T::BlockNumber,
            h: T::Hash,
            _u: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // Prepare make sure we are not taking the deposit again
            if ReferenceStatus::<T>::contains_key(&h) {
                fail!(Error::<T>::HashExists);
            }

            // You cannot prefund any amount unless you have at least at balance of 1618 units + the amount you want to prefund
            // Ensure that the funds can be subtracted from sender's balance without causing the account to be destroyed by the existential deposit
            let min_balance: ComparisonAmounts = 1618_u128;
            let current_balance: ComparisonAmounts =
                T::PrefundingConverter::convert(T::Currency::free_balance(&s));
            let prefund_amount: ComparisonAmounts =
                T::PrefundingConverter::try_convert(c.clone()).ok_or(Error::<T>::Overflow)?;
            let minimum_amount = min_balance + prefund_amount;

            if current_balance >= minimum_amount {
                let amount = T::PrefundingConverter::try_convert(c).ok_or(Error::<T>::Overflow)?;
                // Lock the amount from the sender and set deadline
                T::Escrowable::set_lock(
                    Self::get_prefunding_id(h),
                    &s,
                    amount,
                    d,
                    Reason::Escrowing,
                )
                .or(Err(Error::<T>::LockFailed))?;
            } else {
                fail!(Error::<T>::InsufficientPreFunds);
            }

            Ok(().into())
        }

        /// Generate a Prefund Id from a hash.
        fn get_prefunding_id(hash: T::Hash) -> LockIdentifier {
            // Convert Hash to ID using first 8 bytes of hash
            T::PrefundingConverter::convert(hash.encode())
        }

        /// Generate a reference from a hash.
        fn get_pseudo_random_hash(sender: T::AccountId, recipient: T::AccountId) -> T::Hash {
            let tuple = (sender, recipient);
            let input = (
                tuple,
                pallet_timestamp::Pallet::<T>::get(),
                sp_io::offchain::random_seed(),
                frame_system::Pallet::<T>::extrinsic_index(),
                frame_system::Pallet::<T>::block_number(),
            );

            T::Hashing::hash(input.encode().as_slice()) // default hash BlakeTwo256
        }

        /// Check a hash exists and is valid.
        fn reference_valid(h: T::Hash) -> bool {
            match ReferenceStatus::<T>::get(&h) {
                Some(0) | Some(1) | Some(100) | Some(200) | Some(300) | Some(400) => true,
                _ => false,
            }
        }

        /// Prefunding deadline passed?
        fn prefund_deadline_passed(h: T::Hash) -> bool {
            match Self::prefunding(&h) {
                Some((_, deadline)) if deadline < frame_system::Pallet::<T>::block_number() => true,
                _ => false,
            }
        }

        /// Gets the state of the locked funds.
        /// The hash needs to be prequalified before passing in as no checks performed here.
        fn get_release_state(h: T::Hash) -> (LockStatus, LockStatus) {
            let owners = Self::prefunding_hash_owner(&h).unwrap(); //TODO

            (owners.1, owners.3)
        }

        /// Cancels lock for owner.
        fn cancel_prefunding_lock(
            o: T::AccountId,
            h: T::Hash,
            s: Status,
        ) -> DispatchResultWithPostInfo {
            // funds can be unlocked for the owner
            // convert hash to lock identifyer
            let prefunding_id = Self::get_prefunding_id(h);
            // unlock the funds
            T::Escrowable::remove_lock(prefunding_id, &o).or(Err(Error::<T>::LockFailed))?;
            // perform cleanup removing all reference hashes. No accounting posting have been made, so no cleanup needed there
            Prefunding::<T>::remove(&h);
            PrefundingHashOwner::<T>::remove(&h);
            ReferenceStatus::<T>::insert(&h, s); // This sets the status but does not remove the hash
            OwnerPrefundingHashList::<T>::mutate_or_err(&o, |owner_prefunding_hash_list| {
                owner_prefunding_hash_list.retain(|e| e != &h)
            })?;

            // Issue event
            Self::deposit_event(Event::PrefundingCancelled(o, h));

            Ok(().into())
        }

        /// Unlocks & pays beneficiary with funds transfer and account updates (settlement of invoice).
        fn unlock_funds_for_beneficiary(
            o: T::AccountId,
            h: T::Hash,
            _u: T::Hash,
        ) -> DispatchResultWithPostInfo {
            use LockStatus::*;

            if Self::reference_valid(h) == false {
                fail!(Error::<T>::HashDoesNotExist);
            }

            if Self::check_ref_beneficiary(o.clone(), h) == false {
                fail!(Error::<T>::NotOwner);
            }

            // TODO this should return the details otherwise there is second read later in the process
            match Self::get_release_state(h) {
                // submitted, but not yet accepted
                (Locked, Unlocked) => fail!(Error::<T>::NotApproved),
                (Locked, Locked) => fail!(Error::<T>::FundsInPlay),
                // Owner has approved now get status of hash. Only allow if invoiced.
                // Note handling the account posting is done outside of this function
                (Unlocked, Locked) => {
                    match ReferenceStatus::<T>::get(&h) {
                        Some(400) => {
                            // get details of lock
                            let details =
                                Self::prefunding_hash_owner(&h).ok_or("Error fetching details")?;
                            // get details of prefunding
                            let prefunding =
                                Self::prefunding(&h).ok_or("Error getting prefunding details")?;
                            // Cancel prefunding lock
                            let status: Status = 500; // Settled
                            Self::cancel_prefunding_lock(details.0.clone(), h, status)?;
                            // transfer to beneficiary.
                            // TODO when currency conversion is implemnted the payment should be at the current rate for the currency
                            if let Err(_) = T::Currency::transfer(
                                &details.0,
                                &o,
                                prefunding.0,
                                ExistenceRequirement::KeepAlive,
                            ) {
                                fail!("Error during transfer")
                            }
                        }
                        _ => fail!("Only allowed when status is Invoiced"),
                    }
                }
                // Owner has been given permission by beneficiary to release funds
                (Unlocked, Unlocked) => fail!(Error::<T>::NotAllowed1),
            }

            Ok(().into())
        }

        /// Set the status for the prefunding.
        fn set_ref_status(h: T::Hash, s: Status) -> DispatchResultWithPostInfo {
            ReferenceStatus::<T>::insert(&h, s);

            Ok(().into())
        }

        // TODO Check should be made for available balances, and if the amount submitted is more than the invoice amount.
        /// Settles invoice by updates to various relevant accounts and transfer of funds.
        #[allow(dead_code)/*TODO use it */]
        fn settle_unfunded_invoice() -> DispatchResultWithPostInfo {
            fail!("TODO")
        }

        /// Return a pair of:
        /// - The amount given as a parameter, but signed.
        /// - The opposite of that amount.
        fn increase_decrease_amounts(
            amount: CurrencyBalanceOf<T>,
        ) -> Result<(LedgerBalance, LedgerBalance), Error<T>> {
            let increase_amount: LedgerBalance =
                T::PrefundingConverter::try_convert(amount).ok_or(Error::<T>::Overflow)?;
            let decrease_amount = increase_amount.checked_neg().ok_or(Error::<T>::Overflow)?;

            Ok((increase_amount, decrease_amount))
        }
    }

    impl<T: Config> Encumbrance<T::AccountId, T::Hash, T::BlockNumber, CurrencyBalanceOf<T>>
        for Pallet<T>
    {
        fn prefunding_for(
            who: T::AccountId,
            recipient: T::AccountId,
            amount: CurrencyBalanceOf<T>, //todo rename amount
            deadline: T::BlockNumber,
            ref_hash: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // As amount will always be positive, convert for use in accounting
            let (increase_amount, decrease_amount) = Self::increase_decrease_amounts(amount)?;
            let current_block = <frame_system::Pallet<T>>::block_number();
            // Prefunding is always recorded in the same block. It cannot be posted to another period
            let current_block_dupe = <frame_system::Pallet<T>>::block_number();
            let prefunding_hash: T::Hash = ref_hash;
            // NEED TO CHECK THAT THE DEADLINE IS SENSIBLE!!!!
            // 48 hours is the minimum deadline. This is the minimum amountof time before the money can be reclaimed
            let minimum_deadline: T::BlockNumber = current_block
                + <T::PrefundingConverter as Convert<u32, T::BlockNumber>>::convert(11520_u32);
            if deadline < minimum_deadline {
                fail!(Error::<T>::ShortDeadline);
            }
            let prefunded = (amount, deadline.clone());
            let owners = (who.clone(), true, recipient, false);
            // manage the deposit
            if let Err(_) =
                Self::set_prefunding(who.clone(), increase_amount, deadline, prefunding_hash, uid)
            {
                fail!(Error::<T>::PrefundNotSet);
            }

            // Deposit taken at this point. Note that if an error occurs beyond here we need to remove the locked funds.
            let keys = [
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::B1010005_0000000D, // debit  increase 110100050000000 Prefunding Account
                    amount: increase_amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: prefunding_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::B1010004_0000000D, // credit decrease 110100040000000 XTX Balance
                    amount: decrease_amount,
                    debit_credit: Indicator::Debit,
                    reference_hash: prefunding_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::C6060002_0000000D, // debit  increase 360600020000000 Runtime Ledger by Module
                    amount: increase_amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: prefunding_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::C6060006_0000000D, // debit  increase 360600060000000 Runtime Ledger Control
                    amount: increase_amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: prefunding_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
            ];

            if let Err(_) = T::Accounting::handle_multiposting_amounts(&keys) {
                fail!(Error::<T>::InAccounting1);
            }

            // Record Prefunding ownership and status
            PrefundingHashOwner::<T>::insert(&prefunding_hash, owners);
            Prefunding::<T>::insert(&prefunding_hash, prefunded);

            // Add reference hash to list of hashes
            OwnerPrefundingHashList::<T>::mutate_or_err(&who, |owner_prefunding_hash_list| {
                owner_prefunding_hash_list.push(prefunding_hash)
            })?;

            // Submitted, Locked by sender.
            if let Err(_) = Self::set_ref_status(prefunding_hash, 1) {
                fail!(Error::<T>::SettingStatus1);
            }

            Self::deposit_event(Event::PrefundingCompleted(uid));

            Ok(().into())
        }

        /// Simple invoice. Does not include tax jurisdiction, tax amounts, freight, commissions,
        /// tariffs, discounts and other extended line item values.
        /// Must include a connection to the originating reference.
        /// Invoices cannot be made to parties that haven't asked for something identified by a valid hash.
        fn send_simple_invoice(
            who: T::AccountId,
            recipient: T::AccountId,
            amount: LedgerBalance,
            ref_hash: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // Validate that the hash is indeed assigned to the seller
            if Self::check_ref_beneficiary(who.clone(), ref_hash) == false {
                fail!(Error::<T>::NotAllowed2);
            }

            // Amount CAN be negative - this is therefore not an Invoice but a Credit Note!
            // The account postings are identical to an invoice, however we must also handle the refund immediately if possible.
            // In order to proceed with a credit note, validate that the vendor has sufficient funds.
            // If they do not have sufficient funds, the credit note can still be issued, but will remain outstanding until it is settled.
            // As amount will always be positive, convert for use in accounting
            let current_block = frame_system::Pallet::<T>::block_number();
            let current_block_dupe = frame_system::Pallet::<T>::block_number();

            // Keys for posting
            let keys = [
                // Seller
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::B1010008_0000000D, // Debit  increase 110100080000000	Accounts receivable (Sales Control Account or Trade Debtor's Account)
                    amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::P4040001_0000000C, // Credit increase 240400010000000	Product or Service Sales
                    amount,
                    debit_credit: Indicator::Debit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::C6060001_0000000D, // Debit  increase 360600010000000	Sales Ledger by Payer
                    amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: who.clone(),
                    counterparty: who.clone(),
                    ledger: Ledger::C6060005_0000000D, // Debit  increase 360600050000000	Sales Ledger Control
                    amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                // Buyer
                PostingRecord {
                    primary_party: recipient.clone(),
                    counterparty: recipient.clone(),
                    ledger: Ledger::B2020003_0000000C, // Credit increase 120200030000000	Accounts payable
                    amount,
                    debit_credit: Indicator::Debit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: recipient.clone(),
                    counterparty: recipient.clone(),
                    ledger: Ledger::P5050012_0000013D, // Debit  increase 250500120000013	Labour
                    amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: recipient.clone(),
                    counterparty: recipient.clone(),
                    ledger: Ledger::C6060003_0000000D, // Debit  increase 360600030000000	Purchase Ledger by Vendor
                    amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
                PostingRecord {
                    primary_party: recipient.clone(),
                    counterparty: recipient.clone(),
                    ledger: Ledger::C6060007_0000000D, // Debit  increase 360600070000000	Purchase Ledger Control
                    amount,
                    debit_credit: Indicator::Credit,
                    reference_hash: ref_hash,
                    changed_on_blocknumber: current_block,
                    applicable_period_blocknumber: current_block_dupe,
                },
            ];

            if let Err(_) = T::Accounting::handle_multiposting_amounts(&keys) {
                fail!(Error::<T>::InAccounting2);
            }

            // Add status processing
            let new_status: Status = 400; // invoiced(400), can no longer be accepted,
            if let Err(_) = Self::set_ref_status(ref_hash, new_status) {
                fail!(Error::<T>::SettingStatus2);
            }

            Self::deposit_event(Event::InvoiceIssued(uid));

            Ok(().into())
        }

        /// Settles invoice by unlocking funds and updates various relevant accounts and pays prefunded amount.
        fn settle_prefunded_invoice(
            who: T::AccountId,
            ref_hash: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            use LockStatus::*;

            // release state must be 11
            // sender must be owner
            // accounts updated before payment, because if there is an error then the accounting can be rolled back
            let (payer, beneficiary) = match Self::get_release_state(ref_hash) {
                // submitted, but not yet accepted
                (Locked, Unlocked) => fail!(Error::<T>::NotApproved2),
                (Locked, Locked) => {
                    // Validate that the hash is indeed owned by the buyer
                    if Self::check_ref_owner(who.clone(), ref_hash) == false {
                        fail!(Error::<T>::NotAllowed3);
                    }

                    // get beneficiary from hash
                    let (_, _, details /*TODO better name*/, _) =
                        Self::prefunding_hash_owner(&ref_hash).ok_or(Error::<T>::NoDetails)?;
                    // get prefunding amount for posting to accounts
                    let (prefunded_amount, _) =
                        Self::prefunding(&ref_hash).ok_or(Error::<T>::NoPrefunding)?;
                    // convert to Account Balance type
                    let (increase_amount, decrease_amount) =
                        Self::increase_decrease_amounts(prefunded_amount)?;
                    let current_block = frame_system::Pallet::<T>::block_number();
                    let current_block_dupe = frame_system::Pallet::<T>::block_number();

                    // Keys for posting
                    let keys = [
                        // Buyer
                        PostingRecord {
                            primary_party: who.clone(),
                            counterparty: who.clone(),
                            ledger: Ledger::B2020003_0000000C, // 120200030000000	Debit  decrease Accounts payable
                            amount: decrease_amount,
                            debit_credit: Indicator::Credit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: who.clone(),
                            counterparty: who.clone(),
                            ledger: Ledger::B1010005_0000000D, // 110100050000000	Credit decrease Totem Runtime Deposit (Escrow)
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: who.clone(),
                            counterparty: who.clone(),
                            ledger: Ledger::C6060002_0000000D, // 360600020000000	Credit decrease Runtime Ledger by Module
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: who.clone(),
                            counterparty: who.clone(),
                            ledger: Ledger::C6060006_0000000D, // 360600060000000	Credit decrease Runtime Ledger Control
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: who.clone(),
                            counterparty: who.clone(),
                            ledger: Ledger::C6060003_0000000D, // 360600030000000	Credit decrease Purchase Ledger by Vendor
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: who.clone(),
                            counterparty: who.clone(),
                            ledger: Ledger::C6060007_0000000D, // 360600070000000	Credit decrease Purchase Ledger Control
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        // Seller
                        PostingRecord {
                            primary_party: details.clone(),
                            counterparty: details.clone(),
                            ledger: Ledger::B1010004_0000000D, // 110100040000000	Debit  increase XTX Balance
                            amount: increase_amount,
                            debit_credit: Indicator::Credit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: details.clone(),
                            counterparty: details.clone(),
                            ledger: Ledger::B1010008_0000000D, // 110100080000000	Credit decrease Accounts receivable (Sales Control Account or Trade Debtor's Account)
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: details.clone(),
                            counterparty: details.clone(),
                            ledger: Ledger::C6060001_0000000D, // 360600010000000	Credit decrease Sales Ledger by Payer
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                        PostingRecord {
                            primary_party: details.clone(),
                            counterparty: details.clone(),
                            ledger: Ledger::C6060005_0000000D, // 360600050000000	Credit decrease Sales Ledger Control
                            amount: decrease_amount,
                            debit_credit: Indicator::Debit,
                            reference_hash: ref_hash,
                            changed_on_blocknumber: current_block,
                            applicable_period_blocknumber: current_block_dupe,
                        },
                    ];

                    if let Err(_) = T::Accounting::handle_multiposting_amounts(&keys) {
                        fail!(Error::<T>::InAccounting3);
                    }

                    // export details for final payment steps
                    (who, details)
                }
                // This state is not allowed for this functions
                (Unlocked, Locked) => fail!(Error::<T>::NotAllowed4),
                // Owner has been given permission by beneficiary to release funds
                (Unlocked, Unlocked) => fail!(Error::<T>::NotAllowed5),
            };

            // Set release lock "buyer who has approved invoice"
            // this may have been set independently, but is required for next step
            if let Err(_) = Self::set_release_state(payer, Unlocked, ref_hash, uid) {
                fail!(Error::<T>::ReleaseState);
            }

            // Unlock, tansfer funds and mark hash as settled in full
            if let Err(_) = Self::unlock_funds_for_beneficiary(beneficiary, ref_hash, uid) {
                fail!(Error::<T>::Unlocking);
            }

            Self::deposit_event(Event::InvoiceSettled(uid));

            Ok(().into())
        }

        /// Sets the release state by the owner or the beneficiary is only called when something already exists.
        fn set_release_state(
            who: T::AccountId,
            o_lock: LockStatus,
            ref_hash: T::Hash,
            uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            use LockStatus::*;

            // 0= false, 1=true
            // 10, sender can take after deadline (initial state)
            // 11, accepted by recipient. (funds locked, nobody can take)
            // 01, sender approves (recipient can take, or refund)
            // 00, only the recipient authorises sender to retake funds regardless of deadline.
            // Initialise new tuple with some dummy values
            let mut change = (who.clone(), Unlocked, who.clone(), Unlocked);

            match Self::prefunding_hash_owner(&ref_hash) {
                Some(state_lock) => {
                    let locks = (state_lock.1, state_lock.3);
                    change.0 = state_lock.0.clone();
                    change.2 = state_lock.2.clone();
                    let commander = state_lock.0.clone();
                    let fulfiller = state_lock.2.clone();
                    match locks {
                        // In this state the commander has created the lock, but it has not been accepted.
                        // The commander can withdraw the lock (set to false) if the deadline has passed, or
                        // the fulfiller can accept the order (set to true)
                        (Locked, Unlocked) => {
                            match o_lock {
                                Locked => {
                                    if who == commander {
                                        fail!(Error::<T>::WrongState1);
                                    } else if who == fulfiller {
                                        change.1 = state_lock.1;
                                        change.3 = o_lock;
                                    } else {
                                        fail!(Error::<T>::LockNotAllowed1);
                                    };
                                }
                                Unlocked => {
                                    // We do care if the deadline has passed IF this is the commander calling directly
                                    // but that must be handled outside of this function
                                    if who == commander {
                                        change.1 = o_lock;
                                        change.3 = state_lock.3;
                                    } else if who == fulfiller {
                                        fail!(Error::<T>::WrongState2);
                                    } else {
                                        fail!(Error::<T>::LockNotAllowed2);
                                    };
                                }
                            }
                        }
                        // In this state the commander can change the lock, and they can only change it to false
                        // In this state the fulfiller can change the lock, and they can only change it to false
                        (Locked, Locked) => match o_lock {
                            Locked => fail!(Error::<T>::WrongState3),
                            Unlocked => {
                                if who == commander {
                                    change.1 = o_lock;
                                    change.3 = state_lock.3;
                                } else if who == fulfiller {
                                    change.1 = state_lock.1;
                                    change.3 = o_lock;
                                } else {
                                    fail!(Error::<T>::LockNotAllowed3);
                                }
                            }
                        },
                        // In this state the commander cannot change the lock
                        // In this state the fulfiller can change the lock, and they can only change it to false
                        (Unlocked, Locked) => match o_lock {
                            Locked => fail!(Error::<T>::LockNotAllowed4),
                            Unlocked => {
                                if who == commander {
                                    fail!(Error::<T>::WrongState5);
                                } else if who == fulfiller {
                                    change.1 = state_lock.1;
                                    change.3 = o_lock;
                                } else {
                                    fail!(Error::<T>::LockNotAllowed5);
                                };
                            }
                        },
                        // This state should technically make the funds refundable to the buyer.
                        // Even if the buy wanted to set this state they cannot. Meaning they must create a new order.
                        (Unlocked, Unlocked) => fail!(Error::<T>::LockNotAllowed5),
                    }
                }
                None => fail!(Error::<T>::HashDoesNotExist2),
            };
            PrefundingHashOwner::<T>::insert(&ref_hash, change);
            // Issue event
            Self::deposit_event(Event::PrefundingLockSet(uid));

            Ok(().into())
        }

        /// Unlocks for owner.
        fn unlock_funds_for_owner(
            who: T::AccountId,
            ref_hash: T::Hash,
            _uid: T::Hash,
        ) -> DispatchResultWithPostInfo {
            use LockStatus::*;

            if Self::reference_valid(ref_hash) == false {
                fail!(Error::<T>::HashDoesNotExist3);
            }

            if Self::check_ref_owner(who.clone(), ref_hash) == false {
                fail!(Error::<T>::NotOwner2);
            }

            match Self::get_release_state(ref_hash) {
                // submitted, but not yet accepted
                // Check if the dealine has passed. If not funds cannot be release
                (Locked, Unlocked) => {
                    if Self::prefund_deadline_passed(ref_hash) {
                        let status: Status = 50; // Abandoned or Cancelled
                        if let Err(_) = Self::cancel_prefunding_lock(who, ref_hash, status) {
                            fail!(Error::<T>::CancelFailed2);
                        }
                    } else {
                        fail!(Error::<T>::DeadlineInPlay);
                    }
                }
                (Locked, Locked) => fail!(Error::<T>::FundsInPlay2),
                (Unlocked, Locked) => fail!(Error::<T>::NotAllowed6),
                (Unlocked, Unlocked) => {
                    // Owner has been  given permission by beneficiary to release funds
                    let status: Status = 50; // Abandoned or cancelled
                    if let Err(_) = Self::cancel_prefunding_lock(who, ref_hash, status) {
                        fail!(Error::<T>::CancellingPrefund);
                    }
                }
            }

            Ok(().into())
        }

        /// Checks owner (of hash) - if anything fails then returns false.
        fn check_ref_owner(who: T::AccountId, ref_hash: T::Hash) -> bool {
            match Self::prefunding_hash_owner(&ref_hash) {
                Some(owners) if owners.0 == who => true,
                _ => false,
            }
        }

        /// Checks beneficiary (of hash reference).
        fn check_ref_beneficiary(who: T::AccountId, ref_hash: T::Hash) -> bool {
            match Self::prefunding_hash_owner(&ref_hash) {
                Some(owners) if owners.2 == who => true,
                _ => false,
            }
        }
    }
}
