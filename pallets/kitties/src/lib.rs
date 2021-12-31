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
        KittyNotExist,
        NotKittyOwner,
        TransferToSelf,
        BuyerIsKittyOwner,
        KittyBidPriceTooLow,
        KittyNotForSale,
        InsufficientBalance,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Action #3: Declare events
        Created(T::AccountId, T::Hash),
        PriceSet(T::AccountId, T::Hash, Option<BalanceOf<T>>),
        Transferred(T::AccountId, T::AccountId, T::Hash),
        Bought(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
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
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub kitties: Vec<(T::AccountId, [u8; 16], Gender)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { kitties: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			for (acct, dna, gender) in &self.kitties {
				let _ = <Pallet<T>>::mint(acct, Some(dna.clone()), Some(gender.clone()));
			}
		}
	}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        // todo part III: create_kitty
        #[pallet::weight(100)]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
            // Action #1: create_kitty
            let owner = ensure_signed(origin)?;

            let kitty_id = Self::mint(&owner, None, None)?;
            // log::info!("A kitty is born with ID: {:?}", kitty_id);

            // Action #4: Deposit 'Created' event
			Self::deposit_event(Event::Created(owner, kitty_id));

            Ok(())
        }

        // todo part IV: set_price
        #[pallet::weight(100)]
        pub fn set_price(origin: OriginFor<T>, kitty_id: T::Hash, new_price: Option<BalanceOf<T>>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            ensure!(Self::is_kitty_owner(&kitty_id, &owner)?, Error::<T>::NotKittyOwner);

            if let Some(mut kitty) = Self::kitties(&kitty_id) {
                kitty.price = new_price.clone();
                Kitties::<T>::insert(&kitty_id, kitty);
                Self::deposit_event(Event::PriceSet(owner, kitty_id, new_price));
                Ok(())
            } else {
                Err(Error::<T>::KittyNotExist)?
            }

        }

        // todo part IV: transfer
        #[pallet::weight(100)]
        pub fn tranfer_kitty(origin: OriginFor<T>, kitty_id: T::Hash, receiver: T::AccountId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(Self::is_kitty_owner(&kitty_id, &sender)?, Error::<T>::NotKittyOwner);
            ensure!(sender != receiver, Error::<T>::TransferToSelf);
            let receiver_owned = Kitties_owned::<T>::get(&receiver);
            ensure!((receiver_owned.len() as u32) < T::MaxKittyOwned::get(), Error::<T>::ExceedMaxKittyOwned);

            Self::transfer_kitty_to(&kitty_id, &receiver)?;

            Self::deposit_event(Event::Transferred(sender, receiver, kitty_id));

            Ok(())
        }

        // todo part IV: buy_kitty
        #[transactional]
        #[pallet::weight(100)]
        pub fn buy_kitty(origin: OriginFor<T>, kitty_id: T::Hash, bid_price: BalanceOf<T>) -> DispatchResult {
            let buyer = ensure_signed(origin)?;

            let kitty = Self::kitties(&kitty_id).ok_or(Error::<T>::KittyNotExist)?;
            ensure!(kitty.owner != buyer, Error::<T>::BuyerIsKittyOwner);

            if let Some(ask_price) = kitty.price {
                ensure!(ask_price <= bid_price, Error::<T>::KittyBidPriceTooLow);
            } else {
                Err(Error::<T>::KittyNotForSale)?;
            }

            ensure!(T::Currency::free_balance(&buyer) >= bid_price, Error::<T>::InsufficientBalance);

            let buyer_owned = Kitties_owned::<T>::get(&buyer);
            ensure!((buyer_owned.len() as u32) < T::MaxKittyOwned::get(), Error::<T>::ExceedMaxKittyOwned);

            let seller = kitty.owner.clone();

            T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;

            Self::transfer_kitty_to(&kitty_id, &buyer)?;

            Self::deposit_event(Event::Bought(buyer, seller, kitty_id, bid_price));

            Ok(())
        }


        // todo part IV: breed_kitty

    }

	//** Our helper functions.**//
    impl<T:Config> Pallet<T> {
        // todo part III: helper functions for dispatchable functions
        fn is_kitty_owner(kitty_id: &T::Hash, acct: &T::AccountId) -> Result<bool, Error<T>> {
            match Self::kitties(kitty_id) {
                Some(kitty) => Ok(kitty.owner == *acct),
                None => Err(Error::<T>::KittyNotExist)
            }
        }


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

        // Todo part III: mint
		fn mint(owner: &T::AccountId, dna: Option<[u8; 16]>, gender: Option<Gender>) -> Result<T::Hash, Error<T>> {
			let kitty = Kitty::<T> {
                dna: dna.unwrap_or_else(Self::gen_dna),
                gender: gender.unwrap_or_else(Self::gen_gender),
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
			Ok(kitty_id)
		}


        // Todo part IV: transfer_kitty_to
        #[transactional]
        fn transfer_kitty_to(kitty_id: &T::Hash, receiver: &T::AccountId) -> Result<(), Error<T>> {
            let mut kitty = Self::kitties(&kitty_id).ok_or(Error::<T>::KittyNotExist)?;
            let prev_owner = kitty.owner.clone();

            Kitties_owned::<T>::try_mutate(&prev_owner, |owned| {
                if let Some(ind) = owned.iter().position(|&id| id == *kitty_id) {
                    owned.swap_remove(ind);
                    return Ok(())
                }
                Err(())
            }).map_err(|_| Error::<T>::KittyNotExist)?;

            kitty.owner = receiver.clone();
            kitty.price = None;

            Kitties::<T>::insert(kitty_id, kitty);

            Kitties_owned::<T>::try_mutate(receiver, |vec|{
                vec.try_push(*kitty_id)
            }).map_err(|_| Error::<T>::ExceedMaxKittyOwned)?;

            Ok(())
        }

    }
}