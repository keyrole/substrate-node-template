#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Randomness};
    use frame_system::pallet_prelude::*;
    use codec::{Encode, Decode};
    use frame_support::sp_io::hashing::blake2_128;

    #[derive(Encode, Decode)]
    pub struct Kitty(pub [u8; 16]);

    type KittyIndex = u32;

    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreate(T::AccountId, KittyIndex),
    }

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type KittiesCount<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty>, ValueQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn owner)]
    pub type Owner<T: Config> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<T::AccountId>, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        KittiesCountOverflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty_id = match Self::kitties_count() {
                Some(id) => {
                    ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
                    id
                },
                None => {
                    1
                }
            };

            let dna = Self::random_value(&who);

            Kitties::<T>::insert(kitty_id, Some(Kitty(dna)));

            Owner::<T>::insert(kitty_id, Some(who.clone()));

            KittiesCount::<T>::put(kitty_id + 1);

            Self::deposit_event(Event::KittyCreate(who, kitty_id));

            Ok(())
        }

    }

    impl<T: Config> Pallet<T> {
        fn random_value(sender: &T::AccountId) -> [u8; 16] {
            let payload = (
                T::Randomness::random_seed(),
                &sender,
                <frame_system::Pallet<T>>::extrinsic_index(),
            );
            payload.using_encoded(blake2_128)
        }

    }
}