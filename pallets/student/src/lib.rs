#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, Default, Debug)]
	pub struct Student {
		name: [u8; 4],
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

	#[pallet::storage]
	#[pallet::getter(fn map_person_slice)]
	pub type Students<T: Config> = StorageMap<_, Blake2_128, T::AccountId, Student, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreatedStudent { account: T::AccountId },
		UpdatedStudent { account: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		StudentExisted,
		NotFoundStudent,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn create_student(
			origin: OriginFor<T>,
			name: [u8; 4],
			age: u16,
			grade: u8,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			//ensure_root(origin)?;
			// TODO
			// check student is existed (return StudentExisted) or not
			// Each student can only create information once
			//ensure!(!Self::map_person_slice(&who).is_some(), Error::<T>::StudentExisted);
			//ensure!(!Students::<T>::contains_key(&who), Error::<T>::StudentExisted);
			// TODO
			// Define new student
			// Update on chain storage
			// khi get thông tin thì 2 trường hợp
			// match
			// Some(value) , None

			// if Students::<T>::get(&who).is_some(){
			// 	let student = Student::<T>::get(&who).unwrap();
			// }
			// if Students::<T>::get(&who).is_some(){
			// 	return Err(Error::<T>::StudentExisted.into())

			// }
			// else {
			// 	let new_student = Student { name, age, grade };
			// 	<Students<T>>::insert(&who, new_student);

			// }
			if let Some(_) = Students::<T>::get(&who) {
				return Err(Error::<T>::StudentExisted.into())
			} else {
				let new_student = Student { name, age, grade };
				<Students<T>>::insert(&who, new_student);
			}

			Self::deposit_event(Event::CreatedStudent { account: who });

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn update_student(origin: OriginFor<T>, age: u16, grade: u8) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// TODO
			// check student is existing or not (return NotFoundStudent)

			// let updated_student = match Students::<T>::get(&who) {
			// 	Some(mut s) => {
			// 		s.age = age;
			// 		s.grade = grade;
			// 		s
			// 	},

			// 	None => return Err(Error::<T>::NotFoundStudent.into()),
			// };
			// TODO
			// Get student info

			// TODO
			// Mutate student info

			// TODO
			// Update modified info to onchain storage
			//Students::<T>::insert(&who, updated_student);

			Students::<T>::mutate(&who, |student| {
				if let Some(s) = student {
					s.age = age;
					s.grade = grade;
				}
			});

			Self::deposit_event(Event::UpdatedStudent { account: who });
			Ok(())
		}
	}
}
