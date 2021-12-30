#![cfg(test)]

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_a_kitty_successfully() {
    new_test_ext().execute_with(||{
        assert_ok!(Kitties::create_kitty(Origin::signed(1)));
        assert_eq!(Kitties::all_kitties_count(), 1);
        assert_eq!(Kitties::kitties_owned(1).len(), 1);
    })
}