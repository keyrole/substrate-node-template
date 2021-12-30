#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{sp_runtime::traits::{Hash, Zero},
                        dispatch::{DispatchResultWithPostInfo, DispatchResult},
                        traits::{Currency, ExistenceRequirement, Randomness},
                        pallet_prelude::*,
                        transactional};
    use frame_system::pallet_prelude::*;
    use frame_support::sp_io::hashing::blake2_128;
    use scale_info::TypeInfo;

    #[cfg(feature = "std")]
    use frame_support::serde::{Deserialize, Serialize};

    type AccountOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    // Action #1: Write a Struct to hold Kitty information
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    pub struct Kitty<T: Config> {
        pub dna: [u8; 16],
        pub price: Option<BalanceOf<T>>,
        pub gender: Gender,
        pub owner: AccountOf<T>,
    }
    
    // Action #2: Enum declaration for Gender
    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
    #[scale_info(skip_type_params(T))]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum Gender {
        Male,
        Female,
    }

    // Action #3: Implementation to handle Gender type in Kitty struct


    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId>;

        // Action #5: Specify the type for Randomness we want to specify for runtime
        type KittyRandomness: Randomness<Self::Hash, Self::BlockNumber>;

        // Action #9: Add MaxKittyOwned constant
        #[pallet::constant]
        type MaxKittyOwned: Get<u32>;   
    }

    #[pallet::error]
    pub enum Error<T> {
        // Action #5a: Declare errors.
        OverMaxNumber,
        KittyCntOverflow,
        ExceedMaxKittyOwned,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Action #3: Declare events
        Created(T::AccountId, T::Hash),
    }

    #[pallet::storage]
    #[pallet::getter(fn all_kitties_count)]
    pub(super) type KittyCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub(super) type Kitties<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Kitty<T>>;

    // Action #7: Remaining storage items.
    #[pallet::storage]
    #[pallet::getter(fn kitties_owned)]
    pub(super) type Kitties_owned<T: Config> = 
        StorageMap<_, Twox64Concat, T::AccountId, BoundedVec<T::Hash, T::MaxKittyOwned>, ValueQuery>;

    // Todo Part IV: Our pallet's genesis configuration

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // todo part III: create_kitty
        #[pallet::weight(100)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            // Action #1: create_kitty
            let owner = ensure_signed(origin)?;

            let kitty = Kitty::<T> {
                dna: Self::gen_dna(),
                gender: Self::gen_gender(),
                owner: owner.clone(),
                price: None,
            };

            let kitty_id = T::Hashing::hash_of(&kitty);

            let new_cnt = Self::all_kitties_count().checked_add(1).ok_or(Error::<T>::KittyCntOverflow)?;

            <Kitties_owned<T>>::try_mutate(&owner, |kitty_vec|{
                kitty_vec.try_push(kitty_id)
            }).map_err(|_| Error::<T>::ExceedMaxKittyOwned)?;

            <Kitties<T>>::insert(kitty_id, kitty);
            <KittyCnt<T>>::put(new_cnt);
            // log::info!("A kitty is born with ID: {:?}", kitty_id);

            // Action #4: Deposit 'Created' event
			Self::deposit_event(Event::Created(owner, kitty_id));

            Ok(())
        }

        // todo part IV: set_price

        // todo part IV: transfer

        // todo part IV: buy_kitty

        // todo part IV: breed_kitty

    }

	//** Our helper functions.**//
    impl<T:Config> Pallet<T> {

        // Generate a random gender value
        fn gen_gender() -> Gender {
            let random = T::KittyRandomness::random(&b"gender"[..]).0;
            match random.as_ref()[0] %2 {
                0 => Gender::Male,
                _ => Gender::Female,
            }
        }

        // Generate a random DNA value
        fn gen_dna() -> [u8; 16] {
            let payload = (
                T::KittyRandomness::random(&b"dna"[..]).0,
                <frame_system::Pallet<T>>::block_number(),
            );
            payload.using_encoded(blake2_128)
        }

        // todo part III: helper functions for dispatchable functions

        // Todo part III: mint

        // Todo part IV: transfer_kitty_to

    }
}