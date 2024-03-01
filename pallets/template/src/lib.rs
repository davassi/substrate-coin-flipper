#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

use frame_system::pallet_prelude::OriginFor;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::sp_runtime::traits::AccountIdConversion;
	use frame_support::pallet_prelude::{OptionQuery, *};
	use frame_support::PalletId;
	use frame_support::traits::Randomness;
	use frame_system::pallet_prelude::*;
	
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type PalletId: Get<PalletId>;

		type MyRandomness: Randomness<Self::Hash, u32>;
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, PartialOrd, Default)]
	enum CoinSide {
		#[default]
		Head,
		Tail,
	}
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, PartialOrd, Default)]
	pub struct Coin {
		side: CoinSide,
	}
	
	// The pallet's runtime storage items.
	//
	// StorageMap { Account => Coin }: Each Account has a Coin
	#[pallet::storage]
	pub type CoinStorage<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Coin, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Event emitted when a coin is created. 
		CoinCreated { who: AccountIdOf<T> },
		// Event emitted when a coin is flipped. 
		CoinFlipped { who: AccountIdOf<T> },
		// Event emitted when a coin is tossed. 
		CoinTossed { who: AccountIdOf<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		// Error returned when a coin already exists
		CoinAlreadyExists,
		// Error returned when a coin does not exist
		CoinDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Create a coin for the sender's account and save it in the StorageMap
		///
		/// - `origin`: The sender's account
		/// 
		/// It generates a new event when a coin is created
		/// - Event: `CoinCreated`
		///
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create_coin(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_coin(&who)?;
			Self::deposit_event(Event::CoinCreated { who });
			Ok(())
		}

		/// Flip the coin (head to tail or tail to head) and update the StorageMap
		///
		/// - origin: The sender's account
		///
		/// It generates a new event when a coin is flipped
		/// - Event: `CoinFlipped`
		///
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_flip(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_flip_coin(&who)?;
			Self::deposit_event(Event::CoinFlipped { who });
			Ok(())
		}

		/// Toss the coin for the sender
		///
		/// - origin: The sender's account
		///
		/// It generates a new event when a coin is tossed
		/// - Event: `CoinTossed`
		///
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn do_toss(origin: OriginFor<T>) -> DispatchResult {
			let who : AccountIdOf<T> = ensure_signed(origin)?;
			Self::do_toss_coin(&who)?;
			Self::deposit_event(Event::CoinTossed { who });
			Ok(())
		}
	}
	impl<T: Config> Pallet<T> {

		// This method generates the palled account id
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}

		// This method creates a new coin for the given account
		pub fn do_create_coin(account_id: &T::AccountId) -> DispatchResult {

			if CoinStorage::<T>::contains_key(account_id) {
				// If a coin already exists, return an error
				return Err(Error::<T>::CoinAlreadyExists.into());
			} 
			
			// Create a new coin
			CoinStorage::<T>::insert(account_id, Coin::default());
			Ok(())
		}

		// This method flips the coin for the given account
		pub fn do_flip_coin(account_id: &T::AccountId) -> DispatchResult {
			
			// If a coin does not exist, return an error
			let mut coin = CoinStorage::<T>::get(account_id)
				.ok_or(Error::<T>::CoinDoesNotExist)?;

			// Flip the coin
			coin.side = match coin.side {
				CoinSide::Head => CoinSide::Tail,
				CoinSide::Tail => CoinSide::Head,
			};
			
			// Update the coin
			CoinStorage::<T>::insert(account_id, coin);
			
			Ok(())
		}

		// This method tosses the coin for the given account
		pub fn do_toss_coin(account_id: &T::AccountId) -> DispatchResult {
			let mut coin = CoinStorage::<T>::get(account_id)
				.ok_or(Error::<T>::CoinDoesNotExist)?;

			let block_number = <frame_system::Pallet<T>>::block_number();
			let seed = block_number.try_into().unwrap_or_else(|_| 0u32);

			// Use the random value to decide the coin's new side
			// This is very a simpcalistic approach that uses blocknunber as seed source. Never use it in production. 
			let new_side = if Self::generate_insecure_random_boolean(seed) == true {
				CoinSide::Head
			} else {
				CoinSide::Tail
			};
		
			// Update the coin's side
			coin.side = new_side;
			CoinStorage::<T>::insert(account_id, coin);
			
			Ok(())
		}

		// You should call this function with different seed values 	
		pub fn generate_insecure_random_boolean(seed: u32) -> bool {
			let (random_seed, _) = T::MyRandomness::random(&(T::PalletId::get(), seed).encode());
			let random_number = <u32>::decode(&mut random_seed.as_ref())
				.expect("secure hashes should always be bigger than u32; qed");
			random_number % 2 == 0
		}
	}


}
