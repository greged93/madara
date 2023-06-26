use mp_starknet::execution::types::Felt252Wrapper;
use mp_starknet::transaction::types::{InvokeTransaction, Transaction};
use sp_core::bounded_vec;

use crate::tests::mock::Starknet;

mod invoke_tx;

// data ref: https://github.com/tdelabro/blockifier/blob/no_std-support/crates/blockifier/feature_contracts/account_without_validations.cairo
fn get_invoke_dummy() -> Transaction {
    let sender_address = Felt252Wrapper::from_hex_be(super::constants::NO_VALIDATE_ACCOUNT_ADDRESS_CAIRO_1).unwrap();
    let nonce = Felt252Wrapper::ZERO;
    let calldata = bounded_vec!(
        Felt252Wrapper::from_hex_be("0x024d1e355f6b9d27a5a420c8f4b50cea9154a8e34ad30fc39d7c98d3c177d0d7").unwrap(), /* contract_address */
        Felt252Wrapper::from_hex_be("0x00e7def693d16806ca2a2f398d8de5951344663ba77f340ed7a958da731872fc").unwrap(), /* selector */
        Felt252Wrapper::from_hex_be("0x0000000000000000000000000000000000000000000000000000000000000001").unwrap(), /* calldata_len */
        Felt252Wrapper::from_hex_be("0x0000000000000000000000000000000000000000000000000000000000000019").unwrap(), /* calldata[0] */
    );

    InvokeTransaction {
        version: 1,
        sender_address,
        calldata,
        nonce,
        signature: bounded_vec!(),
        max_fee: Felt252Wrapper::from(u128::MAX),
    }
    .from_invoke(Starknet::chain_id())
}
