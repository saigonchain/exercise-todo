#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::Vec, pallet_prelude::*};
use frame_system::pallet_prelude::*;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Task {
	id: u32,
	description: Vec<u8>,
	done: bool,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, Default)]
#[scale_info(skip_type_params(T))]
pub struct ToDoList {
	tasks: Vec<Task>,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::log::Log;
	use scale_info::prelude::vec;

	#[pallet::pallet]
	#[pallet::without_storage_info]
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
	#[pallet::getter(fn get_todo)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub(super) type Todo<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, ToDoList, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn task_count)]
	pub(super) type TaskCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddedTask { owner: T::AccountId, task_id: u32 },
		UpdatedTask { task_id: u32 },
		RemovedTask { task_id: u32 },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		TaskIdNotFound,
		NotOwner,
		BoundsOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn add_task(origin: OriginFor<T>, description: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Get Counter
			let count = TaskCount::<T>::get();
			let new_count = count.checked_add(1).ok_or(Error::<T>::BoundsOverflow)?;

			// create new task
			let task = Task { id: new_count, description, done: false };

			// get owned task list and add newtask
			// if Todo::<T>::get(&who).is_some() {
			// 	let mut todo_list = Todo::<T>::get(&who).unwrap();
			// 	todo_list.tasks.push(task);
			// 	Todo::<T>::insert(&who, todo_list);
			// } else {
			// 	let todo_list = ToDoList { tasks: vec![task] };
			// 	Todo::<T>::insert(&who, todo_list);
			// }

			let todos = match Todo::<T>::get(&who) {
				Some(mut todos) => {
					todos.tasks.push(task);
					todos
				},
				None => {
					let new_todo = ToDoList { tasks: vec![task] };
					new_todo
				},
			};
			Todo::<T>::insert(&who, todos);
			//mutate
			// if let Some
			// match

			// update count
			TaskCount::<T>::put(new_count);

			// Emit an event.
			Self::deposit_event(Event::AddedTask { owner: who, task_id: new_count });

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(10_000)]
		pub fn remove_task(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;
			//let count = TaskCount::<T>::get();

			// check task_id exist
			//ensure!(task_id <= count, Error::<T>::TaskIdNotFound);
			ensure!(Todo::<T>::contains_key(&who), Error::<T>::TaskIdNotFound);

			// check task who owned task_id
			// if Todo::<T>::get(&who).is_some() {
			// 	let mut todo_list = Todo::<T>::get(&who).unwrap();
			// 	let index_to_remove = todo_list.tasks.iter().position(|task| task.id == task_id);
			// 	match index_to_remove {
			// 		Some(index) => {
			// 			todo_list.tasks.remove(index);
			// 			Todo::<T>::insert(&who, todo_list);
			// 		},
			// 		None => return Err(Error::<T>::NotOwner.into()),
			// 	}
			// } else {
			// 	return Err(Error::<T>::NotOwner.into())
			// }

			let mut todo = Todo::<T>::get(&who).ok_or(Error::<T>::NotOwner)?;

			// cách này dùng dc, nhưng sẽ vấn đề nếu ko có task id 
			todo.tasks.retain(|task| task_id != task.id);

			Todo::<T>::insert(&who, todo);

			// Emit an event.
			Self::deposit_event(Event::RemovedTask { task_id });

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(10_000)]
		pub fn update_task(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;
			//let count = TaskCount::<T>::get();

			// check task_id exist
			//ensure!(task_id <= count, Error::<T>::TaskIdNotFound);
			ensure!(Todo::<T>::contains_key(&who), Error::<T>::TaskIdNotFound);

			// check task who owned task_id
			// if Todo::<T>::get(&who).is_some() {
			// 	let mut todo_list = Todo::<T>::get(&who).unwrap();
			// 	if let Some(update_task) =
			// 		todo_list.tasks.iter_mut().find(|task| task.id == task_id)
			// 	{
			// 		update_task.done = true;
			// 		Todo::<T>::insert(&who, todo_list);
			// 	} else {
			// 		return Err(Error::<T>::NotOwner.into())
			// 	}
			// } else {
			// 	return Err(Error::<T>::NotOwner.into())
			// }

			Todo::<T>::try_mutate(&who, |todos| -> DispatchResult {
				if let Some(todo) = todos {
					let task = todo.tasks.iter_mut().find(|task| task.id == task_id && !task.done);

					match task {
						Some(t) => {
							t.done = true;
							Ok(())
						},
						None => return Err(Error::<T>::NotOwner.into()),
					}
				} else {
					return Err(Error::<T>::NotOwner.into())
				}
			})?;
			// Emit an event.
			Self::deposit_event(Event::UpdatedTask { task_id });

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
