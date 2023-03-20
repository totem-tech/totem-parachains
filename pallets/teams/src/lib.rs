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

//! Totem Teams module.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
mod pallet {

    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
        traits::StorageVersion,
        sp_runtime::traits::BadOrigin,
    };
    use frame_system::pallet_prelude::*;

    use sp_std::prelude::*;

    use totem_common::StorageMapExt;
    use totem_primitives::teams::{DeletedTeam, TeamStatus, Validating};

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    /// Status of the team.
    #[pallet::storage]
    #[pallet::getter(fn team_hash_status)]
    pub type TeamHashStatus<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, TeamStatus>;

    /// List of deleted teams.
    #[pallet::storage]
    #[pallet::getter(fn deleted_team)]
    pub type DeletedTeams<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Vec<DeletedTeam<T::AccountId, TeamStatus>>>;

    /// Owner of the team.
    #[pallet::storage]
    #[pallet::getter(fn team_hash_owner)]
    pub type TeamHashOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::AccountId>;
    
    /// List of owned teams.
    #[pallet::storage]
    #[pallet::getter(fn owner_teams_list)]
    pub type OwnerTeamsList<T: Config> =
    StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;
    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// team has the wrong status to be changed.
        StatusWrong,
        /// The proposed team status is the same as the existing one.
        StatusSameProposed,
        /// The proposed team status cannot be applied to the current team status.
        StatusCannotApply,
        /// This proposed team status may not yet be implemented or is incorrect.
        StatusIncorrect,
        /// Error fetching team status.
        CannotFetchStatus,
        /// The team already exists.
        TeamAlreadyExists,
        /// The team does not exist.
        TeamDoesNotExist,
        /// The team was already deleted.
        AlreadyDeleted,
        /// Error fetching team owner.
        CannotFetchTeamOwner,
        /// You cannot reassign a team you do not own.
        CannotReassignNotOwned,
        /// You cannot close a team you do not own.
        CannotCloseNotOwned,
        /// You cannot change a team you do not own.
        TeamCannotChangeNotOwned,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0/*TODO*/)]
        pub fn add_new_team(
            origin: OriginFor<T>,
            team_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let who = ensure_signed(origin)?;
            // Check that the team does not exist
            ensure!(
                !TeamHashStatus::<T>::contains_key(team_hash),
                Error::<T>::TeamAlreadyExists);

            // Check that the team was not deleted already
            ensure!(
                !DeletedTeams::<T>::contains_key(team_hash),
                Error::<T>::AlreadyDeleted);

            // proceed to store team
            let team_status: TeamStatus = 0;

            // TODO limit nr of teams per Account.
            TeamHashStatus::<T>::insert(team_hash, &team_status);
            TeamHashOwner::<T>::insert(team_hash, &who);
            OwnerTeamsList::<T>::mutate_or_err(&who, |owner_teams_list| {
                owner_teams_list.push(team_hash)
            })?;

            Self::deposit_event(Event::TeamRegistered(team_hash, who));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn remove_team(
            origin: OriginFor<T>,
            team_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            // check transaction is signed.
            let changer: T::AccountId = ensure_signed(origin)?;
            
            ensure!(
                TeamHashStatus::<T>::contains_key(team_hash),
                Error::<T>::TeamDoesNotExist
            );
            // get team by hash
            let team_owner: T::AccountId = Self::team_hash_owner(team_hash)
                .ok_or(Error::<T>::CannotFetchTeamOwner)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(
                team_owner == changer,
                "You cannot delete a team you do not own"
            );

            let changed_by: T::AccountId = changer.clone();

            let deleted_team_struct = DeletedTeam {
                owned_by: team_owner.clone(),
                deleted_by: changed_by,
                status: 999,
            };

            // retain all other teams except the one we want to delete
            OwnerTeamsList::<T>::mutate_or_err(&team_owner, |owner_teams_list| {
                owner_teams_list.retain(|h| h != &team_hash)
            })?;

            // remove team from owner
            TeamHashOwner::<T>::remove(team_hash);

            // remove status record
            TeamHashStatus::<T>::remove(team_hash);

            // record the fact of deletion by whom
            DeletedTeams::<T>::mutate_or_err(team_hash, |deleted_team| {
                deleted_team.push(deleted_team_struct)
            })?;

            Self::deposit_event(Event::TeamDeleted(
                team_hash,
                team_owner,
                changer,
                999,
            ));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn reassign_team(
            origin: OriginFor<T>,
            new_owner: T::AccountId,
            team_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let changer: T::AccountId = ensure_signed(origin)?;
            ensure!(
                TeamHashStatus::<T>::contains_key(team_hash),
                Error::<T>::TeamDoesNotExist
            );

            // get team owner from hash
            let team_owner: T::AccountId = Self::team_hash_owner(team_hash)
                .ok_or(Error::<T>::CannotFetchTeamOwner)?;

            let changed_by: T::AccountId = changer.clone();

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(team_owner == changer, Error::<T>::CannotReassignNotOwned);

            // retain all other teams except the one we want to reassign
            OwnerTeamsList::<T>::mutate_or_err(&team_owner, |owner_teams_list| {
                owner_teams_list.retain(|h| h != &team_hash)
            })?;

            // Set new owner for hash
            TeamHashOwner::<T>::insert(team_hash, &new_owner);
            OwnerTeamsList::<T>::mutate_or_err(&new_owner, |owner_teams_list| {
                owner_teams_list.push(team_hash)
            })?;

            Self::deposit_event(Event::TeamReassigned(
                team_hash,
                new_owner,
                changed_by,
            ));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn close_team(
            origin: OriginFor<T>,
            team_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            let changer = ensure_signed(origin)?;
            ensure!(
                TeamHashStatus::<T>::contains_key(team_hash),
                Error::<T>::TeamDoesNotExist
            );

            // get team owner by hash
            let team_owner = Self::team_hash_owner(team_hash)
                .ok_or(Error::<T>::CannotFetchTeamOwner)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise onlu the owner can change the data
            ensure!(
                team_owner == changer,
                Error::<T>::CannotCloseNotOwned
            );
            let team_status: TeamStatus = 500;
            TeamHashStatus::<T>::insert(team_hash, &team_status);

            Self::deposit_event(Event::TeamChanged(team_hash, changer, team_status));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn reopen_team(
            origin: OriginFor<T>,
            team_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            // Can only reopen a team that is in status "closed"
            let changer = ensure_signed(origin)?;
            let team_status: TeamStatus = match Self::team_hash_status(team_hash) {
                Some(500) => 100,
                _ => return Err(Error::<T>::StatusWrong.into()),
                // None => return Err("Team has no status"),
            };


            // get team owner by hash
            let team_owner: T::AccountId = Self::team_hash_owner(team_hash)
                .ok_or(Error::<T>::CannotFetchTeamOwner)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(
                team_owner == changer,
                Error::<T>::TeamCannotChangeNotOwned
            );

            TeamHashStatus::<T>::insert(team_hash, &team_status);

            Self::deposit_event(Event::TeamChanged(team_hash, changer, team_status));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn set_status_team(
            origin: OriginFor<T>,
            team_hash: T::Hash,
            team_status: TeamStatus,
        ) -> DispatchResultWithPostInfo {
            if ensure_none(origin.clone()).is_ok() {
                return Err(BadOrigin.into())
            }
            ensure!(
                TeamHashStatus::<T>::contains_key(team_hash),
                Error::<T>::TeamDoesNotExist
            );

            let changer = ensure_signed(origin)?;

            // get team owner by hash
            let team_owner: T::AccountId = Self::team_hash_owner(team_hash)
                .ok_or(Error::<T>::CannotFetchTeamOwner)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(
                team_owner == changer,
                Error::<T>::TeamCannotChangeNotOwned
            );

            let current_team_status = Self::team_hash_status(team_hash)
                .ok_or(Error::<T>::CannotFetchStatus)?;
            // let proposed_team_status: TeamStatus = team_status.clone();
            let proposed_team_status = team_status.clone();

            //TODO this should be an enum
            // Open	0
            // Reopen	100
            // On Hold	200
            // Abandon	300
            // Cancel	400
            // Close	500
            // Delete	999

            // team owner creates team, set status to 0
            // team owner puts on hold, setting the state to 200... 200 can only be set if the current status is  <= 101
            // team owner abandons, setting the state to 300... 300 can only be set if the current status is  <= 101
            // team owner cancels, setting the state to 400... 400 can only be set if the current status is  <= 101
            // team owner close, setting the state to 500... 500 can only be set if the current status is  <= 101
            // team owner reopen, setting the state to 100... 100 can only be set if the current status is  200 || 300 || 500
            // team owner deletes, setting the state to 999... 999 cannot be set here.
            // team owner other, setting the state to other value... cannot be set here.

            match current_team_status {
                0 | 100 => {
                    // can set 200, 300, 400, 500
                    match proposed_team_status {
                        0 | 100 => return Err(Error::<T>::StatusWrong.into()),
                        200 | 300 | 400 | 500 => (),
                        _ => return Err(Error::<T>::StatusCannotApply.into()),
                    };
                }
                200 | 300 | 500 => {
                    // only set 100
                    match proposed_team_status {
                        100 => (),
                        _ => return Err(Error::<T>::StatusCannotApply.into()),
                    };
                }
                _ => return Err(Error::<T>::StatusCannotApply.into()),
            };

            let allowed_team_status: TeamStatus = proposed_team_status.into();

            TeamHashStatus::<T>::insert(team_hash, &allowed_team_status);

            Self::deposit_event(Event::TeamChanged(
                team_hash,
                changer,
                allowed_team_status,
            ));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TeamRegistered(T::Hash, T::AccountId),
        TeamDeleted(T::Hash, T::AccountId, T::AccountId, TeamStatus),
        TeamReassigned(T::Hash, T::AccountId, T::AccountId),
        TeamChanged(T::Hash, T::AccountId, TeamStatus),
    }

    impl<T: Config> Validating<T::AccountId, T::Hash> for Pallet<T> {
        fn is_team_owner(o: T::AccountId, h: T::Hash) -> bool {
            Self::team_hash_owner(h)
                .map(|owner| o == owner)
                .unwrap_or(false)
        }

        fn is_team_valid(h: T::Hash) -> bool {
            // check that the status of the team exists and is open or reopened.
            match Self::team_hash_status(h) {
                Some(0) | Some(100) => true,
                _ => false,
            }
        }

        fn is_owner_and_team_valid(o: T::AccountId, h: T::Hash) -> bool {
            //TODO
            // check validity of team
            Self::is_team_valid(h) && Self::is_team_owner(o, h)
        }
    }
}
