use crate::tests::mock::{account_helper, AccountType};

#[test]
fn given_salt_should_calculate_new_contract_addr() {
    let (addr_0, _, _) =
        account_helper("0x000000000000000000000000000000000000000000000000000000000000BEEF", AccountType::Argent, 0);
    let (addr_1, _, _) =
        account_helper("0x000000000000000000000000000000000000000000000000000000000000DEAD", AccountType::Argent, 0);
    assert_ne!(addr_0, addr_1);
}
