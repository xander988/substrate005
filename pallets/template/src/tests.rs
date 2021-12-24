use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;
	#[test]
	fn create_claim_works(){
		new_test_ext().execute_with(|| {
			let claim = vec![0,1];
			assert_ok!(TemplateModule::create_claim(Origin::signed(1), claim.clone()));
			assert_eq!(Proofs::<Test>::get(&claim),(1, frame_system::Pallet::<Test>::block_number()));
		})
	}

	#[test]
	fn create_claim_failed_when_claim_already_exist(){
		new_test_ext().execute_with(|| {
			let claim = vec![0, 1];
			let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
			assert_noop!(
				TemplateModule::create_claim(Origin::signed(1), claim.clone()),
				Error::<Test>::ProofAlreadyExist
			);
		})
	}


	#[test]
	fn revoke_claim_works(){
		new_test_ext().execute_with(|| {
			let claim = vec![0, 1];
			let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
			assert_ok!( TemplateModule::revoke_claim(Origin::signed(1), claim.clone()) );
			assert_eq!(Proofs::<Test>::get(&claim), None);

		})
	}

	#[test]
	fn revoke_claim_failed_when_claim_is_not_exist(){
		new_test_ext().execute_with(|| {
			let claim = vec![0, 1];
			assert_noop!(
				TemplateModule::revoke_claim(Origin::signed(1), claim.clone()),
				Error::<Test>::ClaimedNotExist
			);
		})
	}

	#[test]
	fn revoke_claim_failed_with_wrong_owner(){
		new_test_ext().execute_with(|| {
			let claim = vec![0, 1];
			let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
			assert_noop!(
				TemplateModule::revoke_claim(Origin::signed(2), claim.clone()),
				Error::<Test>::NotProofOwner
			);
		})
	}	

	#[test]
	fn transfer_claim_works(){
		new_test_ext().execute_with(||{
			let claim: Vec<u8> = vec![0,1];
			let _ = TemplateModule::create_claim(Origin::signed(1), claim.clone());
			let _ = TemplateModule::transfer_claim(Origin::signed(1), claim.clone(), 2);
			let (owner, _) = Proofs::<Test>::get(&claim);
			assert_eq!( owner, 2);
		})
	}




