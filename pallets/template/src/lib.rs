#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use scale_info::prelude::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	

	#[derive(
		Encode,
		Decode,
		TypeInfo,      // hỗ trợ SCALE Codec
		MaxEncodedLen, // giới hạn chiều dài của struct khi encode
		Default,       // khởi tạo impl Default cho struct
		Debug,
	)] // hỗ trợ impl printable
	pub struct StudentSlice {
		name: [u8; 32],
		age: u16,
		grade: u8,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn something_value)]
	pub type SomethingValue<T> = StorageValue<_, u32, ValueQuery>;

	/*
	   Map Storage
	*/
	#[pallet::storage]
	#[pallet::getter(fn map_option)]
	pub type OptionMap<T> = StorageMap<_, Twox128, u8, u8>;

	#[pallet::storage]
	#[pallet::getter(fn map_value)]
	pub type ValueMap<T> = StorageMap<_, Twox128, u8, u8, ValueQuery>;

	/*
	   Map Storage with Struct
	*/
	#[pallet::storage]
	#[pallet::getter(fn map_person_slice)]
	pub type StudentSliceMapStorage<T: Config> =
		StorageMap<_, Blake2_128, T::AccountId, StudentSlice, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		DivideZero,

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000_000)]
		pub fn insert_person_slice(origin: OriginFor<T>, name: Vec<u8>, age: u16, grade: u8) -> DispatchResult {
			let _who = ensure_signed(origin)?;
		 
			let p = StudentSlice{
				name : Self::convert_str_to_slice(&name), 
				age: age,
				grade: grade
			};
 
			// <StudentSliceMapStorage<T>>::insert(_who, p);
			StudentSliceMapStorage::<T>::insert(_who.clone(), p);
			// let pslice: StudentSlice = StudentSliceMapStorage::<T>::get(_who.clone());

 

			Ok(())	
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000_000)]
		pub fn get_something(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
		   
		   // get value by getter
			let s1 = Self::something();
			let s2 = Self::something_value();
			
			// get value by alias type
			let a1 = Something::<T>::get();
			let a2 = <Something<T>>::get();

			// put value 
			SomethingValue::<T>::put(1);
			Something::<T>::put(1); 

			// insert map
			ValueMap::<T>::insert(1, 10);
			<ValueMap<T>>::insert(2, 20);

			// Query map
			let map1 = ValueMap::<T>::get(0);
			let map1 = <ValueMap<T>>::get(1);
			let map1 = Self::map_value(2);
 
			

			Ok(())	      
		}

		#[pallet::call_index(4)]
		#[pallet::weight(10_000_000)]
		pub fn div_number(origin: OriginFor<T>, dividend_number: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;
		   
		   // cach 1
			ensure!(dividend_number == 0, <Error<T>>::DivideZero);

			// cach 2
			if dividend_number == 0 {
				return Err(<Error<T>>::DivideZero.into());
			}
			
			let something = Something::<T>::get();
			match something {
				None => return Err(Error::<T>::NoneValue.into()),
				Some(value) => {
					// cach 3
					let new_value = value.checked_div(dividend_number).ok_or(Error::<T>::StorageOverflow)?;
					Something::<T>::put(new_value);
					
				}
			}


			Ok(())	      
		}

		
	}
}


impl<T: Config> Pallet<T> {
		
	fn convert_str_to_slice(_str: &Vec<u8>) -> [u8; 32] {
		let bytes = _str;
		let mut array:  [u8; 32] = [0; 32];
		frame_support::log::info!("called by {:?}", bytes);
		 
		let mut length = 32;
		if bytes.len() < 32 {
			length = bytes.len();
		}
		
		for i in 0..length {
			array[i] = bytes[i];
		}


		return array;
	}
}
