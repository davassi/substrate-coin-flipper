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
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {

		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, PartialOrd, Default)]
	enum CoinFace {
		#[default]
		Head,
		Tail,
	}
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, PartialOrd, Default)]
	pub struct Coin {
		side: CoinFace,
	}
	
	#[pallet::storage]
	pub type CoinStorage<T> = StorageMap<_, Blake2_128Concat, AccountIdOf<T>, Coin, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CoinCreated { something: u32, who: AccountIdOf<T> },
		CoinFlipped { who: AccountIdOf<T> },
		CoinTossed { who: AccountIdOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		CoinDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create_coin(origin: OriginFor<T>, something: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_coin(&who)?;
			Self::deposit_event(Event::CoinCreated { something, who });
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

		pub fn do_create_coin(account_id: &T::AccountId) -> DispatchResult {
			
			CoinStorage::<T>::insert(account_id, Coin { side: CoinFace::Head });
			Ok(())
		}

		pub fn do_flip_coin(account_id: &T::AccountId) -> DispatchResult {
			
			let coin = CoinStorage::<T>::get(account_id)
				.ok_or(Error::<T>::CoinDoesNotExist)?;

			let new_side = match coin.side {
				CoinFace::Head => CoinFace::Tail,
				CoinFace::Tail => CoinFace::Head,
			};
			CoinStorage::<T>::insert(account_id, Coin { side: new_side });
			
			Ok(())
		}

		pub fn do_toss_coin(account_id: &T::AccountId) -> DispatchResult {
			let coin = CoinStorage::<T>::get(account_id)
				.ok_or(Error::<T>::CoinDoesNotExist)?;
			
			Ok(())
		}
	}
}
