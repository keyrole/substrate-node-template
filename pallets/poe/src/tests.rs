use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use super::*;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(
            Proofs::<Test>::get(&claim),
            (1, frame_system::Pallet::<Test>::block_number()
        ));
    })
}


#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1),claim.clone());
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (0, 0));
    });
}

#[test]
fn failed_to_revoke_when_the_claim_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0,1];
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::NoSuchProof
        );
    });
}

// sender is not owner
#[test]
fn failed_to_revoke_claim_when_sender_is_not_the_owner() {
    new_test_ext().execute_with(||{
        let claim =vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotProofOwner
        );
    });
}

// transfer works
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert_eq!(Proofs::<Test>::get(&claim), (2, frame_system::Pallet::<Test>::block_number()));
    });
}


// not the owner
#[test]
fn failed_to_transfer_claim_when_the_sender_is_not_the_owner() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        let _ =PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));
        assert_noop!(PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3), Error::<Test>::NotProofOwner);
    });
}

// claim do not exist
#[test]
fn failed_to_transfer_when_the_claim_does_not_exist() {
    new_test_ext().execute_with(||{
        let claim = vec![0, 1];
        assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2), Error::<Test>::NoSuchProof);
    });
}