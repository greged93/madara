use frame_support::{assert_ok, bounded_vec};
use mp_starknet::crypto::commitment::calculate_invoke_tx_hash;
use mp_starknet::execution::types::Felt252Wrapper;
use mp_starknet::transaction::types::{EventWrapper, InvokeTransaction, TransactionReceiptWrapper, TxType};

use crate::tests::cairo1::get_invoke_dummy;
use crate::tests::mock::*;

#[test]
fn given_hardcoded_contract_run_invoke_tx_then_it_works() {
    new_test_ext().execute_with(|| {
        basic_test_setup(2);

        let none_origin = RuntimeOrigin::none();

        let transaction: InvokeTransaction = get_invoke_dummy().into();
        let chain_id = Starknet::chain_id();
        let transaction_hash = calculate_invoke_tx_hash(transaction.clone(), chain_id);

        assert_ok!(Starknet::invoke(none_origin, transaction));

        let pending = Starknet::pending();
        pretty_assertions::assert_eq!(pending.len(), 2);

        let receipt = &pending.get(0).unwrap().1;
        let expected_receipt = TransactionReceiptWrapper {
            transaction_hash: Felt252Wrapper::from_hex_be(
                "0x01b8ffedfb222c609b81f301df55c640225abaa6a0715437c89f8edc21bbe5e8",
            )
            .unwrap(),
            actual_fee: Felt252Wrapper::from(53510_u128),
            tx_type: TxType::Invoke,
            events: bounded_vec![EventWrapper {
                keys: bounded_vec!(
                    Felt252Wrapper::from_hex_be("0x0099cd8bde557814842a3121e8ddfd433a539b8c9f14bf31ebf108d12e6196e9")
                        .unwrap(),
                ),
                data: bounded_vec![
                    Felt252Wrapper::from_hex_be("0x0").unwrap(),
                    Felt252Wrapper::from_hex_be("0x000000000000000000000000000000000000000000000000000000000000dead")
                        .unwrap(),
                    Felt252Wrapper::from_hex_be("0x000000000000000000000000000000000000000000000000000000000000d106")
                        .unwrap(),
                    Felt252Wrapper::ZERO,
                ],
                from_address: Starknet::fee_token_address(),
                transaction_hash
            },],
        };

        pretty_assertions::assert_eq!(*receipt, expected_receipt);
    });
}
