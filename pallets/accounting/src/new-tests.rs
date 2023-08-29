// For the function defined above, you can create a series of unit tests. Below is an example of some tests that could be used to verify the functionality and behavior of `set_accounting_ref_date()` function.

// In these tests we have covered three cases:
// - The happy path where everything works as expected.
// - A case where a non-signed origin tries to set the accounting ref date.
// - A case where a signed origin tries to re-set an existing Accounting Ref Date.

// We use fixtures provided by our `new_test_ext()` mockup environment. Using the test context provided by this fixture `execute_with()` we can test how our pallet functions interact with the runtime Storage and Events.

// Please note that creating all possible cases for each possible variant would be recommended for comprehensive coverage. This involves testing valid/invalid inputs (parameters) patterns or sequences etc.

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

#[test]
fn it_works_setting_accounting_ref_date() {
    new_test_ext().execute_with(|| {
        let block_number = 450_000u32.into();
        let _ = Balances::deposit_creating(&1, 1_000);

        // Case where block number is not already set
        assert_ok!(Accounting::set_accounting_ref_date(Origin::signed(1), block_number.clone()));
        
        // Expected event after successful call
        assert_eq!(last_event(), Event::<Test>::TestPallet(crate::Event::AccountingRefDateSet { who: 1, at_blocknumber: block_number }));

    });
}

#[test]
fn should_not_work_for_non_signed_origin() {
    new_test_ext().execute_with(|| {

         // Ensure that it rejects non-signed origins i.e., origin is not coming from one account.
         let bad_origin = Origin::none();
         let block_number = 450_000u32.into();
         assert_err!(
            Accounting::set_accounting_ref_date(bad_origin ,block_number.clone()),
            BadOrigin
            );

    });
}

#[test]
fn should_not_set_if_block_already_set(){
   new_test_ext().execute_with(|| {
       let initial_block = 446_401u32.into();
       <AccountingRefDate<Test>>::insert(&2, initial_block);
       let next_block=446_500u32.into();
       
        // Ensure accounting reference date has already been set for this 'who'.
        assert_noop!(
              Accounting::set_accounting_ref_date(OriginSigned(2),next_block),
              Error::<Test>::AccountingRefDateAlreadySet,
          );
   });

}

// Sure, I can help you write some unit tests to ensure the `set_opening_balance` function is working as expected. Here's an example of how such a test might look. I'll focus on testing the expected storage results and outcomes of this function.

// In this test case:

// * We're using an Extension Builder with a default configuration for our environment where Bob has a balance of `1000`.
// * We're confirming that setting the opening balance for Bob's account works correctly by calling our method with no additional entries.
// * Then we check if our Opening Balance status and Accounting Reference Date are correctly updated in our blockchain's storage. The logic behind these checks would need to evolve alongside your pallet implementation details.
// * Testing when setting an opening balance fails because there are no previous accounting references stored yet or when we try overstepping maximum number entries allowed.


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use frame_support::{assert_ok, assert_noop};
    
    #[test]
    fn it_should_set_opening_balance_correctly() {
        let mut ext = ExtBuilder::default().one_thousand_for_bob().build();
        ext.execute_with(|| {
            // Bob has a balance of 1000.
            let bob = 1;
            
            // First time setting opening balance should be successful.
            assert_ok!(Accounting::set_opening_balance(Origin::signed(bob), vec![], 0));
            
            // Check that the opening balance status and accounting reference date have been correctly set in storage.
            assert_eq!(OpeningBalance::<Test>::get(&bob, LedgerId::default()), true);
            assert_ne!(AccountingRefDate::<Test>::get(&bob), None);
           
           // Check that trying to set opening balance before accounting reference data is set would result in error
           AccountingRefDate::<Test>::remove(&bob);
           assert_noop!(
                Accounting::set_opening_balance(Origin::signed(bob), vec![], 0),
                Error::<Test>::AccountingRefDateNotSet
            );
            
           // Try to set more entries than allowed 
           let mut many_entries = vec![AdjustmentDetail{ledger: LedgerId::default(), amount: CurrencyBalanceOf<Test>(1), debit_credit: DebitCreditType}]; 
           many_entries.extend(repeat(AdjustmentDetail{ledger: LedgerId(i as u32 % 166u32 +1u32),.. AdjustmentDetail.default()})
                    .take((167usize)));
                    
           assert_noop!(
                Accounting::set_opening_balance(Origin::signed(bob), many_entries, 0),
                Error::<Test>::TooManyOpeningEntries
          );
            
          
        });
    }
// }

// Here are the unit tests for `adjustment( )` function:

// Note that these are rough examples of how you might want your unit tests. Please adapt and modify them according to your detailed needs and scenario setup such as creating ajustments dynamically etc., along with returning appropriate Result types from your functions.

// Also make sure that you have created an object of the struct related to the method you want apply unit testing on.

use super::*;
use codec::{Decode, Encode};
use frame_support::{assert_noop, assert_ok, sp_runtime::traits::BadOrigin};
use pallet_balances as balances;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

/// A macro to execute adjustment call as an Origin
macro_rules! exec_with_origin {
    ($origin:expr) => {
        AdjustmentAdjustmentExecuteCall::<TestRuntime> {
            origin: $origin.into(),
            adjustments: vec![],
            index: 1,
            applicable_period: 10,
        }
        .execute(Call)
    };
}

#[test]
fn it_works_for_valid_adjustments() {
    
    // Create a new account with a balance.
    let original_account = 1u64;
    
    // Generate fake transactions
   let mut adjustments : Vec<AdjustmentDetail<CurrencyBalanceOf<T>>> = vec![];
   
   for i in 0..5 {
       let adj_detail : AdjustmentDetail<CurrencyBalanceOf<T>> = AdjustmentDetail{
           ledger : "Test".to_owned(),
           amount : i+1 as u32,
           debit_credit: "debit".to_string()
       };
      adjustments.push(adj_detail);
   }

	// Making sure the testable function is working fine.
	let output =
	Accounting::adjustment(system::RawOrigin::Signed(original_account).into(),adjustments,1);
	assert_eq!(output.is_ok(), true);

}

#[test]
fn it_should_fail_because_number_of_opening_entries_exceed_10() {

     // Create a new account with a balance.
     let original_account = 2u64;

	// Generate fake transactions exceeding max limit of entries which is supposed to be equals to or less than '10'
	let mut adjustments_exceed_max_limit : Vec<AdjustmentDetail<CurrencyBalanceOf<T>>> = vec![];

	for i in 0..11 { 
	    let adj_entry : AdjustmentDetail<CurrencyBalanceOf<T>> = AdjustmentDetail{ ledger="TestAccount2".to_string(), amount=i+2 as u32 , debit_credit="credit".to_string()};
	    adjustments_exceed_max_limit.push(adj_entry);
	    
	}

      // The operation should fail due to too many entries (>=11)
      assert_noop!(
          Accounting::adjustment(
              system::RawOrigin::Signed(original_account).into(),
              adjustments_exceed_max_limit,2), 
              Error::<TestRuntime>::TooManyOpeningEntries

          );

}


#[test]
fn it_should_fail_due_to_zero_indexing() {

      // Create another account with zero index values.
      let empty_or_zero_indexed_accounts=3u64;

	  let output=
	  Accounting::adjustment (
	           system::RawOrigin::Signed(empty_or_zero_indexed_accounts).into()
	        );
	        
         /// The operation must fail With Error::<T>::IndexNotFound.  
         assert_noop!(empty_or_zero_indexed_accounts(Error::<Test>::index_empty()));
         
}