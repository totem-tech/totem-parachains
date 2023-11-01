// This is a Pallet lib.rs file boilerplate to ensure that the structure of pallets are consistent

#![cfg_attr(not(feature = "std"), no_std)]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
mod pallet {
    use frame_support::{}
    use frame_system::pallet_prelude::*;
    use sp_std::prelude::*;

    pub use weights::WeightInfo;

    use totem_primitives::{}

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn posting_number)]
    pub type PostingNumber<T: Config> = StorageValue<
    _, 
    PostingIndex, 
    ValueQuery
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {}

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {}

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {}

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::config]
    pub trait Config: frame_system::Config {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {}
    
    // Other Traits example from Accounting
    impl<T: Config> Posting<T::AccountId, T::Hash, T::BlockNumber, CurrencyBalanceOf<T>> for Pallet<T>
    where 
        T: pallet_timestamp::Config,
    {}

}