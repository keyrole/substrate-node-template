#![cfg(test)]

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};


#[test]
fn create_a_kitty_successfully() {
    new_test_ext().execute_with(||{
        assert_eq!(SubstrateKitties::all_kitties_count(), 2);
        assert_ok!(SubstrateKitties::create_kitty(Origin::signed(1)));
        assert_eq!(SubstrateKitties::all_kitties_count(), 3);
        assert_eq!(SubstrateKitties::kitties_owned(1).len(), 2);
    });
}

#[test]
fn set_price_should_work() {
    new_test_ext().execute_with(||{
        // let kitty = <pallet::Pallet<mock::Test> as Trait>::Kitty::<T> {
        //     dna: *b"1234567890123456",
        //     gender: <pallet::Pallet<mock::Test> as Trait>::Gender::Female,
        //     owner: 1,
        //     price: None,
        // };

        // let kitty_id = <pallet::Pallet<Test> as Trait>::T::Hashing::hash_of(&kitty);
        // let acct: <pallet::Pallet<Test> as Trait>::T::AccountId = 1;
        // assert_ok!(let kitty_id = SubstrateKitties::kitties_owned(1).0);
        assert_ok!(SubstrateKitties::set_price(Origin::signed(1), SubstrateKitties::kitties_owned(1)[0], Some(3)));
        assert_eq!(SubstrateKitties::kitties(SubstrateKitties::kitties_owned(1)[0]).unwrap().price, Some(3));
    });
}

#[test]
fn transfer_kitty_should_work() {
    new_test_ext().execute_with(||{
        assert_eq!(SubstrateKitties::kitties_owned(2).len(), 1);
        assert_ok!(SubstrateKitties::tranfer_kitty(Origin::signed(1), SubstrateKitties::kitties_owned(1)[0], 2));
        assert_eq!(SubstrateKitties::kitties_owned(2).len(), 2);
        assert_eq!(SubstrateKitties::kitties_owned(1).len(), 0);

    });
}

#[test]
fn buy_kitty_should_work(){
    new_test_ext().execute_with(||{
        assert_ok!(SubstrateKitties::set_price(Origin::signed(1), SubstrateKitties::kitties_owned(1)[0], Some(3)));
        assert_ok!(SubstrateKitties::buy_kitty(Origin::signed(2), SubstrateKitties::kitties_owned(1)[0], 4));
        assert_eq!(SubstrateKitties::kitties_owned(1).len(), 0);
        assert_eq!(SubstrateKitties::kitties_owned(2).len(), 2);
        assert_eq!(Balances::free_balance(&1), 14);
        assert_eq!(Balances::free_balance(&2), 6);

    });
}