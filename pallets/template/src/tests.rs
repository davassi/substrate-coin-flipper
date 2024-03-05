use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BadOrigin;

type SignedOrigin = u64;

const ALICE: SignedOrigin = 1u64;

#[test]
fn create_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		System::assert_has_event(Event::CoinCreated { who: ALICE }.into());
	});
}

#[test]
fn create_coin_with_an_unsigned_user_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		let result = TemplateModule::create_coin(RuntimeOrigin::none());
		assert_noop!(result, BadOrigin);
	});
}

#[test]
fn create_coin_twice_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		let result = TemplateModule::create_coin(origin);
		assert_noop!(result, Error::<Test>::CoinAlreadyExists);
	});
}

#[test]
fn flip_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		let result = TemplateModule::do_flip(origin);
		assert_ok!(result);

		System::assert_has_event(Event::CoinFlipped { who: ALICE }.into());
	});
}

#[test]
fn flip_coin_with_an_unsigned_user_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		let result = TemplateModule::do_flip(RuntimeOrigin::none());
		assert_noop!(result, BadOrigin);
	});
}

#[test]
fn flip_coin_without_creating_a_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::do_flip(origin);
		assert_noop!(result, Error::<Test>::CoinDoesNotExist);
	});
}

#[test]
fn toss_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		let result = TemplateModule::do_toss(origin);
		assert_ok!(result);

		System::assert_has_event(Event::CoinTossed { who: ALICE }.into());
	});
}

#[test]
fn toss_coin_with_an_unsigned_user_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		let result = TemplateModule::do_toss(RuntimeOrigin::none());
		assert_noop!(result, BadOrigin);
	});
}

#[test]
fn toss_coin_without_creating_a_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::do_toss(origin);
		assert_noop!(result, Error::<Test>::CoinDoesNotExist);
	});
}

#[test]
fn toss_coin_with_an_error_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		let result = TemplateModule::do_toss(origin);
		assert_noop!(result, Error::<Test>::CoinDoesNotExist);
	});
}

#[test]
fn create_coin_and_flip_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		let result = TemplateModule::do_flip(origin);
		assert_ok!(result);

		System::assert_has_event(Event::CoinCreated { who: ALICE }.into());
		System::assert_has_event(Event::CoinFlipped { who: ALICE }.into());
	});
}

#[test]
fn create_coin_and_toss_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		let result = TemplateModule::do_toss(origin);
		assert_ok!(result);

		System::assert_has_event(Event::CoinCreated { who: ALICE }.into());
		System::assert_has_event(Event::CoinTossed { who: ALICE }.into());
	});
}

#[test]
fn create_coin_and_flip_coin_and_toss_coin_test() {
	new_test_ext().execute_with(|| {
		
		System::set_block_number(1);
		
		let origin = RuntimeOrigin::signed(ALICE);

		let result = TemplateModule::create_coin(origin);
		assert_ok!(result);

		let result = TemplateModule::do_flip(origin);
		assert_ok!(result);

		let result = TemplateModule::do_toss(origin);
		assert_ok!(result);

		System::assert_has_event(Event::CoinCreated { who: ALICE }.into());
		System::assert_has_event(Event::CoinFlipped { who: ALICE }.into());
		System::assert_has_event(Event::CoinTossed { who: ALICE }.into());
	});
}
