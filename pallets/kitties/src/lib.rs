#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Randomness};
    use frame_system::pallet_prelude::*;
    use codec::{Encode, Decode};
    use frame_support::sp_io::hashing::blake2_128;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {

    }

    #[pallet::error]
    pub enum Error<T> {

    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

    }

    impl<T: Config> Pallet<T> {

    }
}