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

    use frame_support::{fail, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    use sp_std::prelude::*;

    use totem_common::StorageMapExt;
    use totem_primitives::teams::{DeletedProject, ProjectStatus, Validating};

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(_);

    /// Status of the project.
    #[pallet::storage]
    #[pallet::getter(fn project_hash_status)]
    pub type ProjectHashStatus<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, ProjectStatus>;

    /// List of deleted projects.
    #[pallet::storage]
    #[pallet::getter(fn deleted_project)]
    pub type DeletedProjects<T: Config> =
        StorageMap<_, Blake2_128Concat, T::Hash, Vec<DeletedProject<T::AccountId, ProjectStatus>>>;

    /// Owner of the project.
    #[pallet::storage]
    #[pallet::getter(fn project_hash_owner)]
    pub type ProjectHashOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::AccountId>;

    /// List of owned projects.
    #[pallet::storage]
    #[pallet::getter(fn owner_projects_list)]
    pub type OwnerProjectsList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<T::Hash>>;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Project has the wrong status to be changed.
        StatusWrong,
        /// The proposed project status is the same as the existing one.
        StatusSameProposed,
        /// The proposed project status cannot be applied to the current project status.
        StatusCannotApply,
        /// This proposed project status may not yet be implemented or is incorrect.
        StatusIncorrect,
        /// Error fetching project status.
        ProjectCannotFetchStatus,
        /// The project already exists.
        ProjectAlreadyExists,
        /// The project does not exist.
        ProjectDoesNotExist,
        /// The project was already deleted.
        ProjectAlreadyDeleted,
        /// Error fetching project owner.
        ProjectCannotFetchOwner,
        /// You cannot reassign a project you do not own.
        ProjectCannotReassignNotOwned,
        /// You cannot close a project you do not own.
        ProjectCannotCloseNotOwned,
        /// You cannot change a project you do not own.
        ProjectCannotChangeNotOwned,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0/*TODO*/)]
        pub fn add_new_project(
            origin: OriginFor<T>,
            project_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // Check that the project does not exist
            ensure!(
                !ProjectHashStatus::<T>::contains_key(project_hash),
                Error::<T>::ProjectAlreadyExists
            );

            // Check that the project was not deleted already
            ensure!(
                !DeletedProjects::<T>::contains_key(project_hash),
                Error::<T>::ProjectAlreadyDeleted
            );

            // proceed to store project
            let who = ensure_signed(origin)?;
            let project_status: ProjectStatus = 0;

            // TODO limit nr of Projects per Account.
            ProjectHashStatus::<T>::insert(project_hash, &project_status);
            ProjectHashOwner::<T>::insert(project_hash, &who);
            OwnerProjectsList::<T>::mutate_or_err(&who, |owner_projects_list| {
                owner_projects_list.push(project_hash)
            })?;

            Self::deposit_event(Event::ProjectRegistered(project_hash, who));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn remove_project(
            origin: OriginFor<T>,
            project_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                ProjectHashStatus::<T>::contains_key(project_hash),
                Error::<T>::ProjectDoesNotExist
            );

            // get project by hash
            let project_owner: T::AccountId = Self::project_hash_owner(project_hash)
                .ok_or(Error::<T>::ProjectCannotFetchOwner)?;

            // check transaction is signed.
            let changer: T::AccountId = ensure_signed(origin)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(
                project_owner == changer,
                "You cannot delete a project you do not own"
            );

            let changed_by: T::AccountId = changer.clone();

            let deleted_project_struct = DeletedProject {
                owned_by: project_owner.clone(),
                deleted_by: changed_by,
                status: 999,
            };

            // retain all other projects except the one we want to delete
            OwnerProjectsList::<T>::mutate_or_err(&project_owner, |owner_projects_list| {
                owner_projects_list.retain(|h| h != &project_hash)
            })?;

            // remove project from owner
            ProjectHashOwner::<T>::remove(project_hash);

            // remove status record
            ProjectHashStatus::<T>::remove(project_hash);

            // record the fact of deletion by whom
            DeletedProjects::<T>::mutate_or_err(project_hash, |deleted_project| {
                deleted_project.push(deleted_project_struct)
            })?;

            Self::deposit_event(Event::ProjectDeleted(
                project_hash,
                project_owner,
                changer,
                999,
            ));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn reassign_project(
            origin: OriginFor<T>,
            new_owner: T::AccountId,
            project_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                ProjectHashStatus::<T>::contains_key(project_hash),
                Error::<T>::ProjectDoesNotExist
            );

            // get project owner from hash
            let project_owner: T::AccountId = Self::project_hash_owner(project_hash)
                .ok_or(Error::<T>::ProjectCannotFetchOwner)?;

            let changer: T::AccountId = ensure_signed(origin)?;
            let changed_by: T::AccountId = changer.clone();

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(
                project_owner == changer,
                Error::<T>::ProjectCannotReassignNotOwned
            );

            // retain all other projects except the one we want to reassign
            OwnerProjectsList::<T>::mutate_or_err(&project_owner, |owner_projects_list| {
                owner_projects_list.retain(|h| h != &project_hash)
            })?;

            // Set new owner for hash
            ProjectHashOwner::<T>::insert(project_hash, &new_owner);
            OwnerProjectsList::<T>::mutate_or_err(&new_owner, |owner_projects_list| {
                owner_projects_list.push(project_hash)
            })?;

            Self::deposit_event(Event::ProjectReassigned(
                project_hash,
                new_owner,
                changed_by,
            ));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn close_project(
            origin: OriginFor<T>,
            project_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                ProjectHashStatus::<T>::contains_key(project_hash),
                Error::<T>::ProjectDoesNotExist
            );

            let changer = ensure_signed(origin)?;

            // get project owner by hash
            let project_owner = Self::project_hash_owner(project_hash)
                .ok_or(Error::<T>::ProjectCannotFetchOwner)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise onlu the owner can change the data
            ensure!(
                project_owner == changer,
                Error::<T>::ProjectCannotCloseNotOwned
            );
            let project_status: ProjectStatus = 500;
            ProjectHashStatus::<T>::insert(project_hash, &project_status);

            Self::deposit_event(Event::ProjectChanged(project_hash, changer, project_status));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn reopen_project(
            origin: OriginFor<T>,
            project_hash: T::Hash,
        ) -> DispatchResultWithPostInfo {
            // Can only reopen a project that is in status "closed"
            let project_status: ProjectStatus = match Self::project_hash_status(project_hash) {
                Some(500) => 100,
                _ => fail!(Error::<T>::StatusWrong),
                // None => return Err("Project has no status"),
            };

            let changer = ensure_signed(origin)?;

            // get project owner by hash
            let project_owner: T::AccountId = Self::project_hash_owner(project_hash)
                .ok_or(Error::<T>::ProjectCannotFetchOwner)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(
                project_owner == changer,
                Error::<T>::ProjectCannotChangeNotOwned
            );

            ProjectHashStatus::<T>::insert(project_hash, &project_status);

            Self::deposit_event(Event::ProjectChanged(project_hash, changer, project_status));

            Ok(().into())
        }

        #[pallet::weight(0/*TODO*/)]
        pub fn set_status_project(
            origin: OriginFor<T>,
            project_hash: T::Hash,
            project_status: ProjectStatus,
        ) -> DispatchResultWithPostInfo {
            ensure!(
                ProjectHashStatus::<T>::contains_key(project_hash),
                Error::<T>::ProjectDoesNotExist
            );

            let changer = ensure_signed(origin)?;

            // get project owner by hash
            let project_owner: T::AccountId = Self::project_hash_owner(project_hash)
                .ok_or(Error::<T>::ProjectCannotFetchOwner)?;

            // TODO Implement a sudo for cleaning data in cases where owner is lost
            // Otherwise only the owner can change the data
            ensure!(
                project_owner == changer,
                Error::<T>::ProjectCannotChangeNotOwned
            );

            let current_project_status = Self::project_hash_status(project_hash)
                .ok_or(Error::<T>::ProjectCannotFetchStatus)?;
            // let proposed_project_status: ProjectStatus = project_status.clone();
            let proposed_project_status = project_status.clone();

            //TODO this should be an enum
            // Open	0
            // Reopen	100
            // On Hold	200
            // Abandon	300
            // Cancel	400
            // Close	500
            // Delete	999

            // Project owner creates project, set status to 0
            // Project owner puts on hold, setting the state to 200... 200 can only be set if the current status is  <= 101
            // Project owner abandons, setting the state to 300... 300 can only be set if the current status is  <= 101
            // Project owner cancels, setting the state to 400... 400 can only be set if the current status is  <= 101
            // Project owner close, setting the state to 500... 500 can only be set if the current status is  <= 101
            // Project owner reopen, setting the state to 100... 100 can only be set if the current status is  200 || 300 || 500
            // Project owner deletes, setting the state to 999... 999 cannot be set here.
            // Project owner other, setting the state to other value... cannot be set here.

            match current_project_status {
                0 | 100 => {
                    // can set 200, 300, 400, 500
                    match proposed_project_status {
                        0 | 100 => fail!(Error::<T>::StatusWrong),
                        200 | 300 | 400 | 500 => (),
                        _ => fail!(Error::<T>::StatusCannotApply),
                    };
                }
                200 | 300 | 500 => {
                    // only set 100
                    match proposed_project_status {
                        100 => (),
                        _ => fail!(Error::<T>::StatusCannotApply),
                    };
                }
                _ => fail!(Error::<T>::StatusCannotApply),
            };

            let allowed_project_status: ProjectStatus = proposed_project_status.into();

            ProjectHashStatus::<T>::insert(project_hash, &allowed_project_status);

            Self::deposit_event(Event::ProjectChanged(
                project_hash,
                changer,
                allowed_project_status,
            ));

            Ok(().into())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ProjectRegistered(T::Hash, T::AccountId),
        ProjectDeleted(T::Hash, T::AccountId, T::AccountId, ProjectStatus),
        ProjectReassigned(T::Hash, T::AccountId, T::AccountId),
        ProjectChanged(T::Hash, T::AccountId, ProjectStatus),
    }

    impl<T: Config> Validating<T::AccountId, T::Hash> for Pallet<T> {
        fn is_project_owner(o: T::AccountId, h: T::Hash) -> bool {
            Self::project_hash_owner(h)
                .map(|owner| o == owner)
                .unwrap_or(false)
        }

        fn is_project_valid(h: T::Hash) -> bool {
            // check that the status of the project exists and is open or reopened.
            match Self::project_hash_status(h) {
                Some(0) | Some(100) => true,
                _ => false,
            }
        }

        fn is_owner_and_project_valid(o: T::AccountId, h: T::Hash) -> bool {
            //TODO
            // check validity of project
            Self::is_project_valid(h) && Self::is_project_owner(o, h)
        }
    }
}
