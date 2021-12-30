#![cfg(test)]

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_a_kitty_successfully() {
    new_test_ext().execute_with(||{
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
        // assert_eq!(SubstrateKitties::all_kitties_count(), 1);
        // assert_eq!(SubstrateKitties::kitties_owned(1).len(), 1);
    })
}