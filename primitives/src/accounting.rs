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

mod chart_of_accounts;
// pub use chart_of_accounts::{Ledger, {CurrentAssets, Sales, OperatingExpenses, _0030_, B,A,P,I,X,Cogs,Commissions,_0009_}};
pub use chart_of_accounts::{Ledger, *};
    
use crate::LedgerBalance;
use frame_support::{dispatch::EncodeLike, pallet_prelude::*};
use scale_info::TypeInfo;
use sp_runtime::traits::Member;
use sp_std::prelude::*;

/// Main Totem accounting trait.
pub trait Posting<AccountId, Hash, BlockNumber, CoinAmount> {
    type PostingIndex: Member + Copy + Into<u128> + Encode + Decode + Eq;

    fn handle_multiposting_amounts(
        keys: &[Record<AccountId, Hash, BlockNumber>],
    ) -> DispatchResultWithPostInfo;

    fn account_for_simple_transfer(
        from: AccountId,
        to: AccountId,
        amount: CoinAmount,
    ) -> DispatchResultWithPostInfo;

    fn account_for_fees(fee: CoinAmount, payer: AccountId) -> DispatchResultWithPostInfo;
    fn account_for_burnt_fees(fee: CoinAmount, loser: AccountId) -> DispatchResultWithPostInfo;
    fn distribute_fees_rewards(fee: CoinAmount, author: AccountId) -> DispatchResultWithPostInfo;

    fn get_escrow_account() -> AccountId;
    
    fn get_netfees_account() -> AccountId;

    fn get_pseudo_random_hash(s: AccountId, r: AccountId) -> Hash;
}

/// Debit or Credit Indicator
/// Debit and Credit balances are account specific - see chart of accounts.
#[derive(Clone, Decode, Encode, Copy, TypeInfo)]
#[scale_info(capture_docs = "always")]
pub enum Indicator {
    /// Debit
    Debit = 0,
    /// Credit
    Credit = 1,
}

#[derive(Clone, Decode, Encode, TypeInfo)]
pub struct Record<AccountId, Hash, BlockNumber> {
    pub primary_party: AccountId,
    pub counterparty: AccountId,
    pub ledger: Ledger,
    pub amount: LedgerBalance,
    pub debit_credit: Indicator,
    pub reference_hash: Hash,
    pub changed_on_blocknumber: BlockNumber,
    pub applicable_period_blocknumber: BlockNumber,
}

// Implementations

impl EncodeLike<Indicator> for bool {}

impl Indicator {
    pub fn reverse(self) -> Self {
        match self {
            Self::Debit => Self::Credit,
            Self::Credit => Self::Debit,
        }
    }
}

#[cfg(any(test, feature = "mock"))]
impl<AccountId, Hash, BlockNumber, CoinAmount> Posting<AccountId, Hash, BlockNumber, CoinAmount>
    for ()
{
    type PostingIndex = u128;

    fn handle_multiposting_amounts(
        _fwd: &[Record<AccountId, Hash, BlockNumber>],
    ) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn account_for_simple_transfer(
        _from: AccountId,
        _to: AccountId,
        _amount: CoinAmount,
    ) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn account_for_fees(_f: CoinAmount, _p: AccountId) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn account_for_burnt_fees(_f: CoinAmount, _p: AccountId) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }
    
    fn distribute_fees_rewards(_f: CoinAmount, _p: AccountId) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn account_for_burnt_fees(_f: CoinAmount, _p: AccountId) -> DispatchResultWithPostInfo {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn get_escrow_account() -> AccountId {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn get_netfees_account() -> AccountId {
        unimplemented!("Used as a mock, shouldn't be called")
    }

    fn get_pseudo_random_hash(_s: AccountId, _r: AccountId) -> Hash {
        unimplemented!("Used as a mock, shouldn't be called")
    }
}