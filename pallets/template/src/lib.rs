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
	
	#[pallet::storage]
	pub type CoinStorage<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Coin, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CoinCreated { who: AccountIdOf<T> },
		CoinFlipped { who: AccountIdOf<T> },
		CoinTossed { who: AccountIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		CoinAlreadyExists,
		CoinDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create_coin(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_coin(&who)?;
			Self::deposit_event(Event::CoinCreated { who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn do_flip(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_flip_coin(&who)?;
			Self::deposit_event(Event::CoinFlipped { who });
			Ok(())
		}

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

			let blockumber = <frame_system::Pallet<T>>::block_number();
			
			// Use the random value to decide the coin's new side
			// This is a simplistic approach; your actual implementation may vary based on your randomness source
			let new_side = if Self::generate_insecure_random_boolean(0) == true {
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
