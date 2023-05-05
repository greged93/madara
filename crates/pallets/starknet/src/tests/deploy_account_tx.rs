use core::str::FromStr;

use frame_support::{assert_err, assert_ok, bounded_vec, BoundedVec};
use hex::FromHex;
use mp_starknet::transaction::types::{DeployAccountTransaction, EventWrapper};
use sp_core::{H256, U256};

use super::mock::*;
use crate::{Error, Event, StorageView};

#[test]
fn given_contract_run_deploy_account_tx_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(0);
        run_to_block(2);
        // pedersen(sn_keccak(b"ERC20_balances"),
        // 0x00a13c294af26c4e940d28b1db914e4bb28158638deeeb4ae9ca9b37ab3e4a97) which is the key in the
        // starknet contract for
        // ERC20_balances(0x015c7ddf2e57acc45be4c0dfa3c7b1aca474eeaab0041b89a30317238401c888).low
        StorageView::<MockRuntime>::insert(
            (
                Starknet::fee_token_address(),
                H256::from_str("0x02450cb55e7682ffca9e1db504e2de1263d587242b9b43e27a1eedf18b4bbabf").unwrap(),
            ),
            U256::from(u128::MAX),
        );
        // pedersen(sn_keccak(b"ERC20_balances"),
        // 0x00a13c294af26c4e940d28b1db914e4bb28158638deeeb4ae9ca9b37ab3e4a97) + 1 which is the key in the
        // starknet contract for
        // ERC20_balances(0x015c7ddf2e57acc45be4c0dfa3c7b1aca474eeaab0041b89a30317238401c888).high
        StorageView::<MockRuntime>::insert(
            (
                Starknet::fee_token_address(),
                H256::from_str("0x02450cb55e7682ffca9e1db504e2de1263d587242b9b43e27a1eedf18b4bbabf").unwrap(),
            ),
            U256::from(u128::MAX),
        );
        let none_origin = RuntimeOrigin::none();
        // TEST ACCOUNT CONTRACT
        // - ref testnet tx(0x0751b4b5b95652ad71b1721845882c3852af17e2ed0c8d93554b5b292abb9810)
        let salt = "0x03b37cbe4e9eac89d54c5f7cc6329a63a63e8c8db2bf936f981041e086752463";
        let (test_addr, account_class_hash, calldata) = no_validate_account_helper(salt);

        let transaction = DeployAccountTransaction {
            account_class_hash,
            sender_address: test_addr,
            salt: U256::from_str(salt).unwrap(),
            version: 1,
            calldata: BoundedVec::try_from(calldata.clone().into_iter().map(U256::from).collect::<Vec<U256>>())
                .unwrap(),
            nonce: U256::zero(),
            signature: bounded_vec!(),
            max_fee: U256::from(u128::MAX),
        };

        assert_ok!(Starknet::deploy_account(none_origin, transaction));
        assert_eq!(Starknet::contract_class_hash_by_address(test_addr).unwrap(), account_class_hash);
        let expected_fee_transfer_event = Event::StarknetEvent(EventWrapper {
                keys: bounded_vec![
                    H256::from_str("0x0099cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9").unwrap()
                ],
                data: bounded_vec!(
                    H256::from_slice(&test_addr), // From
                    H256::from_str("0x0000000000000000000000000000000000000000000000000000000000000002").unwrap(), // To
                    H256::from_str("0x000000000000000000000000000000000000000000000000000000000000d3b8").unwrap(), // Amount low
                    H256::zero(), // Amount high
                ),
                from_address: Starknet::fee_token_address(),
            }).into();
        System::assert_last_event(expected_fee_transfer_event)
    });
}

