use crate as pallet_template;
use frame_support::{parameter_types,traits::{ConstU16, ConstU64}, PalletId};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		TemplateModule: pallet_template,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const CoinFlipperPalletId: PalletId = PalletId(*b"coinflip");
}


impl pallet_template::Config for Test {
	type PalletId = CoinFlipperPalletId;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MyRandomness = TestRandomness<Self>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}


//
// Mock implementations
//

use frame_system::pallet_prelude::BlockNumberFor;

pub struct TestRandomness<T>(frame_support::pallet_prelude::PhantomData<T>);

impl<Output: codec::Decode + Default, T>
	frame_support::traits::Randomness<Output, BlockNumberFor<T>> for TestRandomness<T>
where
	T: frame_system::Config,
{
	fn random(subject: &[u8]) -> (Output, BlockNumberFor<T>) {
		use sp_runtime::traits::TrailingZeroInput;

		(
			Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default(),
			frame_system::Pallet::<T>::block_number(),
		)
	}
}