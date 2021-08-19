use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};


use super::*;

const PROOF_LIMIT_SET: u8 = 16;

#[test]
fn create_collection_works() {
    new_test_ext().execute_with(|| {
        let mut proof: Vec<u8> = vec![];
        for i in 0..PROOF_LIMIT_SET {
            proof.push(i);
        }
        let some_thing = vec![0, 1];
        let read_only = true;

        assert_ok!(PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only));

        assert_eq!(Proofs::<Test>::get(&proof), Collections{
            owner: 1,
            block_number: 0,
            some_thing,
            read_only: true,
            count: 1
        });


    })
}

#[test]
fn create_collection_failed_existed() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let some_thing = vec![0, 1];
        let read_only = true;
        let _ = PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only);

        assert_noop!(
            PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing, read_only),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn revoke_collection_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        // let proof2 = vec![1, 1];
        let some_thing = vec![0, 1];
        let read_only = true;
        let _ = PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only);

        assert_ok!(PoeModule::revoke_collection(Origin::signed(1), proof.clone()));

        assert_noop!(
            PoeModule::revoke_collection(Origin::signed(1), proof.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn revoke_collection_failed_not_owner() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        // let proof2 = vec![1, 1];
        let some_thing = vec![0, 1];
        let read_only = true;
        let _ = PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only);

        // assert_ok!(PoeModule::revoke_collection(Origin::signed(1), proof.clone()));
        assert_noop!(
            PoeModule::revoke_collection(Origin::signed(2), proof.clone()),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn revoke_collection_failed_not_existed() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];

        assert_noop!(
            PoeModule::revoke_collection(Origin::signed(1), proof.clone()),
            Error::<Test>::NoSuchProof
        );
    })
}


#[test]
fn transfer_collection_works() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let some_thing = vec![0, 1];
        let read_only = false;
        let _ = PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only);

        assert_ok!(PoeModule::transfer_connection(Origin::signed(1), proof.clone(), 2));

        assert_eq!(Proofs::<Test>::get(&proof), Collections{
            owner: 2,
            block_number: 0,
            some_thing,
            read_only: false,
            count: 2
        });

    })
}

#[test]
fn transfer_collection_failed_readonly() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let some_thing = vec![0, 1];
        let read_only = true;
        let _ = PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only);

        assert_noop!(
            PoeModule::transfer_connection(Origin::signed(1), proof.clone(), 2),
            Error::<Test>::ReadOnly
        );
    })
}

#[test]
fn transfer_collection_failed_not_owner() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];
        let some_thing = vec![0, 1];
        let read_only = false;
        let _ = PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only);

        assert_noop!(
            PoeModule::transfer_connection(Origin::signed(2), proof.clone(), 2),
            Error::<Test>::NotProofOwner
        );
    })
}

#[test]
fn transfer_collection_failed_not_existed() {
    new_test_ext().execute_with(|| {
        let proof = vec![0, 1];

        assert_noop!(
            PoeModule::transfer_connection(Origin::signed(1), proof.clone(), 2),
            Error::<Test>::NoSuchProof
        );
    })
}

#[test]
fn create_collection_failed_length_limit() {
    new_test_ext().execute_with(|| {
        let mut proof: Vec<u8> = vec![];
        for i in 0..PROOF_LIMIT_SET+1 {
            proof.push(i);
        }
        // println!("{}", proof.len());
        let some_thing = vec![0, 1];
        let read_only = true;

        assert_noop!(
            PoeModule::create_collection(Origin::signed(1), proof.clone(), some_thing.clone(), read_only),
            Error::<Test>::LengthLimited
        );

    })
}
