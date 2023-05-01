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
use crate::{mock::{new_test_ext, RuntimeOrigin, Orders, Test}, Orders as OrderStorage};
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use sp_core::H256;
use totem_primitives::orders::{ApprovalStatus, OrderHeader, OrderItem, TxKeysL, TxKeysM};

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
