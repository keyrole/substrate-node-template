#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_support::{traits::{Randomness, Currency, tokens::ExistenceRequirement}, transactional};
    use frame_system::pallet_prelude::*;
    use codec::{Encode, Decode};
    use frame_support::sp_io::hashing::blake2_128;

    type AccountOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
    

    #[derive(Clone, Encode, Decode, PartialEq)]
    pub struct Kitty<T: Config> {
        pub dna: [u8; 16],
        pub owner: AccountOf<T>,
        pub price: Option<BalanceOf<T>>,
    }

    type KittyIndex = u32;

    
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash, Self::BlockNumber>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);
    
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreate(T::AccountId, KittyIndex),
        KittyTransfer(T::AccountId, T::AccountId, KittyIndex),
        SellKitty(T::AccountId, KittyIndex, Option<BalanceOf<T>>),
        CancelSellKitty(T::AccountId, KittyIndex),
        BuyKitty(T::AccountId, T::AccountId, KittyIndex, BalanceOf<T>),
    }

    #[pallet::storage]
    #[pallet::getter(fn kitties_count)]
    pub type KittiesCount<T> = StorageValue<_, u32>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T> = StorageMap<_, Blake2_128Concat, KittyIndex, Option<Kitty<T>>, ValueQuery>;
    
    #[pallet::storage]
    #[pallet::getter(fn kitty_owned)]
    pub type KittiesOwned<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<KittyIndex>, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        KittiesCountOverflow,
        NotOwner,
        InvalidKittyIndex,
        SameParentIndex,
        PushKittiesOwnedFailed,
        KittyNotExist,
        CanNotBuyTheKittyYouOwned,
        TheKittyIsNotOnSell,
        BidPriceIsTooLow,
        InsufficientBalance,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub kitties: Vec<(T::AccountId, [u8; 16])>,
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
            for (acct, dna) in &self.kitties {
                let _ = <Pallet<T>>::mint(acct, Some(dna.clone()));
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty_id = Self::mint(&who, None)?;

            Self::deposit_event(Event::KittyCreate(who, kitty_id));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn transfer(origin: OriginFor<T>, new_owner: T::AccountId, kitty_id: KittyIndex) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(None != Self::kitties(kitty_id), Error::<T>::KittyNotExist);
            ensure!(who.clone() == Self::kitties(kitty_id).unwrap().owner, Error::<T>::NotOwner);

            Self::transfer_kitty_to(kitty_id, &new_owner)?;

            Self::deposit_event(Event::KittyTransfer(who, new_owner, kitty_id));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn breed(origin: OriginFor<T>, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let kitty_id = Self::breed_kitty(&who, kitty_id_1, kitty_id_2)?;

            Self::deposit_event(Event::KittyCreate(who, kitty_id));

            Ok(())
        }

        #[pallet::weight(0)]
        pub fn sell(origin: OriginFor<T>, kitty_id: KittyIndex, price: Option<BalanceOf<T>>) -> DispatchResult {
            let seller = ensure_signed(origin)?;
            Self::set_price(&seller, kitty_id, &price)?;

            Self::deposit_event(Event::SellKitty(seller, kitty_id, price));
            Ok(())
        }

        #[pallet::weight(0)]
        pub fn CancelSell(origin: OriginFor<T>, kitty_id: KittyIndex) -> DispatchResult {
            let seller = ensure_signed(origin)?;
            let price = None;
            Self::set_price(&seller, kitty_id, &price)?;

            Self::deposit_event(Event::CancelSellKitty(seller, kitty_id));
            Ok(())
        }

        #[transactional]
        #[pallet::weight(0)]
        pub fn buy(origin: OriginFor<T>, kitty_id: KittyIndex, bid_price: BalanceOf<T>) -> DispatchResult {
            let buyer = ensure_signed(origin)?;
            let kitty = Self::kitties(kitty_id).ok_or(Error::<T>::KittyNotExist)?;
            ensure!(buyer.clone() != kitty.owner, Error::<T>::CanNotBuyTheKittyYouOwned);

            // ensure the bid_price is not lower than sell_price
            if let Some(sell_price) = kitty.price {
                ensure!(bid_price >= sell_price, Error::<T>::BidPriceIsTooLow);
            } else {
                Err(Error::<T>::TheKittyIsNotOnSell)?;
            }

            ensure!(T::Currency::free_balance(&buyer) >= bid_price, Error::<T>::InsufficientBalance);

            let seller = kitty.owner.clone();
            T::Currency::transfer(&buyer, &seller, bid_price, ExistenceRequirement::KeepAlive)?;
            Self::transfer_kitty_to(kitty_id, &buyer)?;

            Self::deposit_event(Event::BuyKitty(buyer, seller, kitty_id, bid_price));

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

        fn mint(acct: &T::AccountId, mut dna: Option<[u8; 16]>) -> Result<KittyIndex, Error<T>> {
            let kitty_id = match Self::kitties_count() {
                Some(id) => {
                    ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
                    id
                },
                None => {
                    0
                }
            };

            if let None = dna {
                dna = Some(Self::random_value(&acct));
            };

            let kitty = Kitty::<T> {
                dna: dna.unwrap(),
                owner: acct.clone(),
                price: None,
            };

            Kitties::<T>::insert(kitty_id, Some(kitty));

            KittiesOwned::<T>::mutate(&acct, |kitty_vec|{
                kitty_vec.push(kitty_id)
            });

            KittiesCount::<T>::put(kitty_id + 1);

            Ok(kitty_id)
        }

        fn transfer_kitty_to(kitty_id: KittyIndex, receiver: &T::AccountId) -> Result<(), Error<T>> {
            
            // ensure the kitty exists
            let mut kitty = Self::kitties(kitty_id).ok_or(Error::<T>::KittyNotExist)?;

            let sender = kitty.owner.clone();

            kitty.owner = receiver.clone();

            // modify the owned map first
            KittiesOwned::<T>::mutate(&sender, |owned_vec|{
                if let Some(found_id) = owned_vec.iter().position(|&id| id == kitty_id) {
                    owned_vec.swap_remove(found_id);
                    return Ok(());
                }
                Err(())
            }).map_err(|_| Error::<T>::NotOwner)?;

            KittiesOwned::<T>::mutate(&receiver, |vec|{
                vec.push(kitty_id)
            });

            // when the kitty is transfered, the price should be set to None
            kitty.price = None;    
            Kitties::<T>::insert(kitty_id, Some(kitty));
            Ok(())
        }

        fn breed_kitty(owner: &T::AccountId, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> Result<KittyIndex, Error<T>> {
            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

            let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyIndex)?;
            let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyIndex)?;
            ensure!(owner.clone() == Self::kitties(kitty_id_1).unwrap().owner, Error::<T>::NotOwner);
            ensure!(owner.clone() == Self::kitties(kitty_id_2).unwrap().owner, Error::<T>::NotOwner);

            let kitty_id = match Self::kitties_count() {
                Some(id) => {
                    ensure!(id != KittyIndex::max_value(), Error::<T>::KittiesCountOverflow);
                    id
                },
                None => {
                    0
                }
            };

            let dna_1 = kitty1.dna;
            let dna_2 = kitty2.dna;

            let selector = Self::random_value(&owner);
            let mut new_dna = [0u8; 16];

            for i in 0..dna_1.len() {
                new_dna[i] = (selector[i] & dna_1[i]) | (selector[i] & dna_2[i]);
            }

            let new_kitty = Kitty::<T> {
                dna: new_dna,
                owner: owner.clone(),
                price: None,
            };

            Kitties::<T>::insert(kitty_id, Some(new_kitty));
            KittiesOwned::<T>::mutate(owner.clone(), |vec|{
                vec.push(kitty_id);
            });
            KittiesCount::<T>::put(kitty_id + 1);
            Ok(kitty_id)
        }

        fn set_price(acct: &T::AccountId, kitty_id: KittyIndex, price: &Option<BalanceOf<T>>) -> Result<(), Error<T>> {
            let mut kitty = Self::kitties(kitty_id).ok_or(Error::<T>::KittyNotExist)?;
            ensure!(acct.clone() == Self::kitties(kitty_id).unwrap().owner, Error::<T>::NotOwner);
            kitty.price = *price;
            Kitties::<T>::insert(kitty_id, Some(kitty));
            Ok(())
        }
    }
}