#[test]
fn given_contract_run_deploy_account_tx_twice_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(0);
        run_to_block(2);
        // pedersen(sn_keccak(b"ERC20_balances"),
        // 0x00a13c294af26c4e940d28b1db914e4bb28158638deeeb4ae9ca9b37ab3e4a97) which is the key in the
        // starknet contract for
        // ERC20_balances(0x015c7ddf2e57acc45be4c0dfa3c7b1aca474eeaab0041b89a30317238401c888).low
        StorageView::<MockRuntime>::insert(
            (
                Starknet::fee_token_address(),
                H256::from_str("0x02450cb55e7682ffca9e1db504e2de1263d587242b9b43e27a1eedf18b4bbabf").unwrap(),
            ),
            U256::from(u128::MAX),
        );
        // pedersen(sn_keccak(b"ERC20_balances"),
        // 0x00a13c294af26c4e940d28b1db914e4bb28158638deeeb4ae9ca9b37ab3e4a97) + 1 which is the key in the
        // starknet contract for
        // ERC20_balances(0x015c7ddf2e57acc45be4c0dfa3c7b1aca474eeaab0041b89a30317238401c888).high
        StorageView::<MockRuntime>::insert(
            (
                Starknet::fee_token_address(),
                H256::from_str("0x02450cb55e7682ffca9e1db504e2de1263d587242b9b43e27a1eedf18b4bbabf").unwrap(),
            ),
            U256::from(u128::MAX),
        );
        let none_origin = RuntimeOrigin::none();
        let salt = "0x03b37cbe4e9eac89d54c5f7cc6329a63a63e8c8db2bf936f981041e086752463";
        let (test_addr, account_class_hash, calldata) = no_validate_account_helper(salt);

        // TEST ACCOUNT CONTRACT
        // - ref testnet tx(0x0751b4b5b95652ad71b1721845882c3852af17e2ed0c8d93554b5b292abb9810)
        let transaction = DeployAccountTransaction {
            account_class_hash,
            sender_address: test_addr,
            calldata: BoundedVec::try_from(calldata.clone().into_iter().map(U256::from).collect::<Vec<U256>>())
                .unwrap(),

            salt: U256::from_str(salt).unwrap(),
            version: 1,
            nonce: U256::zero(),
            signature: bounded_vec!(),
            max_fee: U256::from(u128::MAX),
        };

        assert_ok!(Starknet::deploy_account(none_origin.clone(), transaction.clone()));
        // Check that the account was created
        assert_eq!(Starknet::contract_class_hash_by_address(test_addr).unwrap(), account_class_hash);
        assert_err!(Starknet::deploy_account(none_origin, transaction), Error::<MockRuntime>::AccountAlreadyDeployed);
    });
}

#[test]
fn given_contract_run_deploy_account_tx_undeclared_then_it_fails() {
    new_test_ext().execute_with(|| {
        System::set_block_number(0);
        run_to_block(2);
        let salt = "0x03b37cbe4e9eac89d54c5f7cc6329a63a63e8c8db2bf936f981041e086752463";
        let none_origin = RuntimeOrigin::none();
        let rand_address =
            <[u8; 32]>::from_hex("0000000000000000000000000000000000000000000000000000000000001234").unwrap();
        let (_, account_class_hash, _) = account_helper(salt);
        let transaction = DeployAccountTransaction {
            account_class_hash,
            sender_address: rand_address,
            version: 1,
            calldata: bounded_vec!(),
            nonce: U256::zero(),
            salt: U256::zero(),
            signature: bounded_vec!(),
            max_fee: U256::from(u128::MAX),
        };

        assert_err!(
            Starknet::deploy_account(none_origin, transaction),
            Error::<MockRuntime>::TransactionExecutionFailed
        );
    });
}

#[test]
fn given_contract_run_deploy_account_tx_fails_wrong_tx_version() {
    new_test_ext().execute_with(|| {
        System::set_block_number(0);
        run_to_block(2);

        let none_origin = RuntimeOrigin::none();
        // TEST ACCOUNT CONTRACT
        // - ref testnet tx(0x0751b4b5b95652ad71b1721845882c3852af17e2ed0c8d93554b5b292abb9810)
        let salt = "0x03b37cbe4e9eac89d54c5f7cc6329a63a63e8c8db2bf936f981041e086752463";
        let (test_addr, account_class_hash, calldata) = account_helper(salt);

        let wrong_tx_version = 50_u8;

        let transaction = DeployAccountTransaction {
            account_class_hash,
            sender_address: test_addr,
            version: wrong_tx_version,
            calldata: BoundedVec::try_from(calldata.clone().into_iter().map(U256::from).collect::<Vec<U256>>())
                .unwrap(),
            nonce: U256::zero(),
            salt: U256::zero(),
            signature: bounded_vec!(),
            max_fee: U256::from(u128::MAX),
        };

        assert_err!(
            Starknet::deploy_account(none_origin, transaction),
            Error::<MockRuntime>::TransactionExecutionFailed
        );
    });
}
