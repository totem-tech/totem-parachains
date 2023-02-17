//! # Unit-Of-Account
//!
//! A module for calculating unit of account based on issued/stored currencies
//!
//! ## Overview
//!
//! The Unit-of-accounting module provides functionality for the following:
//!
//! * Add whitelisted account
//! * Remove whitelisted account
//! * Add new currency
//! * Remove currency
//!
//! The supported dispatchable functions are documented in the [`Call`] enum.
//!
//! ### Goals
//!
//! The Unit-of-Account in Totem is designed to make the following possible:
//!
//! * Add a new currency to the basket of currencies and then generate/calulate the weight and PEER of currencies
//!
//! ## UnitOfAccountInterface Interface
//!
//! `add_currency`: Adds a currency with it's issuance to the basket.
//!
//! `remove_currency`: Removes a currency from the basket.
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{pallet_prelude::DispatchError, BoundedVec};
use sp_std::{
	convert::{TryFrom, TryInto},
	prelude::*,
};
use totem_primitives::unit_of_account::{UnitOfAccountCurrency, UnitOfAccountInterface};

pub use pallet::*;
use totem_primitives::LedgerBalance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use totem_primitives::LedgerBalance;
	use totem_primitives::unit_of_account::UnitOfAccountCurrency;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]

	pub struct Pallet<T, I = ()>(_);

	#[pallet::config]
	/// The module configuration trait.
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self, I>>
		+ IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The max length of a whitelisted account
		type MaxWhitelistedAccounts: Get<u32>;

		/// The max number of currencies allowed in the basket
		type MaxCurrencyStringLength: Get<u32>;
	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub phantom: (PhantomData<T>, PhantomData<I>),
	}

	#[cfg(feature = "std")]
	impl<T: Config<I>, I: 'static> Default for GenesisConfig<T, I> {
		fn default() -> Self {
			Self { phantom: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> GenesisBuild<T, I> for GenesisConfig<T, I> {
		fn build(&self) {}
	}

	/// The list of whitelisted accounts
	#[pallet::storage]
	#[pallet::getter(fn whitelisted_accounts)]
	pub type WhitelistedAccounts<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		()
	>;


	/// The map of currency(represented in bytes) to the issuance and the weight of the currency
	#[pallet::storage]
	#[pallet::getter(fn currency_basket)]
	pub type CurrencyBasket<T: Config<I>, I: 'static = ()> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxCurrencyStringLength>,
		UnitOfAccountCurrency,
	>;

	/// The calculated Nominal Effective Exchange Rate which is
	/// also known as the unit of account
	#[pallet::storage]
	#[pallet::getter(fn neer)]
	pub type Neer<T: Config<I>, I: 'static = ()> = StorageValue<
		_,
		LedgerBalance,
		ValueQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// Account Whitelisted
		AccountWhitelisted(T::AccountId),
		/// Account removed from whitelisted accounts
		AccountRemoved(T::AccountId),
		/// Currency added to the basket
		CurrencyAddedToBasket(Vec<u8>),
		/// Currency removed from the basket
		CurrencyRemovedFromTheBasket(Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// Whitelisted account out of bounds
		AlreadyWhitelistedAccount,
		/// Unknown whitelisted account
		UnknownWhitelistedAccount,
		/// Max currencies exceeded
		CurrencyStringLengthOutOfBound,
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {}

	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::weight(0)]
		#[pallet::call_index(0)]
		pub fn whitelist_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(account.clone());
			ensure!(already_whitelisted.is_some(), Error::<T, I>::AlreadyWhitelistedAccount);

			<WhitelistedAccounts<T, I>>::insert(account.clone(), ());

			Self::deposit_event(Event::AccountWhitelisted(account.clone()));
			Ok(().into())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(1)]
		pub fn remove_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(account.clone());
			ensure!(already_whitelisted == None, Error::<T, I>::UnknownWhitelistedAccount);

			<WhitelistedAccounts<T, I>>::remove(account.clone());

			Self::deposit_event(Event::AccountRemoved(account));

			Ok(().into())
		}

		#[pallet::weight(0)]
		#[pallet::call_index(2)]
		pub fn add_currency(
			origin: OriginFor<T>,
			currency: Vec<u8>,
			issuance: LedgerBalance,
			price: LedgerBalance
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(whitelisted_caller.clone());
			ensure!(already_whitelisted == None, Error::<T, I>::UnknownWhitelistedAccount);

			<Self as UnitOfAccountInterface>::add_currency(currency, issuance, price)?;

			Ok(().into())
		}
	}
}

impl<T: Config<I>, I: 'static> UnitOfAccountInterface for Pallet<T, I> {
	fn add_currency(currency: Vec<u8>, issuance: LedgerBalance, price: LedgerBalance) -> Result<(), DispatchError> {
		let currency_issuance_inverse = 1 / issuance;

		let unit_of_account_in_currency_basket: Vec<UnitOfAccountCurrency> =  CurrencyBasket::<T, I>::iter()
			.map(|(_, v)| v)
			.collect();

		let total_weights_in_currency_basket = unit_of_account_in_currency_basket.iter()
			.fold(0, |acc, unit| 1/acc + 1/unit.issuance);

		let weight_of_currency = currency_issuance_inverse / total_weights_in_currency_basket;

		let unit_of_account_currency = UnitOfAccountCurrency {
			issuance,
			price,
			weight: Some(weight_of_currency)
		};

		let bounded_currency = BoundedVec::<u8, T::MaxCurrencyStringLength>::try_from(currency.clone())
			.map_err(|_e| Error::<T, I>::CurrencyStringLengthOutOfBound)?;



		CurrencyBasket::<T, I>::insert(bounded_currency, unit_of_account_currency);

		Ok(())
	}

	fn remove_currency(currency: Vec<u8>) -> Result<(), DispatchError> {
		Ok(())
	}
}
