use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn create_should_work() {
    new_test_ext().execute_with(|| {
        assert_eq!(SubstrateKitties::kitties_count(), Some(2));
        assert_eq!(SubstrateKitties::kitty_owned(1).len(), 1);
        assert_ok!(SubstrateKitties::create(Origin::signed(1)));
        assert_eq!(SubstrateKitties::kitties(2).unwrap().owner, 1);
        assert_eq!(SubstrateKitties::kitties_count(), Some(3));
        assert_eq!(SubstrateKitties::kitty_owned(1).len(), 2);
	});
}

#[test]
fn tranfer_should_work() {
    new_test_ext().execute_with(||{
        assert_eq!(SubstrateKitties::kitties(0).unwrap().owner, 1);
        assert_eq!(SubstrateKitties::kitty_owned(1).len(), 1);
        assert_eq!(SubstrateKitties::kitty_owned(2).len(), 1);
        assert_ok!(SubstrateKitties::transfer(Origin::signed(1), 2, 0));
        assert_eq!(SubstrateKitties::kitty_owned(1).len(), 0);
        assert_eq!(SubstrateKitties::kitty_owned(2).len(), 2);
        assert_eq!(SubstrateKitties::kitty_owned(2), [1, 0]);
        assert_eq!(SubstrateKitties::kitties(0).unwrap().owner, 2);
    });
}

#[test]
fn transfer_failed_because_not_the_owner() {
    new_test_ext().execute_with(||{
        assert_noop!(SubstrateKitties::transfer(Origin::signed(1), 2, 1), Error::<Test>::NotOwner);
    });
}

#[test]
fn transfer_failed_because_kitty_not_exists() {
    new_test_ext().execute_with(||{
        assert_noop!(SubstrateKitties::transfer(Origin::signed(1), 2, 1000), Error::<Test>::KittyNotExist);
    });
}

#[test]
fn breed_should_work() {
    new_test_ext().execute_with(||{
        assert_ok!(SubstrateKitties::transfer(Origin::signed(2), 1, 1));
        assert_eq!(SubstrateKitties::kitty_owned(1), [0, 1]);
        assert_ok!(SubstrateKitties::breed(Origin::signed(1), 0, 1));
        assert_eq!(SubstrateKitties::kitty_owned(1), [0, 1, 2]);
        assert_eq!(SubstrateKitties::kitties_count(), Some(3));
        assert_eq!(SubstrateKitties::kitties(2).unwrap().owner, 1);
    });
}

// fn tmp() {
//     new_test_ext().execute_with(||{

//     });
// }

// fn tmp() {
//     new_test_ext().execute_with(||{

//     });
// }

// fn tmp() {
//     new_test_ext().execute_with(||{

//     });
// }