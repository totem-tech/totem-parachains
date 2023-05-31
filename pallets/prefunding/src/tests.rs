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
use crate::{mock::{new_test_ext, RuntimeOrigin, Test, Prefunding, Balances, Escrow}, PrefundingHashOwner, ReferenceStatus, Prefunding as PrefundingStorage};
use totem_primitives::escrow::EscrowableCurrency;
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use frame_support::pallet_prelude::DispatchResult;
use sp_core::H256;
use totem_primitives::orders::{ApprovalStatus, OrderHeader, OrderItem, TxKeysL, TxKeysM};
use totem_primitives::prefunding::LockStatus;

#[test]
fn should_prefund_someone_successfully() {
	new_test_ext().execute_with(|| {
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let amount = 1000000;
		let someone = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), someone, amount, deadline, tx_uid);
		assert_ok!(res);

		let latest_prefunding_hash_owner = PrefundingHashOwner::<Test>::iter().last();
		assert_eq!(latest_prefunding_hash_owner.is_some(), true);

		let latest_prefunding_storage = PrefundingStorage::<Test>::iter().last();
		assert_eq!(latest_prefunding_storage.is_some(), true);
	});
}

#[test]
fn should_prefund_someone_should_fail_with_beneficiary_error() {
	new_test_ext().execute_with(|| {
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let amount = 1000000;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), 1, amount, deadline, tx_uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [49, 0, 0, 0],
				message: Some("BeneficiaryError"),
			})
		);
	});
}

#[test]
fn should_prefund_someone_should_fail_with_prefund_not_set() {
	new_test_ext().execute_with(|| {
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let amount = u64::MAX;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), 2, amount, deadline, tx_uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [30, 0, 0, 0],
				message: Some("PrefundNotSet"),
			})
		);
	});
}

#[test]
fn should_prefund_someone_should_fail_with_short_deadline() {
	new_test_ext().execute_with(|| {
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let amount = u64::MAX;
		let deadline = 2;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), 2, amount, deadline, tx_uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [29, 0, 0, 0],
				message: Some("ShortDeadline"),
			})
		);
	});
}

#[test]
fn invoice_prefunded_order_successfully() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);
	});
}

#[test]
fn invoice_prefunded_fail_with_beneficiary_error() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(3), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [13, 0, 0, 0],
				message: Some("NotAllowed2"),
			})
		);
	});
}

#[test]
fn pay_prefunded_invoice_successfully() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		PrefundingHashOwner::<Test>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((1, LockStatus::Locked, 2, LockStatus::Locked));
		});

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let res = Prefunding::pay_prefunded_invoice(RuntimeOrigin::signed(1), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);
	});
}

#[test]
fn pay_prefunded_invoice_should_fail_with_not_allowed_3() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		PrefundingHashOwner::<Test>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((1, LockStatus::Locked, 2, LockStatus::Locked));
		});

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let res = Prefunding::pay_prefunded_invoice(RuntimeOrigin::signed(2), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [14, 0, 0, 0],
				message: Some("NotAllowed3"),
			})
		);
	});
}

#[test]
fn pay_prefunded_invoice_should_fail_with_unlocking() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		PrefundingHashOwner::<Test>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((1, LockStatus::Locked, 2, LockStatus::Locked));
		});

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		let res = Prefunding::pay_prefunded_invoice(RuntimeOrigin::signed(1), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [38, 0, 0, 0],
				message: Some("Unlocking"),
			})
		);
	});
}

#[test]
fn pay_prefunded_invoice_should_fail_with_not_approved_2() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let res = Prefunding::pay_prefunded_invoice(RuntimeOrigin::signed(1), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [19, 0, 0, 0],
				message: Some("NotApproved2"),
			})
		);
	});
}

#[test]
fn cancel_prefunded_closed_order_successfully() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		PrefundingHashOwner::<Test>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((1, LockStatus::Unlocked, 2, LockStatus::Unlocked));
		});

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let res = Prefunding::cancel_prefunded_closed_order(RuntimeOrigin::signed(1), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);
	});
}

#[test]
fn cancel_prefunded_closed_order_should_fail_with_hash_does_not_exist() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		PrefundingHashOwner::<Test>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((1, LockStatus::Unlocked, 2, LockStatus::Unlocked));
		});

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let res = Prefunding::cancel_prefunded_closed_order(RuntimeOrigin::signed(1), uid, uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [28, 0, 0, 0],
				message: Some("HashDoesNotExist3"),
			})
		);
	});
}

#[test]
fn cancel_prefunded_closed_order_should_fail_with_not_owner() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		PrefundingHashOwner::<Test>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((1, LockStatus::Unlocked, 2, LockStatus::Unlocked));
		});

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let res = Prefunding::cancel_prefunded_closed_order(RuntimeOrigin::signed(2), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [24, 0, 0, 0],
				message: Some("NotOwner2"),
			})
		);
	});
}

#[test]
fn cancel_prefunded_closed_order_should_fail_with_funds_in_play() {
	new_test_ext().execute_with(|| {
		let reference = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let uid = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let amount = 1000000;
		let payer = 2;
		let deadline = 200000;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Prefunding::prefund_someone(RuntimeOrigin::signed(1), payer, amount, deadline, reference);
		assert_ok!(res);

		let latest_prefunding_hash_owner_key = PrefundingHashOwner::<Test>::iter().next().map(|(key, _)| key);
		assert_eq!(latest_prefunding_hash_owner_key.is_some(), true);

		let latest_prefunding_hash_owner_value = PrefundingHashOwner::<Test>::iter().next();
		dbg!(latest_prefunding_hash_owner_value.unwrap());

		PrefundingHashOwner::<Test>::mutate(&latest_prefunding_hash_owner_key.unwrap(), |value| {
			*value = Some((1, LockStatus::Locked, 2, LockStatus::Locked));
		});

		let res = Prefunding::invoice_prefunded_order(RuntimeOrigin::signed(2), payer, amount.into(), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_ok!(res);

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let res = Prefunding::cancel_prefunded_closed_order(RuntimeOrigin::signed(1), latest_prefunding_hash_owner_key.unwrap(), uid);
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 3,
				error: [22, 0, 0, 0],
				message: Some("FundsInPlay2"),
			})
		);
	});
}

