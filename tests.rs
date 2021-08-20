use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

#[test]
fn test_create_claim() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		// assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
		// // Read pallet storage and assert an expected result.
		// assert_eq!(TemplateModule::something(), Some(42));

    assert_noop!(
      PoeModule::create_claim(Origin::signed(1), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
      Error::<Test>::InvalidClaimLength
    );


    let claim = vec![0, 1];
    assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

    assert_eq!( 
      Proofs::<Test>::get(&claim), 
      (1, frame_system::Pallet::<Test>::block_number())
    );

    assert_noop!(
      PoeModule::create_claim(Origin::signed(1), claim.clone()),
      Error::<Test>::ProofAlreadyClaimed
    );
	});
}

#[test]
fn test_revoke_claim() {
	new_test_ext().execute_with(|| {

    let claim = vec![0, 1];

    assert_noop!(
      PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
      Error::<Test>::NoSuchProof
    );

    assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

    assert_eq!( 
      Proofs::<Test>::get(&claim), 
      (1, frame_system::Pallet::<Test>::block_number())
    );

    assert_noop!(
      PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
      Error::<Test>::NotProofOwner
    );

    assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    assert_eq!( 
      Proofs::<Test>::get(&claim), 
      (0, 0)
    );
	
	});
}


#[test]
fn test_transfer_claim() {
	new_test_ext().execute_with(|| {
    let claim = vec![0, 1];
    assert_noop!(
      PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
      Error::<Test>::NoSuchProof
    );

    assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

    assert_eq!( 
      Proofs::<Test>::get(&claim), 
      (1, frame_system::Pallet::<Test>::block_number())
    );

    assert_noop!(
      PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
      Error::<Test>::NotProofOwner
    );
		
	});
}