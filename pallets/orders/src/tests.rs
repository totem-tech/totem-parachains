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
use crate::{mock::{new_test_ext, RuntimeOrigin, Orders, Test, Balances, Escrow}, Orders as OrderStorage};
use totem_primitives::escrow::EscrowableCurrency;
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use frame_support::pallet_prelude::DispatchResult;
use sp_core::H256;
use pallet_prefunding::{PrefundingHashOwner, ReferenceStatus};
use totem_primitives::orders::{ApprovalStatus, OrderHeader, OrderItem, TxKeysL, TxKeysM};
use totem_primitives::prefunding::LockStatus;

#[test]
fn should_create_order_successfully_when_market_order_is_false() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};
		OrderStorage::<Test>::insert(&parent_id, order_header);

		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large );
		assert_ok!(res);

		assert_eq!(OrderStorage::<Test>::get(&record_id).is_some(),  true);
	});
}

#[test]
fn should_create_order_successfully_when_market_order_is_true() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = true;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large );
		assert_ok!(res);
	});
}

#[test]
fn should_create_order_should_fail_with_hash_exists() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};
		OrderStorage::<Test>::insert(&record_id, order_header);

		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [1, 0, 0, 0],
				message: Some("HashExists"),
			})
		);
	});
}

#[test]
fn should_create_order_should_fail_with_market_order() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = true;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [32, 0, 0, 0],
				message: Some("MarketOrder"),
			})
		);
	});
}

#[test]
fn should_create_order_should_fail_when_cannot_be_both_2() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};
		OrderStorage::<Test>::insert(&parent_id, order_header);

		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, 1, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [5, 0, 0, 0],
				message: Some("CannotBeBoth2"),
			})
		);
	});
}

#[test]
fn should_create_order_should_fail_when_hash_exists_2() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;


		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [2, 0, 0, 0],
				message: Some("HashExists2"),
			})
		);
	});
}

#[test]
fn should_delete_order_successfully() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&parent_id, order_header);

		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large.clone() );
		assert_ok!(res);

		assert_eq!(OrderStorage::<Test>::get(&record_id).is_some(),  true);

		let tx_uid = H256::from_slice("01234567890123456789012345678902".as_bytes());

		let tx_keys_medium = TxKeysM {
			record_id,
			bonsai_token,
			tx_uid
		};

		assert_ok!(Orders::delete_order(RuntimeOrigin::signed(1), tx_keys_medium));
		assert_eq!(OrderStorage::<Test>::get(&record_id).is_some(),  false);
	});
}

#[test]
fn should_delete_order_should_fail_with_status_not_allowed_6() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 1u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&parent_id, order_header);


		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large.clone() );
		assert_ok!(res);

		let tx_uid = H256::from_slice("01234567890123456789012345678902".as_bytes());

		let tx_keys_medium = TxKeysM {
			record_id,
			bonsai_token,
			tx_uid
		};

		assert_err!(
			Orders::delete_order(RuntimeOrigin::signed(2), tx_keys_medium),
			DispatchError::Module(ModuleError {
				index: 2,
				error: [17, 0, 0, 0],
				message: Some("StatusNotAllowed6"),
			})
		);
	});
}

#[test]
fn should_delete_order_should_fail_with_hash_exists_3() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let mut order_items = Vec::new();
		order_items.push(order_item);

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let tx_keys_large = TxKeysL {
			record_id,
			parent_id,
			bonsai_token,
			tx_uid
		};

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 1u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&parent_id, order_header);


		let res = Orders::create_order(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_items, tx_keys_large.clone() );
		assert_ok!(res);

		let tx_uid = H256::from_slice("01234567890123456789012345678902".as_bytes());

		let tx_keys_medium = TxKeysM {
			record_id: H256::from_slice("01234567890123456789012345678903".as_bytes()),
			bonsai_token,
			tx_uid
		};

		assert_err!(
			Orders::delete_order(RuntimeOrigin::signed(1), tx_keys_medium),
			DispatchError::Module(ModuleError {
				index: 2,
				error: [3, 0, 0, 0],
				message: Some("HashExists3"),
			})
		);
	});
}

#[test]
fn should_create_spfso_successfully_if_who_is_not_fulfiller() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};

		let record_id = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let parent_id = H256::from_slice("01234567890123456789012345678902".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 0;
		let due_date = 0;

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), approver, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_ok!(res);
	});
}

#[test]
fn should_create_spfso_successfully_if_who_is_fulfiller() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), 1, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_ok!(res);
	});
}

#[test]
fn should_create_spfso_should_fail_with_cannot_be_both_error() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), 1, 1, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [4, 0, 0, 0],
				message: Some("CannotBeBoth"),
			})
		);
	});
}

#[test]
fn should_create_spfso_should_fail_with_amount_overflow_error() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1000000000000000000000000;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), 1, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [33, 0, 0, 0],
				message: Some("AmountOverflow"),
			})
		);
	});
}

#[test]
fn should_create_spfso_should_fail_with_in_prefunding_error() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1000;
		let market_order = false;
		let order_type = 1;
		let deadline = 20;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), 1, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [25, 0, 0, 0],
				message: Some("InPrefunding1"),
			})
		);
	});
}

