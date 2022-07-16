#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

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
    pub type Clubs<T: Config> = StorageMap<_, Twox64Concat, u32, BoundedBTreeSet<T::AccountId,T::MaxClubSize> >;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        /// Club created.
		ClubCreated { club_number: u32 },
        /// User added to club.
		UserAddedToClub { club_number: u32, user: T::AccountId },
		/// User removed from club.
		UserRemovedFromClub { club_number: u32, user: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Club already exists.
		ClubAlreadyExists,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
        /// Create a new club.
		#[pallet::weight(50_000_000)]
		pub fn create_club(
			origin: OriginFor<T>,
			club_number: u32,
		) -> DispatchResult {
			ensure_root(origin)?;
			match Clubs::<T>::get(club_number) {
				Some(_) => return Err(Error::<T>::ClubAlreadyExists.into()),
				None => {
					let club: BoundedBTreeSet<T::AccountId, T::MaxClubSize> = BoundedBTreeSet::new();
					<Clubs<T>>::insert(&club_number, club);
				},
			};
			Self::deposit_event(Event::<T>::ClubCreated { club_number });
			Ok(())
		}
	}
}
