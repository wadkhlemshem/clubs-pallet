#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, BoundedBTreeSet};
    use frame_system::pallet_prelude::*;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// The maximum size of a club.
        #[pallet::constant]
        type MaxClubSize: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage items.
    #[pallet::storage]
    pub type Clubs<T: Config> =
        StorageMap<_, Twox64Concat, u32, BoundedBTreeSet<T::AccountId, T::MaxClubSize>>;

    // Pallets use events to inform users when important changes are made.
    // https://docs.substrate.io/v3/runtime/events-and-errors
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Club created.
        ClubCreated { club_number: u32 },
        /// User added to club.
        UserAddedToClub {
            club_number: u32,
            user: T::AccountId,
        },
        /// User removed from club.
        UserRemovedFromClub {
            club_number: u32,
            user: T::AccountId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Club membership is full.
        ClubFull,
        /// Club does not exist.
        ClubDoesNotExist,
        /// User does not exist in club.
        UserNotInClub,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a user to a club. If the club does not exist, it will be created.
        #[pallet::weight(50_000_000)]
        pub fn add_user_to_club(
            origin: OriginFor<T>,
            club_number: u32,
            user: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            if let None = Clubs::<T>::get(club_number) {
                let club: BoundedBTreeSet<T::AccountId, T::MaxClubSize> =
                        BoundedBTreeSet::new();
                <Clubs<T>>::insert(&club_number, club);
            };
            Self::deposit_event(Event::<T>::ClubCreated { club_number });

            Clubs::<T>::try_mutate(&club_number, |club| match club {
                None => Err(Error::<T>::ClubDoesNotExist.into()),
                Some(club) => club
                    .try_insert(user.clone())
                    .map_err(|_| Error::<T>::ClubFull),
            })?;
            Self::deposit_event(Event::<T>::UserAddedToClub { club_number, user });
            Ok(())
        }

        /// Remove a user from a club.
        #[pallet::weight(50_000_000)]
        pub fn remove_user_from_club(
            origin: OriginFor<T>,
            club_number: u32,
            user: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Clubs::<T>::try_mutate(&club_number, |club| -> DispatchResult {
                match club {
                    None => Err(Error::<T>::ClubDoesNotExist.into()),
                    Some(club) => {
                        let user_exists = club.remove(&user);
                        if !user_exists {
                            Err(Error::<T>::UserNotInClub.into())
                        } else {
                            Ok(())
                        }
                    }
                }
            })?;
            Self::deposit_event(Event::<T>::UserRemovedFromClub { club_number, user });
            Ok(())
        }
    }
}
