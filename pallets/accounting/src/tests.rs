#![cfg(test)]

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

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

// #![cfg(test)]

// use super::*;
// use frame_support::{assert_noop, assert_ok, dispatch::DispatchError};
// use mock::{Accounting, Event, Origin, System, Test};

// #[test]
// fn do_something_works() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(Accounting::do_something(Origin::signed(1), 42));
//         assert_eq!(Accounting::something(), Some(42));

//         // Verify that the correct event was emitted
//         let event = Event::Accounting(crate::Event::SomethingStored(42, 1));
//         assert!(System::events().iter().any(|record| record.event == event));
//     });
// }

// #[test]
// fn cause_error_works() {
//     new_test_ext().execute_with(|| {
//         assert_noop!(
//             Accounting::cause_error(Origin::signed(1)),
//             DispatchError::Module {
//                 index: 0,
//                 error: 1,
//                 message: Some("NoneValue")
//             }
//         );
//     });
// }
