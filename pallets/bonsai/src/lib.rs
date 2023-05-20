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

//! Provides a decentralised authority for data storage.
//!
//! In Totem we require an off-chain searchable database that may end up containing billions of records.
//! IPFS is not a solution as the type of data to be stored may be queried, editied, and each time IPFS cannot overwrite or update existing datasets.
//! Additionally IPFS may drop files that are not considered current, used or needed, which is not ideal for static records like invoices.
//!
//! We wanted a solution where permission for storing an editing data should not be dependent on third-party authentication and access
//! was global, recoverable and self-sovereign.
//!
//! Bonsai is a simple protocol, for allowing independent databases to come to a consensus on content.
//! It works by assuming that the data to be stored must be previously authenticated by its owner on-chain.
//!
//! # How it works
//!
//! Firstly, a reference to the record is created either on-chain or offchain by an account which immediately becomes its owner.
//! The reference is a hash (H256) with sufficient entropy to be unique per the record.
//! A transaction is sent to the blockchain at some point associating the reference to an address for the first time.
//! The reference is considered to be the key to some other data which is not suitable for onchain storage, but will be stored in an offchain database.
//! The offchain database will only accept new or changing records, provided that it can
//! a) find the reference hash onchain, and
//! b) an associated data-hash which it also finds on chain with a hash of the incoming data.
//! The data may be plaintext or encrypted, neither matters as long as the hash of this data matches onchain data-hash.
//! As the on-chain transaction validates the signature, the off-chain database does not need to authenticate the client that communicates
//! the insertion or change request as it has already been "pre-authorised" by the blockchain runtime.
//! Totem believes there is a fee market for storage in this model.
//!
//! # Process
//!
//! A third party database receives a request to store some data. The Database queries the blockchain to find out:
//!
//! 1. Does the reference hash exist on chain and of it does, then collect the associated data-hash also stored onchain;
//! 2. Upon confirmation the reference hash exists, hashing the received data and compare the data-hash to the one found on chain. If it does not match, then do nothing
//! (effectively rejecting the attempt to store the data), and if it does match then store the data using the reference hash as the key;
//! 3. In the event that an reference hash already exists, the data-hash obtained from the blockchain is always king. Provided it matches, overwrite exiting data.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{
        pallet_prelude::*,
        traits::StorageVersion,
        ensure,
    };
    use frame_system::pallet_prelude::*;

    use sp_runtime::traits::{Convert, Hash};
    use sp_std::prelude::*;

    use totem_common::StorageMapExt;
    use totem_primitives::{
        bonsai::Storing, orders::Validating as OrderValidating,
        teams::Validating as TeamsValidating, timekeeping::Validating as TimeValidating,
        RecordType,
    };

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Bonsai Storage.
    #[pallet::storage]
    #[pallet::getter(fn is_valid_record)]
    pub type IsValidRecord<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::Hash>;

    /* Hacky workaround for inability of RPC to query transaction by hash */

    /// Maps to current block number allows interrogation of errors.
    #[pallet::storage]
    #[pallet::getter(fn is_started)]
    pub type IsStarted<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::BlockNumber>;

    /// Future block number beyond which the Hash should deleted.
    #[pallet::storage]
    #[pallet::getter(fn is_successful)]
    pub type IsSuccessful<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::BlockNumber>;

    /// Tracking to ensure that we can perform housekeeping on finalization of block.
    #[pallet::storage]
    #[pallet::getter(fn tx_list)]
    pub type TxList<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::Hash>>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Timekeeping: TimeValidating<Self::AccountId, Self::Hash>;
        type Teams: TeamsValidating<Self::AccountId, Self::Hash>;
        type Orders: OrderValidating<Self::AccountId, Self::Hash>;
        type BonsaiConverter: Convert<Self::BlockNumber, u32> + Convert<u32, Self::BlockNumber>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Queued transaction already completed.
        TransactionCompleted,
        /// Someone is attempting to use this TX_UID after a transaction failed.
        TransactionIdInUse,
        /// Incorrect owner of transaction
        NotTransactionOwner,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// This function stores a record hash for BONSAI 2FA for couchDB
        ///
        /// Record types are the same as the Archive Record Types
        /// * 3000 Activities (previously Teams)
        /// * 4000 Timekeeping
        /// * 5000 Orders
        ///
        #[pallet::weight(0/*TODO*/)]
        pub fn update_record(
            origin: OriginFor<T>,
            record_type: RecordType,
            key: T::Hash,
            bonsai_token: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // check transaction signed
            let who = ensure_signed(origin)?;
            Self::check_remote_ownership(who, key.clone(), bonsai_token.clone(), record_type)?;
            Self::insert_record(key, bonsai_token)?;

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn on_finalize_example(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;
            let current_block: T::BlockNumber = frame_system::Pallet::<T>::block_number();
            let current: u32 = T::BonsaiConverter::convert(current_block);
            // Get all hashes
            let default_bytes = b"nobody can save fiat currency now";
            let list_key: T::Hash = T::Hashing::hash(default_bytes.encode().as_slice());
            if let Some(hashes) = Self::tx_list(&list_key) {
                // check which storage the hashes come from and hashes that are old
                for key in hashes {
                    match Self::is_started(&key) {
                        Some(block) => {
                            let target_block = T::BonsaiConverter::convert(block) + 172800_u32;
                            // let mut target_deletion_block: T::BlockNumber = <T::BonsaiConverter as Convert<u32, T::BlockNumber>>::convert(target_block);
                            // cleanup 30 Days from when the transaction started, but did not complete
                            // It's possible this comparison is not working
                            if current >= target_block {
                                IsStarted::<T>::remove(key);
                            }
                        }
                        None => {
                            if let Some(block) = Self::is_successful(&key) {
                                let target_block = T::BonsaiConverter::convert(block);
                                if current >= target_block {
                                    IsSuccessful::<T>::remove(key);
                                }
                            }
                        }
                    }
                    TxList::<T>::mutate_or_err(&list_key, |tx_list| tx_list.retain(|v| v != &key))?;
                }
            }

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// You are not the owner of this Record.
        ErrorRecordOwner(T::Hash),
        /// This is an unknown record type.
        ErrorUnknownType(T::Hash),
    }

    impl<T: Config> Pallet<T> {
        fn check_remote_ownership(
            o: T::AccountId,
            k: T::Hash,
            t: T::Hash,
            e: RecordType,
        ) -> DispatchResultWithPostInfo {
            // check which type of record
            // then check that the supplied hash is owned by the signer of the transaction
            match e {
                RecordType::Teams => {
                    ensure!(T::Teams::is_team_owner(o, k), Error::<T>::NotTransactionOwner);
                    // if false == T::Teams::is_team_owner(o, k) {
                        // Self::deposit_event(Event::ErrorRecordOwner(t));
                        // return Err("You cannot add a record you do not own");
                        // }
                }
                RecordType::Timekeeping => {
                    ensure!(T::Timekeeping::is_time_record_owner(o, k), Error::<T>::NotTransactionOwner);
                    // if false == T::Timekeeping::is_time_record_owner(o, k) {
                        // Self::deposit_event(Event::ErrorRecordOwner(t));
                        // return Err("You cannot add a record you do not own");
                        // }
                }
                    RecordType::Orders => {
                    ensure!(T::Orders::is_order_party(o, k), Error::<T>::NotTransactionOwner);
                    // if false == T::Orders::is_order_party(o, k) {
                    //     Self::deposit_event(Event::ErrorRecordOwner(t));
                    //     return Err("You cannot add a record you do not own");
                    // }
                }
            }

            Ok(().into())
        }

        fn insert_record(k: T::Hash, t: T::Hash) -> DispatchResultWithPostInfo {
            // TODO implement fee payment mechanism (currently just transaction fee) for archiving a record.
            IsValidRecord::<T>::insert(k, t);

            Ok(().into())
        }

        fn start_uuid(u: T::Hash) -> DispatchResultWithPostInfo {
            ensure!(!IsSuccessful::<T>::contains_key(&u), Error::<T>::TransactionCompleted);
            ensure!(!IsStarted::<T>::contains_key(&u), Error::<T>::TransactionIdInUse);
            let current_block = frame_system::Pallet::<T>::block_number();
            let default_bytes = b"nobody can save fiat currency now";
            let list_key: T::Hash = T::Hashing::hash(default_bytes.encode().as_slice());
            TxList::<T>::mutate_or_err(list_key, |tx_list| tx_list.push(u))?;
            IsStarted::<T>::insert(u, current_block);

            // if IsSuccessful::<T>::contains_key(&u) {
            //     // Throw an error because the transaction already completed.
            //     return Err(Error::<T>::TransactionCompleted);
            // } else if IsStarted::<T>::contains_key(&u) {
            //     // Apparently someone is attempting to use this TX_UID after a transaction failed.
            //     return Err(Error::<T>::TransactionIdInUse);
            // } else {
                //     // this is a new UUID just starting the transaction
                //     let current_block = frame_system::Pallet::<T>::block_number();
            //     let default_bytes = b"nobody can save fiat currency now";
            //     let list_key: T::Hash = T::Hashing::hash(default_bytes.encode().as_slice());
            //     TxList::<T>::mutate_or_err(list_key, |tx_list| tx_list.push(u))?;
            //     IsStarted::<T>::insert(u, current_block);
            // }

            Ok(().into())
        }

        fn end_uuid(u: T::Hash) -> DispatchResultWithPostInfo {
            ensure!(!IsSuccessful::<T>::contains_key(&u), Error::<T>::TransactionCompleted);
            // The transaction is now completed successfully update the state change
            // remove from started, and place in successful
            let current_block = frame_system::Pallet::<T>::block_number();
            let block: u32 = T::BonsaiConverter::convert(current_block);
            let block = block + 172800_u32; // cleanup in 30 Days
            let deletion_block: T::BlockNumber = T::BonsaiConverter::convert(block);
            IsStarted::<T>::remove(&u);
            IsSuccessful::<T>::insert(u, deletion_block);

            // if IsSuccessful::<T>::contains_key(&u) {
            //     // Throw an error because the transaction already completed
            //     return Err(Error::<T>::TransactionCompleted);
            // } else if IsStarted::<T>::contains_key(&u) {
            //     // The transaction is now completed successfully update the state change
            //     // remove from started, and place in successful
            //     let current_block = frame_system::Pallet::<T>::block_number();
            //     let block: u32 = T::BonsaiConverter::convert(current_block);
            //     let block = block + 172800_u32; // cleanup in 30 Days
            //     let deletion_block: T::BlockNumber = T::BonsaiConverter::convert(block);
            //     IsStarted::<T>::remove(&u);
            //     IsSuccessful::<T>::insert(u, deletion_block);
            // } else {
            //     // This situation should not exist.
            //     return Err(Error::<T>::TransactionCompleted);
            // }

            Ok(().into())
        }
    }

    impl<T: Config> Storing<T::Hash> for Pallet<T> {
        fn claim_data(r: T::Hash, d: T::Hash) -> DispatchResultWithPostInfo {
            Self::insert_record(r, d)
        }

        fn start_tx(u: T::Hash) -> DispatchResultWithPostInfo {
            Self::start_uuid(u)
        }

        fn end_tx(u: T::Hash) -> DispatchResultWithPostInfo {
            Self::end_uuid(u)
        }
    }
}
