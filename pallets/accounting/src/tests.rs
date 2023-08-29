#![cfg(test)]

// use super::*;
// use codec::{Decode, Encode};
// use pallet_balances as balances;
// use sp_core::H256;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
// use sp_runtime::traits::BadOrigin;
use sp_runtime::{
    // testing::Header,
    traits::{
        BadOrigin,
        // BlakeTwo256, 
        // IdentityLookup
    },
};

/// A macro to execute adjustment call as an Origin
// macro_rules! exec_with_origin {
//     ($origin:expr) => {
//         AdjustmentAdjustmentExecuteCall::<TestRuntime> {
//             origin: $origin.into(),
//             adjustments: vec![],
//             index: 1,
//             applicable_period: 10,
//         }
//         .execute(Call)
//     };
// }

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Accounting::do_something(Origin::signed(1), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(Accounting::something(), Some(42));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            Accounting::cause_error(Origin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}


#[test]
fn it_works_setting_accounting_ref_date() {
    new_test_ext().execute_with(|| {
        let block_number = 450_000u32.into();
        let _ = Balances::deposit_creating(&1, 1_000);

        // Case where block number is not already set
        assert_ok!(TestPallet::set_accounting_ref_date(Origin::signed(1), block_number.clone()));
        
        // Expected event after successful call
        assert_eq!(last_event(), Event::<Test>::TestPallet(crate::Event::AccountingRefDateSet { who: 1, at_blocknumber: block_number }));

    });
}
