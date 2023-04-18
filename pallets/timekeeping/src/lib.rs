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

//! Pallet used for time measurement.
//!
//! It is recognised that measurements of time periods using block numbers as a timestamp is not the recommended approach
//! due to significant time-drift over long periods of elapsed time.
//!
//! This module however uses number of blocks as a time measurement (with 1 block equivalent to approximately 15 seconds)
//! on the basis that the employee's working time measurement segments do not present a
//! significant calculation risk when measuring and capturing relatively small amounts of booked time.
//! The blocktime therefore behaves similar to a stopwatch for timekeeping.
//!
//! It should be noted that validators timestamp each new block with the "correct" timestamp, which can be retrieved
//! when needed to provide time analysis for accounting entries.

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
        sp_runtime::traits::{
            Hash,
            BadOrigin,
        },
        traits::StorageVersion,
    };
    use frame_system::pallet_prelude::*;

    use sp_std::prelude::*;

    use totem_common::StorageMapExt;
    use totem_primitives::{teams::Validating as TeamValidating, timekeeping::*};

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Team owner sends team ref to worker address (AccountId is the Worker).
    /// Note: Currently unbounded Vec!
    ///
    /// This is  a list of the Teams that are currently assigned by a team owner.
    /// The worker can accept to work on these, or remove them from the list.
    /// If they have already worked on them they cannot be removed.
    #[pallet::storage]
    #[pallet::getter(fn worker_teams_backlog_list)]
    pub type WorkerTeamsBacklogList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    /// Accepted Status is true/false.
    #[pallet::storage]
    #[pallet::getter(fn worker_teams_backlog_status)]
    pub type WorkerTeamsBacklogStatus<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::Hash, T::AccountId), AcceptAssignedStatus>;

    /// List of all workers (team) booking time on the team.
    /// Used mainly by the Team owner, but other workers can be seen.
    /// The two here will logically replace the above two storage items, however as much of the code is dependent on the status.
    /// There will have to be a re-write.
    ///
    /// Note: Currently unbounded Vec!
    #[pallet::storage]
    #[pallet::getter(fn team_invites_list)]
    pub type TeamInvitesList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::AccountId>>;

    #[pallet::storage]
    #[pallet::getter(fn team_workers_list)]
    pub type TeamWorkersList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::AccountId>>;

    /// Team worker can be banned by team owner.
    ///
    /// # Notice
    ///
    /// **Team owner should not ban itself.**
    #[pallet::storage]
    #[pallet::getter(fn team_workers_ban_list)]
    pub type TeamWorkersBanList<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::Hash, T::AccountId), BannedStruct>;

    #[pallet::storage]
    #[pallet::getter(fn team_first_seen)]
    // When did the team first book time (blocknumber = first seen block number).
    // maybe this should be moved to the teams.rs file?
    pub type TeamFirstSeen<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, StartOrEndBlockNumber>;

    /// This stores the total number of blocks (blocktime) for a given team.
    /// It collates all time by all team members.
    #[pallet::storage]
    #[pallet::getter(fn total_blocks_per_team)]
    pub type TotalBlocksPerTeam<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, NumberOfBlocks>;

    /// This records the total amount of blocks booked per address (worker), per team.
    /// It records the first seen block which indicates when the team worker first worked on the team
    /// It also records the total time (number of blocks) for that address.
    #[pallet::storage]
    #[pallet::getter(fn total_blocks_per_team_per_address)]
    pub type TotalBlocksPerTeamPerAddress<T: Config> =
        StorageMap<_, Blake2_128Concat, (T::AccountId, T::Hash), NumberOfBlocks>;

    /// Overall hours worked on all teams for a given address for all teams.
    #[pallet::storage]
    #[pallet::getter(fn total_blocks_per_address)]
    pub type TotalBlocksPerAddress<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, NumberOfBlocks>;

    /// Time Record Hashes created by submitter.
    ///
    /// Unbounded! TODO.
    #[pallet::storage]
    #[pallet::getter(fn worker_time_records_hash_list)]
    pub type WorkerTimeRecordsHashList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    /// Simple getter to associate time record to owner.
    #[pallet::storage]
    #[pallet::getter(fn time_hash_owner)]
    pub type TimeHashOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::AccountId>;

    /// All the time records for a given team.
    ///
    /// Unbounded! TODO
    #[pallet::storage]
    #[pallet::getter(fn team_time_records_hash_list)]
    pub type TeamTimeRecordsHashList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::Hash>>;

    /// This records the amount of blocks per address, per team, per entry.
    /// Start block number can be calculated.
    /// Only accepted if an end block number is given in the transaction as this is the "service rendered" date for accounting purposes.
    #[pallet::storage]
    #[pallet::getter(fn time_record)]
    pub type TimeRecord<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Hash,
        Timekeeper<
            T::AccountId,
            T::Hash,
            NumberOfBlocks,
            LockStatus,
            StatusOfTimeRecord,
            ReasonCodeStruct,
            PostingPeriod,
            StartOrEndBlockNumber,
            NumberOfBreaks,
        >,
    >;

    /// ARCHIVE Experimental! May go somewhere else in future.
    #[pallet::storage]
    #[pallet::getter(fn worker_time_records_hash_list_archive)]
    pub type WorkerTimeRecordsHashListArchive<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    /// ARCHIVE Experimental! May go somewhere else in future.
    #[pallet::storage]
    #[pallet::getter(fn team_time_records_hash_list_archive)]
    pub type TeamTimeRecordsHashListArchive<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Vec<T::Hash>>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type Teams: TeamValidating<Self::AccountId, Self::Hash>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Worker has already accepted the team.
        WorkerAlreadyAcceptedTeam,
        /// Worker already assigned the team, but hasn't formally accepted.
        WorkerAlreadyAssigned,
        /// Worker has not been assigned to this team.
        WorkerNotAssigned,
        /// This worker is banned.
        WorkerBanned,
        /// Invalid team or team owner is not correct.
        InvalidTeamOrOwner,
        /// Team not active.
        TeamInactive,
        /// Cannot remove team that has been accepted already.
        TeamCannotBeRemoved,
        /// The team cannot be changed by the team owner anymore.
        TeamCannotBeChanged,
        /// Time record not from the worker.
        TimeRecordNotFromWorker,
        /// You cannot change a locked time record.
        TimeRecordLocked,
        /// You cannot change a time record you do not own.
        TimeRecordNotOwned,
        /// Time record already invoiced. It cannot be changed.
        TimeRecordAlreadyInvoiced,
        /// Time has been blocked by Team Owner. Check the reason code.
        TimeBlocked,
        /// Time record has not been finalised by worker.
        TimeRecordNotFinalised,
        /// Team owner cannot set this status for the time record.
        TimeRecordCannotChange,
        /// Time record does not exist
        TimeRecordDoesNotExist,
        /// This status has not been implemented or is not to be set this way.
        StatusNotImplementedOr,
        /// This status has not been implemented.
        StatusNotImplemented,
        /// Cannot resubmit a record with a submitted status.
        StatusAlreadySubmitted,
        /// Nothing has changed! Record will not be updated.
        StatusIdentical,
        /// This status cannot be set here.
        StatusCannotBeSetHere,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Team owner invites worker/team member to team.
        #[pallet::weight(0/*TODO*/)]
        pub fn notify_team_worker(
            origin: OriginFor<T>,
            worker: T::AccountId,
            team_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            // check team hash exists and is owner by sender
            // let hash_has_correct_owner = <teams::Pallet<T>>::check_owner_valid_team(who.clone(), team_hash);
            let hash_has_correct_owner =
                T::Teams::is_owner_and_team_valid(who.clone(), team_hash);
            ensure!(hash_has_correct_owner, Error::<T>::InvalidTeamOrOwner);

            // ensure that the team has not already been assigned to the worker, and that they have accepted already
            let status_tuple_key = (team_hash, worker.clone());
            // TODO this should be changed for an enum
            if let Some(status) = Self::worker_teams_backlog_status(&status_tuple_key) {
                return Err(match status {
                    true => Error::<T>::WorkerAlreadyAcceptedTeam.into(),
                    false => Error::<T>::WorkerAlreadyAssigned.into(),
                });
            }

            if who == worker {
                // Adds team to list of teams assigned to worker address (in this case worker is team owner)
				WorkerTeamsBacklogList::<T>::try_mutate(&worker, |hashes| -> DispatchResult {
					match hashes {
						Some(ref mut hash_vec) => {
							hash_vec.push(team_hash);
							Ok(())
						},
						None => {
							let new_hash_vec = vec![team_hash];
							*hashes = Some(new_hash_vec);
							Ok(())
						}
					}
				})?;

                // The worker is also the team owner,
                // directly store worker acceptance
                Self::store_worker_acceptance(team_hash, who)?;
            } else {
                // the worker is not the team owner
                // The initial status of the acceptance to work on the team
                let accepted_status: AcceptAssignedStatus = false;

                // Adds team to list of teams assigned to worker address
                // Worker does not therefore need to be notified of new team assigned to them, as it will appear in
                // a list of teams
				WorkerTeamsBacklogList::<T>::try_mutate(&worker, |hashes| -> DispatchResult {
					match hashes {
						Some(ref mut hash_vec) => {
							hash_vec.push(team_hash);
							Ok(())
						},
						None => {
							let new_hash_vec = vec![team_hash];
							*hashes = Some(new_hash_vec);
							Ok(())
						}
					}
				})?;
                // set initial status
                WorkerTeamsBacklogStatus::<T>::insert(&status_tuple_key, accepted_status);

                // add worker to team team invitations, pending acceptance.
				TeamInvitesList::<T>::try_mutate(&team_hash, |teams| -> DispatchResult {
					match teams {
						Some(ref mut team_vec) => {
							team_vec.push(worker.clone());
							Ok(())
						},
						None => {
							let new_hash_vec = vec![worker.clone()];
							*teams = Some(new_hash_vec);
							Ok(())
						}
					}
				})?;
            }

            // issue event
            Self::deposit_event(Event::NotifyTeamWorker(worker, team_hash));

            Ok(().into())
        }

        /// Worker accepts to join the team.
        #[pallet::weight(0/*TODO*/)]
        pub fn worker_acceptance_team(
            origin: OriginFor<T>,
            team_hash: T::Hash,
            accepted: AcceptAssignedStatus,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            // check that this team is still active (not closed or deleted or with no status)
            ensure!(
                T::Teams::is_team_valid(team_hash),
                Error::<T>::TeamInactive
            );

            // check that the worker on this team is the signer
            Self::worker_teams_backlog_list(&who)
                .into_iter()
                .flatten()
                .find(|&x| x == team_hash)
                .ok_or(Error::<T>::WorkerNotAssigned)?;

            // Sets the new status of the acceptance to work on the team
            let status_tuple_key = (team_hash, who.clone());

            // Check that the team worker has accepted the team or rejected.
            if accepted {
                // let accepted_status: AcceptAssignedStatus = true;
                match Self::worker_teams_backlog_status(&status_tuple_key) {
                    // Worker confirms acceptance of team assignment. This effectively is an agreement that
                    // the team owner will accept time bookings from the worker as long as the team is still active.
                    Some(false) => Self::store_worker_acceptance(team_hash, who)?,
                    Some(true) => return Err(Error::<T>::WorkerAlreadyAcceptedTeam.into()),
                    None => return Err(Error::<T>::WorkerNotAssigned.into()),
                };
            } else {
                match Self::worker_teams_backlog_status(&status_tuple_key) {
                    // Only allow remove if the worker has been assigned this team,
                    // and that the status is unaccepted.
                    Some(false) => {
                        // Worker is removing this acceptance status
                        WorkerTeamsBacklogStatus::<T>::take(&status_tuple_key);

                        // Remove team assignment from list
                        WorkerTeamsBacklogList::<T>::mutate_or_err(
                            &who,
                            |worker_teams_backlog_list| {
                                worker_teams_backlog_list.retain(|h| h != &team_hash)
                            },
                        )?;

                        // remove from invitations list
                        TeamInvitesList::<T>::mutate_or_err(
                            &team_hash,
                            |team_invites_list| team_invites_list.retain(|h| h != &who),
                        )?;
                    }
                    Some(true) => return Err(Error::<T>::WorkerNotAssigned.into()),
                    None => return Err(Error::<T>::WorkerNotAssigned.into()),
                };
            }

            Ok(().into())
        }

        /// Worker submits/resubmits time record.
        #[pallet::weight(0/*TODO*/)]
        pub fn submit_time(
            origin: OriginFor<T>,
            team_hash: T::Hash,
            input_time_hash: T::Hash,
            submit_status: StatusOfTimeRecord,
            _reason_for_change: ReasonCodeStruct,
            number_of_blocks: NumberOfBlocks,
            _posting_period: PostingPeriod,
            start_block_number: StartOrEndBlockNumber,
            end_block_number: StartOrEndBlockNumber,
            break_counter: NumberOfBreaks,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            // Check that this team is still active (not closed or deleted or with no status)
            ensure!(
                T::Teams::is_team_valid(team_hash),
                Error::<T>::TeamInactive
            );

            // Check worker is not on the banned list
            let ban_list_key = (team_hash, who.clone());
            ensure!(
                !TeamWorkersBanList::<T>::contains_key(&ban_list_key),
                Error::<T>::WorkerBanned
            );
            // Check worker is part of the team
            let check_team_member = who.clone();
            Self::team_workers_list(team_hash)
                .into_iter()
                .flatten()
                .find(|x| x == &check_team_member)
                .ok_or(Error::<T>::WorkerNotAssigned)?;

            // For testing
            // let input_time_hash_2 = hex!("e4d673a76e8b32ca3989dbb9f444f71813c88d36120170b15151d58c7106cc83");
            // let default_hash: TimeHash = hex!("e4d673a76e8b32ca3989dbb9f444f71813c88d36120170b15151d58c7106cc83");
            // 0x6c9596f9ca96adf2334c4761bc161442a32ef16896427b6d43fc5e9353bbab63

            let default_bytes = "Default hash";
            let default_hash: T::Hash = T::Hashing::hash(&default_bytes.encode().as_slice()); // default hash BlakeTwo256

            // set default lock and reason code and type default values (TODO should come from extrinsic in future)
            let initial_submit_reason = ReasonCodeStruct(0, 0);
            let initial_reason_for_lock = ReasonCodeStruct(0, 0);

            // check that the submission is using either the default hash or some other hash.
            if input_time_hash == default_hash {
                // This is the default hash therefore it is a new submission.

                // prepare new time record
                let time_data = Timekeeper {
                    worker: who.clone(),
                    team_hash,
                    total_blocks: number_of_blocks,
                    locked_status: false,
                    locked_reason: initial_reason_for_lock,
                    submit_status: StatusOfTimeRecord::Submitted, // new record always gets status 1
                    reason_code: initial_submit_reason,
                    posting_period: 0, // temporary for this version of totem (meccano).
                    start_block: start_block_number,
                    end_block: end_block_number,
                    nr_of_breaks: break_counter,
                };

                // Create a new random hash
                let time_hash: T::Hash = time_data
                    .clone()
                    .using_encoded(<T as frame_system::Config>::Hashing::hash);

                // Now update all time relevant records
                WorkerTimeRecordsHashList::<T>::mutate_or_err(
                    &who,
                    |worker_time_records_hash_list| worker_time_records_hash_list.push(time_hash),
                )?;

                // Add time hash to team list
                TeamTimeRecordsHashList::<T>::mutate_or_err(
                    &team_hash,
                    |team_time_hash_list| team_time_hash_list.push(time_hash),
                )?;

                TimeHashOwner::<T>::insert(time_hash, who.clone());

                // Insert record
                TimeRecord::<T>::insert(time_hash, &time_data);
                Self::deposit_event(Event::SubmitedTimeRecord(time_hash));
            } else {
                // find out if this is a genuine original key
                let original_time_key = input_time_hash;

                let mut old_time_record = Self::time_record(&original_time_key)
                    .ok_or(Error::<T>::TimeRecordNotFromWorker)?;
                ensure!(
                    old_time_record.locked_status == false,
                    Error::<T>::TimeRecordLocked
                );

                // reverse out previously accepted time record
                Self::undo_update_totals(
                    old_time_record.worker.clone(),
                    old_time_record.team_hash,
                    old_time_record.total_blocks,
                )?;

                let proposed_new_status = submit_status.clone();

                // prepare incoming time record.
                let new_time_data = Timekeeper {
                    worker: who.clone(),
                    team_hash: team_hash,
                    total_blocks: number_of_blocks,
                    locked_status: false,
                    locked_reason: initial_reason_for_lock,
                    submit_status: submit_status,
                    reason_code: initial_submit_reason,
                    posting_period: 0, // not implemented in totem meccano
                    start_block: start_block_number,
                    end_block: end_block_number,
                    nr_of_breaks: break_counter,
                };

                // Possible states are
                // draft(0),
                // submitted(1),
                // disputed(100), can be resubmitted, if the current status is < 100 return this state
                // rejected(200), can be resubmitted, if the current status is < 100 return this state
                // accepted(300), can no longer be rejected or disputed, > 200 < 400
                // invoiced(400), can no longer be rejected or disputed, > 300 < 500
                // blocked(999),

                // Submit
                // team owner disputes, setting the state to 100... 100 can only be set if the current status is 0
                // team owner rejects, setting the state to 200... 200 can only be set if the current status is 0
                // Worker can resubmit time setting it back to 0... 0 can only be set if the current status < 300

                // team owner accepts time setting status to 300... 300 can only be set if the current status is 0 or 400 - a worker can invoice before acceptance
                // Team worker makes invoice. Worker can only create invoice if the current status is 0 or 300.

                // team owner response window expires

                match old_time_record.submit_status {
                    StatusOfTimeRecord::Draft => match proposed_new_status {
                        StatusOfTimeRecord::Draft | StatusOfTimeRecord::Submitted => {
                            ensure!(
                                old_time_record.worker == new_time_data.worker,
                                Error::<T>::TimeRecordNotOwned
                            );
                            old_time_record.submit_status = proposed_new_status;
                        }
                        // not appropriate to set these codes here. Other specific functions exist.
                        _ => return Err(Error::<T>::StatusNotImplementedOr.into()),
                    },
                    StatusOfTimeRecord::Submitted => return Err(Error::<T>::StatusAlreadySubmitted.into()),
                    StatusOfTimeRecord::Disputed | StatusOfTimeRecord::Rejected => {
                        // The existing record is rejected or disputed. The sender is therefore attempting to change the
                        // record. Only the worker can change the record.
                        // Ensure that the sender is the owner of the time record
                        ensure!(
                            old_time_record.worker == new_time_data.worker,
                            Error::<T>::TimeRecordNotOwned
                        );

                        match proposed_new_status {
                            StatusOfTimeRecord::Draft => {
                                old_time_record.submit_status = proposed_new_status
                            }
                            StatusOfTimeRecord::Submitted => {
                                ensure!(
                                    {
                                        old_time_record.total_blocks != new_time_data.total_blocks
                                            || old_time_record.start_block
                                                != new_time_data.start_block
                                            || old_time_record.end_block != new_time_data.end_block
                                            || old_time_record.posting_period
                                                != new_time_data.posting_period
                                            || old_time_record.nr_of_breaks
                                                != new_time_data.nr_of_breaks
                                    },
                                    Error::<T>::StatusIdentical
                                );

                                old_time_record.submit_status = proposed_new_status
                            } // Resubmitted.
                            // not appropriate to set these codes here. Other specific functions exist.
                            _ => return Err(Error::<T>::StatusCannotBeSetHere.into()),
                        }

                        // TODO remove any submitted reason codes.
                        // 0, 0 initial reason code is the default
                        old_time_record.reason_code = ReasonCodeStruct(0, 0);
                    }
                    StatusOfTimeRecord::Accepted => {
                        // The team owner has already accepted, but a correction is agreed with worker.
                        // therefore reset the record to "draft"
                        let hash_has_correct_owner =
                            T::Teams::is_owner_and_team_valid(who.clone(), team_hash);
                        ensure!(hash_has_correct_owner, Error::<T>::InvalidTeamOrOwner);

                        // ensure that a correct reason is given by team owner
                        // TODO inspect reason code values, change if necessary

                        // force change pending above
                        // [1, 1] = [time record can be re-edited by the team member, set in time module]
                        old_time_record.reason_code = ReasonCodeStruct(1, 1);

                        match proposed_new_status {
                            StatusOfTimeRecord::Draft => {
                                old_time_record.submit_status = proposed_new_status
                            } // Draft to submitted.
                            // not appropriate to set these codes here. Other specific functions exist.
                            _ => return Err(Error::<T>::StatusCannotBeSetHere.into()),
                        }
                    }
                    StatusOfTimeRecord::Invoiced => return Err(Error::<T>::TimeRecordAlreadyInvoiced.into()),
                    StatusOfTimeRecord::Blocked => return Err(Error::<T>::TimeBlocked.into()),
                };

                Self::update_time_record(
                    original_time_key,
                    // update all relevant fields from the incoming data
                    // setting status to submitted (1)
                    Timekeeper {
                        locked_status: false,
                        total_blocks: new_time_data.total_blocks,
                        start_block: new_time_data.start_block,
                        end_block: new_time_data.end_block,
                        posting_period: new_time_data.posting_period,
                        nr_of_breaks: new_time_data.nr_of_breaks,
                        ..old_time_record
                    },
                )?;
            }

            Ok(().into())
        }

        /// Team owner sets authorisation status of time record.
        #[pallet::weight(0/*TODO*/)]
        pub fn authorise_time(
            origin: OriginFor<T>,
            _worker: T::AccountId,
            team_hash: T::Hash,
            input_time_hash: T::Hash,
            status_of_record: StatusOfTimeRecord,
            _reason: ReasonCodeStruct,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            // ensure that the caller is the team owner
            let hash_has_correct_owner =
                T::Teams::is_owner_and_team_valid(who.clone(), team_hash);
            ensure!(hash_has_correct_owner, Error::<T>::InvalidTeamOrOwner);

            // prepare new time key
            let original_time_key = input_time_hash;

            // Check this is an existing time record
            // and get the details using the resubmitted hash
            let mut changing_time_record = Self::time_record(&original_time_key)
                .ok_or(Error::<T>::TimeRecordDoesNotExist)?;
            ensure!(
                !changing_time_record.locked_status,
                Error::<T>::TimeRecordLocked
            );

            let proposed_new_status = status_of_record.clone();

            match changing_time_record.submit_status {
                StatusOfTimeRecord::Draft => return Err(Error::<T>::TimeRecordNotFinalised.into()),
                StatusOfTimeRecord::Submitted => match proposed_new_status {
                    StatusOfTimeRecord::Disputed
                    | StatusOfTimeRecord::Rejected
                    | StatusOfTimeRecord::Accepted
                    | StatusOfTimeRecord::Blocked => {
                        // ensure that a correct reason is given by team owner
                        // TODO inpect reason code values
                        // new_time_data.reason_code = ReasonCodeStruct(1, 1);

                        changing_time_record.submit_status = proposed_new_status;
                    }
                    StatusOfTimeRecord::Draft | StatusOfTimeRecord::Invoiced => {
                        return Err(Error::<T>::TimeRecordCannotChange.into())
                    }
                    StatusOfTimeRecord::Submitted => return Err(Error::<T>::StatusNotImplemented.into()),
                },
                // The existing record is in a state that cannot be changed by the team owner.
                StatusOfTimeRecord::Disputed
                | StatusOfTimeRecord::Rejected
                | StatusOfTimeRecord::Accepted
                | StatusOfTimeRecord::Invoiced
                | StatusOfTimeRecord::Blocked => return Err(Error::<T>::TeamCannotBeChanged.into()),
            };

            TeamFirstSeen::<T>::insert(
                &changing_time_record.team_hash,
                changing_time_record.start_block,
            );

            // Perform update on total amounts of time
            Self::update_totals(
                changing_time_record.worker.clone(),
                changing_time_record.team_hash,
                changing_time_record.total_blocks,
            )?;

            Self::update_time_record(original_time_key, changing_time_record)?;
            Self::deposit_event(Event::SetAuthoriseStatus(who));

            Ok(().into())
        }

        /// Worker invoices the time record.
        #[pallet::weight(0/*TODO*/)]
        pub fn invoice_time(
            origin: OriginFor<T>,
            _team_hash: T::Hash,
            _input_time_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;
            // TODO This is normally set by the invoice module not by the time module
            // This needs to be reviewed once the invoice module is being developed.
            // Could be that this calls a function from within the invoice module.
            // can only invoice when time is accepted

            // Set StatusOfTimeRecord
            // invoiced,
            Self::deposit_event(Event::InvoiceTime(who));
            Ok(().into())
        }

        /// Team owner pays invoice.
        #[pallet::weight(0/*TODO*/)]
        pub fn pay_time(
            origin: OriginFor<T>,
            _team_hash: T::Hash,
            _input_time_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;

            Self::deposit_event(Event::PayTime(who));
            // Self::lock_time_record(who.clone(), team_hash, input_time_hash.clone());
            Self::deposit_event(Event::LockTimeRecord());

            Ok(().into())
        }

        /// Full payment triggers locked record.
        #[pallet::weight(0/*TODO*/)]
        pub fn lock_time_record(
            _origin: OriginFor<T>,
            _team_hash: T::Hash,
            _input_time_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(_origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            Self::deposit_event(Event::LockTimeRecord());

            Ok(().into())
        }

        /// In case of error unlock record.
        #[pallet::weight(0/*TODO*/)]
        pub fn unlock_time_record(
            _origin: OriginFor<T>,
            _team_hash: T::Hash,
            _input_time_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(_origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            Self::deposit_event(Event::UnLockTimeRecord());

            Ok(().into())
        }

        /// Worker or team member is banned from submitting time against this team.
        #[pallet::weight(0/*TODO*/)]
        pub fn ban_worker(
            _origin: OriginFor<T>,
            _team_hash: T::Hash,
            _worker: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(_origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            // check that you are not banning is not yourself!
            Self::deposit_event(Event::Banned());

            Ok(().into())
        }

        /// Worker or team member is released from ban from submitting time against this team.
        #[pallet::weight(0/*TODO*/)]
        pub fn unban_worker(
            _origin: OriginFor<T>,
            _team_hash: T::Hash,
            _worker: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(_origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            Self::deposit_event(Event::UnBanned());

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SubmitedTimeRecord(T::Hash),
        NotifyTeamWorker(T::AccountId, T::Hash),
        WorkerAcceptanceStatus(T::AccountId, T::Hash, AcceptAssignedStatus),
        SetAuthoriseStatus(T::AccountId),
        InvoiceTime(T::AccountId),
        PayTime(T::AccountId),
        LockTimeRecord(),
        UnLockTimeRecord(),
        Banned(),
        UnBanned(),
        IncreaseTotalBlocks(T::AccountId, T::Hash, NumberOfBlocks),
        DecreaseTotalBlocks(T::AccountId, T::Hash, NumberOfBlocks),
    }

    impl<T: Config> Pallet<T> {
        // TODO Move lock/unlock to private function

        /// When the worker accepts to work on the team, they are added to the team.
        fn store_worker_acceptance(
            team_hash: T::Hash,
            who: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let accepted_status: AcceptAssignedStatus = true;
            let status_tuple_key = (team_hash, who.clone());

            // add worker to team team
            TeamWorkersList::<T>::mutate_or_err(&team_hash, |team_workers_list| {
                team_workers_list.push(who.clone())
            })?;

            // Remove from notifications list
            TeamInvitesList::<T>::mutate(&team_hash, |team_invites_list| {
                Some(team_invites_list.as_mut()?.retain(|h| h != &who))
            });

            // set new status to true
            WorkerTeamsBacklogStatus::<T>::insert(status_tuple_key, &accepted_status);

            // issue event
            Self::deposit_event(Event::WorkerAcceptanceStatus(
                who,
                team_hash,
                accepted_status,
            ));

            Ok(().into())
        }

        /// Time record is removed (if it exists) and reinserted.
        fn update_time_record(
            k: T::Hash, // Time record hash
            d: Timekeeper<
                T::AccountId,
                T::Hash, // team record hash
                NumberOfBlocks,
                LockStatus,
                StatusOfTimeRecord,
                ReasonCodeStruct,
                PostingPeriod,
                StartOrEndBlockNumber,
                NumberOfBreaks,
            >,
        ) -> DispatchResultWithPostInfo {
            // store new time record
            TimeRecord::<T>::insert(&k, d);

            // issue event
            Self::deposit_event(Event::SubmitedTimeRecord(k));

            Ok(().into())
        }

        /// Updates the total number of blocks overall.
        ///
        /// Performs three main functions to update time storage:
        ///
        /// * Increments Total Time worked on a team for all workers
        /// * Increments Total Time worked by the worker for everything.
        /// * Increments Total Time booked for a specific worker on a specific team.
        fn update_totals(
            a: T::AccountId,
            r: T::Hash,
            n: NumberOfBlocks,
        ) -> DispatchResultWithPostInfo {
            TotalBlocksPerTeam::<T>::mutate(&r, |stored| match stored {
                Some(val) => *val += n,
                slot => *slot = Some(n),
            });

            TotalBlocksPerAddress::<T>::mutate(&a, |stored| match stored {
                Some(val) => *val += n,
                slot => *slot = Some(n),
            });

            TotalBlocksPerTeamPerAddress::<T>::mutate((&a, &r), |stored| match stored {
                Some(val) => *val += n,
                slot => *slot = Some(n),
            });

            Self::deposit_event(Event::IncreaseTotalBlocks(a, r, n));

            Ok(().into())
        }

        /// Performs reversal of total time booked against team and other storage:
        ///
        /// * Reduction in Total Time worked on a team for all workers.
        /// * Reduction in Total Time worked by the worker for everything.
        /// * Reduction in Total Time booked for a specific worker on a specific team.
        fn undo_update_totals(
            a: T::AccountId,
            r: T::Hash,
            n: NumberOfBlocks,
        ) -> DispatchResultWithPostInfo {
            // Check that the existing values are greater that the new value to be subtracted else do nothing.
            TotalBlocksPerTeam::<T>::mutate_or_err(&r, |val| {
                if *val >= n {
                    *val -= n
                }
            })?;

            TotalBlocksPerAddress::<T>::mutate_or_err(&a, |val| {
                if *val >= n {
                    *val -= n
                }
            })?;

            TotalBlocksPerTeamPerAddress::<T>::mutate_or_err((&a, &r), |val| {
                if *val >= n {
                    *val -= n
                }
            })?;

            Self::deposit_event(Event::DecreaseTotalBlocks(a, r, n));

            Ok(().into())
        }

        fn set_team_time_archive(
            time_hash: T::Hash,
            team_hash: T::Hash,
            archive: bool,
        ) -> DispatchResultWithPostInfo {
            // check if it's a retrieval or an archival process
            if archive {
                // Check that the time record does exist in the main record, otherwise don't update
                Self::team_time_records_hash_list(&team_hash)
                    .into_iter()
                    .flatten()
                    .find(|&x| x == time_hash)
                    .ok_or(Error::<T>::TimeRecordDoesNotExist)?;
                    // .ok_or("This record has either been archived already or does not exist!")?;

                // TODO Implement lock on record, then in other sections check the lock status.
                // Push to archive
                TeamTimeRecordsHashListArchive::<T>::mutate_or_err(
                    &team_hash,
                    |team_time_records_hash_list_archive| {
                        team_time_records_hash_list_archive.push(time_hash)
                    },
                )?;
                // Retain all others except
                TeamTimeRecordsHashList::<T>::mutate_or_err(
                    &team_hash,
                    |team_time_records_hash_list| {
                        team_time_records_hash_list.retain(|h| h != &time_hash)
                    },
                )?;
            } else {
                // Check that the time record does exist in the main record, otherwise don't update
                Self::team_time_records_hash_list_archive(&team_hash)
                    .into_iter()
                    .flatten()
                    .find(|&x| x == time_hash)
                    .ok_or(Error::<T>::TimeRecordDoesNotExist)?;
                    // .ok_or("This record has either been archived already or does not exist!")?;
                // TODO Implement unlock on record.
                // retrieve from archive
                TeamTimeRecordsHashList::<T>::mutate_or_err(
                    &team_hash,
                    |team_time_records_hash_list| team_time_records_hash_list.push(time_hash),
                )?;
                // remove from archive
                TeamTimeRecordsHashListArchive::<T>::mutate_or_err(
                    &team_hash,
                    |team_time_records_hash_list_archive| {
                        team_time_records_hash_list_archive.retain(|h| h != &time_hash)
                    },
                )?;
            }

            Ok(().into())
        }

        fn set_worker_time_archive(
            owner: T::AccountId,
            time_hash: T::Hash,
            archive: bool,
        ) -> DispatchResultWithPostInfo {
            // check if it's a retrieval or an archival process
            if archive {
                // Check that the time record does exist in the main record, otherwise don't update
                Self::worker_time_records_hash_list(&owner)
                    .into_iter()
                    .flatten()
                    .find(|&x| x == time_hash)
                    .ok_or(Error::<T>::TimeRecordDoesNotExist)?;
                    // .ok_or("This record has either been archived already or does not exist!")?;
                // TODO Implement lock on record, then in other sections check the lock status.
                // Push to archive
                WorkerTimeRecordsHashListArchive::<T>::mutate_or_err(
                    &owner,
                    |worker_time_records_hash_list_archive| {
                        worker_time_records_hash_list_archive.push(time_hash)
                    },
                )?;
                // Retain all others except
                WorkerTimeRecordsHashList::<T>::mutate_or_err(
                    &owner,
                    |worker_time_records_hash_list| {
                        worker_time_records_hash_list.retain(|h| h != &time_hash)
                    },
                )?;
            } else {
                // Check that the time record exists in the archive record, otherwise don't update
                Self::worker_time_records_hash_list_archive(&owner)
                    .into_iter()
                    .flatten()
                    .find(|&x| x == time_hash)
                    .ok_or(Error::<T>::TimeRecordDoesNotExist)?;
                    // .ok_or("This record has either been restored already or does not exist!")?;

                // TODO Implement unlock on record.

                // Retrieve from archive
                WorkerTimeRecordsHashList::<T>::mutate_or_err(
                    &owner,
                    |worker_time_records_hash_list| worker_time_records_hash_list.push(time_hash),
                )?;
                // Retain all others except
                WorkerTimeRecordsHashListArchive::<T>::mutate_or_err(
                    &owner,
                    |worker_time_records_hash_list_archive| {
                        worker_time_records_hash_list_archive.retain(|h| h != &time_hash)
                    },
                )?;
            }

            Ok(().into())
        }
    }

    impl<T: Config> Validating<T::AccountId, T::Hash> for Pallet<T> {
        /// Returns if `o` if the owner of the team.
        fn is_time_record_owner(o: T::AccountId, h: T::Hash) -> bool {
            Self::time_hash_owner(&h)
                .map(|owner| owner == o)
                .unwrap_or(false)
        }

        fn validate_and_archive(who: T::AccountId, h: T::Hash, a: bool) -> bool {
            match Self::time_record(h) {
                Some(old_time_record) => {
                    // Check the owner of the time record. If so process archive.
                    who == old_time_record.worker
                    && Self::set_worker_time_archive(who.clone(), h, a).is_ok()
                    // Attempt match on team owner to archive their own record.
                    && T::Teams::is_team_owner(who.clone(), old_time_record.team_hash)
                    && Self::set_team_time_archive(h, old_time_record.team_hash, a).is_ok()
                }
                None => false,
            }
        }
    }
}
