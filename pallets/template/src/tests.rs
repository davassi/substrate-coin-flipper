use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use std::alloc::System;
use sp_runtime::traits::BadOrigin;

//const ALICE: SignedOrigin = 1u64;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		//assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		//assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		//System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}


