#![cfg(test)]

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::H256;
use totem_primitives::RecordType;

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Dispatch a signed extrinsic.
        assert_ok!(Archive::archive_record(
            Origin::signed(1),
            RecordType::Timekeeping,
            H256([1; 32]),
            false,
        ));
        // Read pallet storage and assert an expected result.
        //assert_eq!(Archive::something(), Some(42));
    });
}