#[test]
fn should_change_spfso_successfully_if_who_is_fulfiller() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), approver, fulfiller.clone(), buy_or_sell.clone(), total_amount.clone(), market_order.clone(), order_type.clone(), deadline.clone(), due_date.clone(), order_item.clone(), bonsai_token.clone(), tx_uid.clone() );
		assert_ok!(res);

		let order_hashes = OrderStorage::<Test>::iter().last().unwrap();
		let order_hash = order_hashes.0;

		let tx_uid = H256::from_slice("01234567890123456789012345678902".as_bytes());

		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, total_amount, deadline, due_date, order_item, order_hash, bonsai_token, tx_uid );
		assert_ok!(res);
	});
}

#[test]
fn should_change_spfso_should_fail_with_order_does_not_exist() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, total_amount, deadline, due_date, order_item, tx_uid.clone(), bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [35, 0, 0, 0],
				message: Some("OrderDoesNotExist"),
			})
		);
	});
}

#[test]
fn should_change_spfso_should_fail_with_approved() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Accepted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);


		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, total_amount, deadline, due_date, order_item, order_hash, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [11, 0, 0, 0],
				message: Some("Approved"),
			})
		);
	});
}

#[test]
fn should_change_spfso_should_fail_with_order_status_1() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 1u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);


		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, total_amount, deadline, due_date, order_item, order_hash, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [7, 0, 0, 0],
				message: Some("OrderStatus1"),
			})
		);
	});
}

#[test]
fn should_change_spfso_should_fail_with_order_status_2() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 3u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);


		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, total_amount, deadline, due_date, order_item, order_hash, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [8, 0, 0, 0],
				message: Some("OrderStatus2"),
			})
		);
	});
}

#[test]
fn should_change_spfso_should_fail_with_fulfiller() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);


		let res = Orders::change_spfso(RuntimeOrigin::signed(1), approver, fulfiller, total_amount, deadline, due_date, order_item, order_hash, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [18, 0, 0, 0],
				message: Some("Fulfiller"),
			})
		);
	});
}

#[test]
fn should_change_spfso_should_fail_with_amount() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);


		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, -1, deadline, due_date, order_item, order_hash, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [19, 0, 0, 0],
				message: Some("Amount"),
			})
		);
	});
}

#[test]
fn should_change_spfso_should_fail_with_short_deadline() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);


		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, total_amount, 20011, due_date, order_item, order_hash, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [20, 0, 0, 0],
				message: Some("ShortDeadline"),
			})
		);
	});
}

#[test]
fn should_change_spfso_should_fail_with_short_due_date() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);


		let res = Orders::change_spfso(RuntimeOrigin::signed(approver), approver, fulfiller, total_amount, deadline, 2, order_item, order_hash, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [21, 0, 0, 0],
				message: Some("ShortDueDate"),
			})
		);
	});
}

#[test]
fn should_change_approval_successfully() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);

		let res = Orders::change_approval(RuntimeOrigin::signed(approver), order_hash, ApprovalStatus::Accepted, bonsai_token, tx_uid );
		assert_ok!(res);
	});
}

#[test]
fn should_change_approval_should_fail_with_appr_status() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 0u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);

		let res = Orders::change_approval(RuntimeOrigin::signed(approver), order_hash, ApprovalStatus::Rejected, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [10, 0, 0, 0],
				message: Some("ApprStatus"),
			})
		);
	});
}

#[test]
fn should_change_approval_should_fail_with_not_approver() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 1;
		let deadline = 20011520;
		let due_date = 0;

		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_header = OrderHeader {
			commander: 1,
			fulfiller: fulfiller.clone(),
			approver: approver.clone(),
			order_status: 3u16,
			approval_status: ApprovalStatus::Submitted,
			buy_or_sell,
			amount: total_amount,
			market_order,
			order_type,
			deadline,
			due_date,
		};

		OrderStorage::<Test>::insert(&order_hash, order_header);

		let res = Orders::change_approval(RuntimeOrigin::signed(approver), order_hash, ApprovalStatus::Rejected, bonsai_token, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [0, 0, 0, 0],
				message: Some("NotApprover"),
			})
		);
	});
}

#[test]
fn should_handle_spfso_successfully() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 5;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), 1, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_ok!(res);

		let order_hashes = OrderStorage::<Test>::iter().last().unwrap();
		let order_hash = order_hashes.0;

		let mut stored_order_header = order_hashes.1;

		stored_order_header.order_status = 5;
		OrderStorage::<Test>::mutate(&order_hash, |order_header| {
			*order_header = Some(stored_order_header.clone());
		});

		let prefunding_hash_owner_values = PrefundingHashOwner::<Test>::iter().last().unwrap();
		let prefunding_hash = prefunding_hash_owner_values.0;

		let mut stored_prefunding_tuple = prefunding_hash_owner_values.1;
		stored_prefunding_tuple = (stored_prefunding_tuple.0, LockStatus::Locked,  stored_prefunding_tuple.2, LockStatus::Locked);
		PrefundingHashOwner::<Test>::mutate(&prefunding_hash, |value| {
			*value = Some(stored_prefunding_tuple);
		});

		let reference_status_values = ReferenceStatus::<Test>::iter().last().unwrap();
		let reference_hash = reference_status_values.0;

		ReferenceStatus::<Test>::mutate(&reference_hash, |value| {
			*value = Some(400);
		});

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let tx_uid = H256::from_slice("01234567890123456789012345678902".as_bytes());

		let res = Orders::handle_spfso(RuntimeOrigin::signed(1), order_hash, 6, tx_uid );
		assert_ok!(res);
	});
}

