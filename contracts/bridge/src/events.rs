use crate::{Address, U128};
use ontio_std::abi::EventBuilder;

pub fn new_pending_admin_event(new_pending_admin: &Address) {
    EventBuilder::new()
        .string("setPendingAdmin")
        .address(new_pending_admin)
        .notify();
}

pub fn new_admin_event(old_admin: &Address, new_pending_admin: &Address) {
    EventBuilder::new()
        .string("acceptAdmin")
        .address(old_admin)
        .address(new_pending_admin)
        .notify();
}

pub fn register_token_pair_evt(
    token_pair_name: &str,
    oep_addr: &Address,
    erc_addr: &Address,
    is_oep5: bool,
) {
    EventBuilder::new()
        .string("registerTokenPair")
        .string(token_pair_name)
        .address(oep_addr)
        .address(erc_addr)
        .bool(is_oep5)
        .notify();
}

pub fn transfer_token_pair_owner_evt(old_owner: &Address, new_owner: &Address) {
    EventBuilder::new()
        .string("transferTokenPairOwner")
        .address(old_owner)
        .address(new_owner)
        .notify();
}

pub fn oep5_to_erc721_event(
    ont_acct: &Address,
    eth_acct: &Address,
    token_id: U128,
    oep5_addr: &Address,
    erc721_addr: &Address,
) {
    EventBuilder::new()
        .string("oep5ToOrc721")
        .address(ont_acct)
        .address(eth_acct)
        .number(token_id)
        .address(oep5_addr)
        .address(erc721_addr)
        .notify();
}

pub fn oep8_to_erc1155_event(
    ont_acct: &Address,
    eth_acct: &Address,
    token_id: U128,
    amount: U128,
    oep8_addr: &Address,
    erc1155_addr: &Address,
) {
    EventBuilder::new()
        .string("oep8ToOrc1155")
        .address(ont_acct)
        .address(eth_acct)
        .number(token_id)
        .number(amount)
        .address(oep8_addr)
        .address(erc1155_addr)
        .notify();
}
