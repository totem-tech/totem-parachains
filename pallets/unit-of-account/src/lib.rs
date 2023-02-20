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
use totem_primitives::unit_of_account::{DIVISOR_UNIT, CurrencyDetails, UnitOfAccountInterface};

pub use pallet::*;
use totem_primitives::LedgerBalance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use totem_primitives::LedgerBalance;
	use totem_primitives::unit_of_account::CurrencyDetails;

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
		CurrencyDetails,
	>;

	/// The calculated Nominal Effective Exchange Rate which is
	/// also known as the unit of account
	#[pallet::storage]
	#[pallet::getter(fn unit_of_account)]
	pub type UnitOfAccount<T: Config<I>, I: 'static = ()> = StorageValue<
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
		CurrencySymbolLengthOutOfBound,
		/// Currency bnot found from basket
		CurrencyNotFoundFromBasket
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
			symbol: Vec<u8>,
			issuance: LedgerBalance,
			price: LedgerBalance
		) -> DispatchResultWithPostInfo {
			let whitelisted_caller = ensure_signed(origin)?;

			let already_whitelisted = Self::whitelisted_accounts(whitelisted_caller.clone());
			ensure!(already_whitelisted == None, Error::<T, I>::UnknownWhitelistedAccount);

			<Self as UnitOfAccountInterface>::add_currency(symbol, issuance, price)?;

			Ok(().into())
		}
	}
}

impl<T: Config<I>, I: 'static> UnitOfAccountInterface for Pallet<T, I> {
	fn add_currency(symbol: Vec<u8>, issuance: LedgerBalance, price: LedgerBalance) -> Result<(), DispatchError> {
		let currency_issuance_inverse = 1 / issuance;

		let unit_of_account_in_currency_basket: Vec<CurrencyDetails> =  CurrencyBasket::<T, I>::iter()
			.map(|(_, v)| v)
			.collect();

		let total_weights_in_currency_basket = unit_of_account_in_currency_basket.iter()
			.fold(0, |acc, unit| acc + 1/unit.issuance);

		let weight_of_currency = currency_issuance_inverse / total_weights_in_currency_basket;

		let unit_of_account_currency = CurrencyDetails {
			issuance,
			price,
			weight: Some(weight_of_currency),
			weight_adjusted_price: None,
			unit_of_account: None
		};

		let bounded_currency = BoundedVec::<u8, T::MaxCurrencyStringLength>::try_from(symbol.clone())
			.map_err(|_e| Error::<T, I>::CurrencySymbolLengthOutOfBound)?;

		CurrencyBasket::<T, I>::insert(bounded_currency, unit_of_account_currency);

		Ok(())
	}

	fn remove_currency(currency: Vec<u8>) -> Result<(), DispatchError> {
		Ok(())
	}
}

impl<T: Config<I>, I: 'static> Pallet<T, I> {
	pub fn calculate_individual_weights() {
		let  total_inverse_in_currency_basket = Self::calculate_total_inverse_issuance_in_basket();

		for (bounded_currency, unit_of_account_currency) in CurrencyBasket::<T, I>::iter() {
			let issuance = unit_of_account_currency.issuance;
			let inverse_issuance = 1/issuance;
			let currency_weight = inverse_issuance / total_inverse_in_currency_basket;
			let weight_adjusted_price = currency_weight * unit_of_account_currency.price;

			CurrencyBasket::<T, I>::try_mutate(bounded_currency, |maybe_unit_of_account_currency| ->  Result<(), DispatchError> {
				let mut unit_of_account_currency = maybe_unit_of_account_currency.as_mut().ok_or(Error::<T, I>::CurrencyNotFoundFromBasket)?;

				unit_of_account_currency.weight = Some(currency_weight);
				unit_of_account_currency.weight_adjusted_price = Some(weight_adjusted_price);

				Ok(())
			});
		}


	}

	pub fn calculate_total_inverse_issuance_in_basket() -> LedgerBalance  {
		let unit_of_account_in_currency_basket: Vec<CurrencyDetails> =  CurrencyBasket::<T, I>::iter()
			.map(|(_, v)| v)
			.collect();

		let total_inverse_in_currency_basket = unit_of_account_in_currency_basket.iter()
			.fold(0, |acc, unit| acc + 1/unit.issuance);

		total_inverse_in_currency_basket
	}

	pub fn calculate_unit_of_account() -> LedgerBalance  {
		let unit_of_account_in_currency_basket: Vec<CurrencyDetails> =  CurrencyBasket::<T, I>::iter()
			.map(|(_, v)| v)
			.collect();

		let unit_of_account  =  unit_of_account_in_currency_basket.iter()
			.fold(0, |acc, unit| acc + (unit.weight.unwrap() * unit.price));

		unit_of_account
	}

	pub fn calculate_individual_currency_unit_of_account() {
		let  unit_of_account = Self::calculate_unit_of_account();

		for (bounded_currency, unit_of_account_currency) in CurrencyBasket::<T, I>::iter() {
			CurrencyBasket::<T, I>::try_mutate(bounded_currency, |maybe_unit_of_account_currency| ->  Result<(), DispatchError> {
				let mut unit_of_account_currency = maybe_unit_of_account_currency.as_mut().ok_or(Error::<T, I>::CurrencyNotFoundFromBasket)?;

				let unit_of_account_for_currency = unit_of_account_currency.price / (DIVISOR_UNIT * unit_of_account);
				unit_of_account_currency.unit_of_account = Some(unit_of_account_for_currency);

				Ok(())
			});
		}


	}
}