#[test]
fn should_handle_spfso_should_fail_with_getting_order_error() {
	new_test_ext().execute_with(|| {
		let order_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());


		let res = Orders::handle_spfso(RuntimeOrigin::signed(1), order_hash, 6, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [23, 0, 0, 0],
				message: Some("GettingOrder"),
			})
		);
	});
}

#[test]
fn should_handle_spfso_should_fail_with_urn_nobody_error() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 5;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), 1, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_ok!(res);

		let order_hashes = OrderStorage::<Test>::iter().last().unwrap();
		let order_hash = order_hashes.0;

		let mut stored_order_header = order_hashes.1;

		stored_order_header.order_status = 5;
		OrderStorage::<Test>::mutate(&order_hash, |order_header| {
			*order_header = Some(stored_order_header.clone());
		});

		let prefunding_hash_owner_values = PrefundingHashOwner::<Test>::iter().last().unwrap();
		let prefunding_hash = prefunding_hash_owner_values.0;

		let mut stored_prefunding_tuple = prefunding_hash_owner_values.1;
		stored_prefunding_tuple = (stored_prefunding_tuple.0, LockStatus::Locked,  stored_prefunding_tuple.2, LockStatus::Locked);
		PrefundingHashOwner::<Test>::mutate(&prefunding_hash, |value| {
			*value = Some(stored_prefunding_tuple);
		});

		let reference_status_values = ReferenceStatus::<Test>::iter().last().unwrap();
		let reference_hash = reference_status_values.0;

		ReferenceStatus::<Test>::mutate(&reference_hash, |value| {
			*value = Some(400);
		});

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let tx_uid = H256::from_slice("01234567890123456789012345678902".as_bytes());

		let res = Orders::handle_spfso(RuntimeOrigin::signed(10), order_hash, 6, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [6, 0, 0, 0],
				message: Some("URNobody"),
			})
		);
	});
}

#[test]
fn should_handle_spfso_should_fail_with_set_prefund_state_error() {
	new_test_ext().execute_with(|| {
		let product_hash = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let order_item  = OrderItem {
			product: product_hash,
			unit_price: 1000,
			quantity: 12,
			unit_of_measure: 5
		};


		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let tx_uid = H256::from_slice("01234567890123456789012345678901".as_bytes());

		let approver = 2;
		let fulfiller = 3;
		let buy_or_sell = 1;
		let total_amount = 1;
		let market_order = false;
		let order_type = 5;
		let deadline = 20011520;
		let due_date = 0;

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), 1, 1000000000, 1000000000));

		let res = Orders::create_spfso(RuntimeOrigin::signed(1), 1, fulfiller, buy_or_sell, total_amount, market_order, order_type, deadline, due_date, order_item, bonsai_token, tx_uid );
		assert_ok!(res);

		let order_hashes = OrderStorage::<Test>::iter().last().unwrap();
		let order_hash = order_hashes.0;

		let mut stored_order_header = order_hashes.1;

		stored_order_header.order_status = 5;
		OrderStorage::<Test>::mutate(&order_hash, |order_header| {
			*order_header = Some(stored_order_header.clone());
		});

		let prefunding_hash_owner_values = PrefundingHashOwner::<Test>::iter().last().unwrap();
		let prefunding_hash = prefunding_hash_owner_values.0;

		let mut stored_prefunding_tuple = prefunding_hash_owner_values.1;
		stored_prefunding_tuple = (stored_prefunding_tuple.0, LockStatus::Locked,  stored_prefunding_tuple.2, LockStatus::Locked);
		PrefundingHashOwner::<Test>::mutate(&prefunding_hash, |value| {
			*value = Some(stored_prefunding_tuple);
		});

		let reference_status_values = ReferenceStatus::<Test>::iter().last().unwrap();
		let reference_hash = reference_status_values.0;

		ReferenceStatus::<Test>::mutate(&reference_hash, |value| {
			*value = Some(400);
		});

		assert_ok!(Balances::set_balance(RuntimeOrigin::root(), Escrow::escrow_account(), 1000000000, 1000000000));

		let tx_uid = H256::from_slice("01234567890123456789012345678902".as_bytes());

		let res = Orders::handle_spfso(RuntimeOrigin::signed(3), order_hash, 6, tx_uid );
		assert_err!(
			res,
			DispatchError::Module(ModuleError {
				index: 2,
				error: [24, 0, 0, 0],
				message: Some("SetPrefundState"),
			})
		);
	});
}

