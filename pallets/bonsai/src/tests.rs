#![cfg(test)]

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Timekeeping::invoice_time(
            Origin::signed(1),
            H256([0; 32]),
            H256([0; 32]),
        ));
        // Read pallet storage and assert an expected result.
        //assert_eq!(Timekeeping::something(), Some(42));
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        //assert_noop!(
        //    Timekeeping::cause_error(Origin::signed(1)),
        //    Error::<Test>::NoneValue
        //);
    });
}
