#![cfg_attr(not(feature = "std"), no_std)]

// edit the file to define custom logic or remove it if it is not needed.

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    // Configure the pallet by specifying the parameters and types on which it depends
    #[pallet::config]
    pub trait Config: frame_system::Config {
        // because this pallet emits events, it depends on the runtime's definition of an event
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // The pallet's runtime storage item
    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;

    // Pallets use events to inform users when important changes are made
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Event documentation should end with an array that provides description names for event parameters
        SomethingStored(u32, T::AccountId),
    }

    // Errors inform users that something went wrong
    #[pallet::error]
    pub enum Error<T> {
        // Error names should be descriptive
        NoneValue,
        // Errors should have helpful documentation associated with them.
        StorageOverflow
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrincis" which are often compared to transaction.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult
    #[pallet::call]
    impl<T:Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer
            // This function will return an error if the extrinsic is not signed
            let who = ensure_signed(origin)?;

            // update storage
            <Something<T>>::put(something);

            // emit event
            Self::deposit_event(Event::SomethingStored(something, who));

            // return a successful DispatchResultWithPostInfo
            Ok(().into())
        }

        // An example dispatchable that may throw a custom error
        #[pallet::weight(10_000 + T::DbWeight::get().read_writes(1, 1))]
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let _who = ensure_signed(origin)?;

            // read a value from storage
            match <Something<T>>::get() {
                // return an error if the value has not been set
                None => Err(Error::<T>::NoneValue)?,
                Some(old) => {
                    // Increment the value read from storage will error in the event of overflow
                    let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;

                    // Update the value in storage with the incremented result
                    <Something<T>>::put(new);
                    Ok(().into())
                },
            }
        }
    }
}