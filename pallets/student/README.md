## Pallet Architecture

### One basic pallet consists of six components as follows:
1. Imports and Dependencies
2. Declaration of the Pallet type
3. Runtime Configuration Trait
4. Runtime Storage
5. Runtime Events
6. Extrinsics

### FRAME
	> Framework for Runtime Aggregation of Modularized Entities

1. System Pallets : frame_system, frame_support, frame_executive

2. Functional Pallets: pallet-balance, pallet-staking, ...

3. Parachain Pallets: 

### Define on chain storage 

1. Define storage value:

**Return OptionQuery**
> StorageValue
```rust
#[pallet::storage]   // require
#[pallet::getter(fn something)] //define getter function   
pub type Something<T> = StorageValue<_, u32>;  // define alias type with generic type T
    // Some(value) or None
```

> StorageMap
```rust
#[pallet::storage]
#[pallet::getter(fn map_option)]
pub type OptionMap<T> = StorageMap<_, Twox128, u8, u32> ; // Hasher -> key -> value

```



**Return ValueQuery**
> StorageValue
```rust
#[pallet::storage]   // require
#[pallet::getter(fn something)]  //getter function    	
pub type SomethingValue<T> = StorageValue<_, u32, ValueQuery>; // add ValueQuery 

```
> StorageMap

```rust
#[pallet::storage]
#[pallet::getter(fn map_value)]
pub type ValueMap<T> = StorageMap<_, Twox128, u8, u32, ValueQuery> ; // Hasher -> key -> value -> Return Value query

```


### Access on chain storage (get or update)
> StorageValue : Get
```rust
Self::something();
```
or
```rust
Something::<T>::get();
```

or
```rust
<Something<T>>::get();
```

> StorageValue : Update

```rust
Something::<T>::put(value);  
```
or 
```rust
<Something<T>>::put(value);
```

> StorageMap : Get
 
```rust
Seft::map_option(key);
```
or
```rust
ValueMap::<T>::get(key)
```

or
```rust
<ValueMap<T>>::get(key)
```


> StorageMap : Update
```rust 
ValueMap::<T>::insert(key, value)
```
or
```rust
<ValueMap<T>>::insert(key, value)
```
### Define Event by using macro and enum
```rust

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}
```

### Define Error by using macro and enum
```rust
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		DivideZero,

	}

```
