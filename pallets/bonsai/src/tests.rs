// You should have received a copy of the GNU General Public License
// along with Totem.  If not, see <http://www.gnu.org/licenses/>.
use crate::{mock::{new_test_ext, RuntimeOrigin, Bonsai, Teams, Timekeeping, Test, Balances, Escrow, Orders}, IsValidRecord};
use pallet_timekeeping::TimeHashOwner;
use pallet_orders::Orders as OrdersStorage;
use sp_runtime::{DispatchError, ModuleError};
use frame_support::{assert_err, assert_ok};
use frame_support::pallet_prelude::DispatchResult;
use sp_core::H256;
use pallet_prefunding::{PrefundingHashOwner, ReferenceStatus};
use pallet_teams::TeamHashOwner;
use totem_primitives::orders::{ApprovalStatus, OrderHeader, OrderItem, TxKeysL, TxKeysM};
use totem_primitives::RecordType;
use totem_primitives::timekeeping::{ReasonCodeStruct, StatusOfTimeRecord};

#[test]
fn should_update_record_successfully_when_record_type_is_teams() {
	new_test_ext().execute_with(|| {
		let key = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), key.clone().into()));

		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
		assert_ok!(Bonsai::update_record(RuntimeOrigin::signed(1), RecordType::Teams, key,  bonsai_token));


		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  true);
	});
}

#[test]
fn should_update_record_should_fail_when_record_type_is_teams() {
	new_test_ext().execute_with(|| {
		let key = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());

		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
		assert_err!(
			Bonsai::update_record(RuntimeOrigin::signed(1), RecordType::Teams, key,  bonsai_token),
			DispatchError::Module(ModuleError {
				index: 4,
				error: [2, 0, 0, 0],
				message: Some("NotTransactionOwner"),
			})
		);

		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
	});
}

#[test]
fn should_update_record_successfully_when_record_type_is_timekeeping() {
	new_test_ext().execute_with(|| {
		let bytes = "totemteamforpolkadotpolkadotpolk".as_bytes();
		let team_hash = H256::from_slice(&bytes);

		assert_ok!(Teams::add_new_team(RuntimeOrigin::signed(1), team_hash.into()));

		assert_ok!(Timekeeping::notify_team_worker(RuntimeOrigin::signed(1),  2, team_hash));

		assert_ok!(Timekeeping::worker_acceptance_team(RuntimeOrigin::signed(2), team_hash, true));

		let time_hash = <Timekeeping>::get_default_hash();

		let submit_status = StatusOfTimeRecord::Draft;
		let reason_code = 0u16;
		let reason_code_type = 0u16;
		let reason_for_change = ReasonCodeStruct(reason_code, reason_code_type);
		let number_of_blocks = 100u64;
		let posting_period = 24u16;
		let start_block_number= 200u64;
		let end_block_number = 300u64;
		let break_counter= 0u16;

		assert_ok!(Timekeeping::submit_time(RuntimeOrigin::signed(2), team_hash, time_hash, submit_status,
			reason_for_change, number_of_blocks, posting_period, start_block_number, end_block_number, break_counter
		));

		let team_hash_owner = TimeHashOwner::<Test>::iter().last().unwrap();
		let key = team_hash_owner.0;
		let owner = team_hash_owner.1;
		dbg!(key);
		dbg!(owner);
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());


		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
		assert_ok!(Bonsai::update_record(RuntimeOrigin::signed(owner), RecordType::Timekeeping, key,  bonsai_token));
		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  true)

	});
}

#[test]
fn should_update_record_should_fail_when_record_type_is_timekeeping() {
	new_test_ext().execute_with(|| {
		let key = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());

		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
		assert_err!(
			Bonsai::update_record(RuntimeOrigin::signed(1), RecordType::Timekeeping, key,  bonsai_token),
			DispatchError::Module(ModuleError {
				index: 4,
				error: [2, 0, 0, 0],
				message: Some("NotTransactionOwner"),
			})
		);

		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
	});
}

#[test]
fn should_update_record_successfully_when_record_type_is_order() {
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
		OrdersStorage::<Test>::insert(&parent_id, order_header);

		assert_eq!(IsValidRecord::<Test>::get(&parent_id).is_some(),  false);
		assert_ok!(Bonsai::update_record(RuntimeOrigin::signed(1), RecordType::Orders, parent_id,  bonsai_token));
		assert_eq!(IsValidRecord::<Test>::get(&parent_id).is_some(),  true)
	});
}

#[test]
fn should_update_record_should_fail_when_record_type_is_orders() {
	new_test_ext().execute_with(|| {
		let key = H256::from_slice("01234567890123456789012345678901".as_bytes());
		let bonsai_token = H256::from_slice("01234567890123456789012345678901".as_bytes());

		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
		assert_err!(
			Bonsai::update_record(RuntimeOrigin::signed(1), RecordType::Orders, key,  bonsai_token),
			DispatchError::Module(ModuleError {
				index: 4,
				error: [2, 0, 0, 0],
				message: Some("NotTransactionOwner"),
			})
		);

		assert_eq!(IsValidRecord::<Test>::get(&key).is_some(),  false);
	});
}

