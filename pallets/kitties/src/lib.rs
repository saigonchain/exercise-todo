#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_support::{inherent::Vec, pallet_prelude::*};
use frame_system::pallet_prelude::*;

// Define Kitties
// #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
// #[scale_info(skip_type_params(T))]
// pub struct Kitty<AccountId> {
// 	pub dna: Vec<u8>,
// 	pub price: u64,
// 	pub gender: Gender,
// 	pub owner: AccountId
// }

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Kitty<T: Config> {
	pub dna: Vec<u8>,
	pub price: u64,
	pub gender: Gender,
	pub owner: T::AccountId,
}

// Define Gender
#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum Gender {
	Male,
	Female,
}

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
	}

	// TODO : Define KittyId storage
	#[pallet::storage]
	#[pallet::getter(fn kitty_id)]
	pub(super) type KittyId<T> = StorageValue<_, u32, ValueQuery>;

	//TODO : Define Kitties storage + OptionQuery
	// dna => kitty
	#[pallet::storage]
	#[pallet::getter(fn get_kitty)]
	pub type Kitties<T> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>>;

	//TODO : Define KittiesOwned storage + ValueQuery
	#[pallet::storage]
	#[pallet::getter(fn kitty_owned)]
	pub(super) type KittiesOwned<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { kitty: Vec<u8>, owner: T::AccountId },
		// Transfer
		// Buy  
		// Set Price 
		
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DuplicateKitty,
		OverFlow
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_kitty(origin: OriginFor<T>, dna: Vec<u8>) -> DispatchResult {
			// Make sure the caller is from a signed origin
			let owner = ensure_signed(origin)?;

			//TODO : generate gender
			let gender = Self::gen_gender(&dna)?;
			// TODO: define new kitty


			// TODO: Check if the kitty does not already exist in our storage map
			// using ensure!
			ensure!(!Kitties::<T>::contains_key(&dna), Error::<T>::DuplicateKitty);
			// return DuplicateKitty if error
			let new_kitty = Kitty::<T> {
				dna: dna.clone(),
				gender,
				price: 0, 
				owner: owner.clone()
			
			};

			// TODO: Get current kitty id
			let current_id = Self::kitty_id();


			// TODO: Increase kitty Id by 1 (if overflow return OverFlow)
			let next_id = current_id.checked_add(1).ok_or(Error::<T>::OverFlow)?;
			// TODO: Append new kitty to KittiesOwned

			// let mut dnas = KittiesOwned::<T>::get(&owner);
			// dnas.push(dna.clone());
			// KittiesOwned::<T>::insert(&owner, dnas);

			KittiesOwned::<T>::append(&owner, dna.clone());
			// TODO: Write new kitty to storage
			Kitties::<T>::insert(&dna, new_kitty);
			// TODO: Write new kitty id
			KittyId::<T>::put(next_id);
			// Deposit our "Created" event.
			Self::deposit_event(Event::Created { kitty: dna, owner: owner.clone() });

			Ok(())
		}
	}
}

// helper function
impl<T> Pallet<T> {
	fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error<T>> {
		if dna.len() % 2 == 0 {
			return Ok(Gender::Male)
		} else {
			return Ok(Gender::Female)
		}
	}
}
