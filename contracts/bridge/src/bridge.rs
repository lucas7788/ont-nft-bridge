use crate::erc721and1155::*;
use crate::events::*;
use alloc::collections::BTreeMap;
use common::oep5and8::{balance_of_oep5, balance_of_oep8, lock_oep5, transfer_oep8, owner_of};
use ostd::abi::{Decoder, Encoder, Sink};
use ostd::database::{delete, get, put};
use ostd::prelude::*;
use ostd::runtime::{address, check_witness, contract_migrate};

const KEY_ADMIN: &[u8] = b"1";
const KEY_PENDING_ADMIN: &[u8] = b"2";
const PREFIX_OEP5_ERC721_PAIR: &[u8] = b"3";
const PREFIX_OEP8_ERC1155_PAIR: &[u8] = b"4";
const KEY_TOKEN_PAIR_NAME: &[u8] = b"5";
const KEY_RECEIVERS: &[u8] = b"6";
const PREFIX_OEP8_IDS: &[u8] = b"7";

#[derive(Encoder, Decoder, Default)]
pub struct TokenPair {
    //must be ontology address
    owner: Address,
    erc: Address,
    oep: Address,
    is_oep5_neovm: bool,
}

pub fn initialize(admin: &Address) -> bool {
    assert!(get_admin().is_zero(), "has inited");
    assert!(check_witness(admin), "check admin signature failed");
    put(KEY_ADMIN, admin);
    true
}

pub fn get_admin() -> Address {
    get(KEY_ADMIN).unwrap_or_default()
}

pub fn set_pending_admin(new_admin: &Address) -> bool {
    assert!(!new_admin.is_zero(), "new admin is zero address");
    assert!(check_witness(&get_admin()), "check admin signature failed");
    put(KEY_PENDING_ADMIN, new_admin);
    new_pending_admin_event(new_admin);
    true
}

pub fn get_pending_admin() -> Address {
    get(KEY_PENDING_ADMIN).unwrap_or_default()
}

pub fn accept_admin() -> bool {
    let pending_admin = get_pending_admin();
    assert!(
        check_witness(&get_pending_admin()),
        "check pending admin signature failed"
    );
    let old_admin = get_admin();
    put(KEY_ADMIN, pending_admin);
    delete(KEY_PENDING_ADMIN);
    new_admin_event(&old_admin, &pending_admin);
    true
}

pub fn migrate(
    code: &[u8],
    vm_type: u32,
    name: &str,
    version: &str,
    author: &str,
    email: &str,
    desc: &str,
) -> bool {
    check_admin();
    let this = &address();
    let all_token_pair_name = get_all_token_pair_name();
    let mut oep8_id_map: BTreeMap<String, (TokenPair, Vec<U128>)> = BTreeMap::new();
    for name in all_token_pair_name.iter() {
        let pair = get_token_pair(name.as_bytes());
        let oep8_ids = get_oep8_ids(&pair.oep);
        if !oep8_ids.is_empty() {
            oep8_id_map.insert(name.clone(), (pair, oep8_ids));
        }
    }

    let new_addr = contract_migrate(code, vm_type, name, version, author, email, desc);
    assert!(!new_addr.is_zero(), "migrate failed");
    for (_, (pair, ids)) in oep8_id_map.iter() {
        ids.iter().for_each(|&id| {
            let oep8_balance = balance_of_oep8(&pair.oep, this, id);
            if !oep8_balance.is_zero() {
                transfer_oep8(&pair.oep, this, &new_addr, id, oep8_balance);
            }
        });
    }
    true
}

pub fn get_all_token_pair_name() -> Vec<String> {
    get(KEY_TOKEN_PAIR_NAME).unwrap_or_default()
}

pub fn register_oep5_erc721_pair(
    token_pair_name: &str,
    oep5_addr: &Address,
    erc721_addr: &Address,
    is_neovm: bool,
) -> bool {
    register_token_pair(token_pair_name, oep5_addr, erc721_addr, true, is_neovm)
}

pub fn register_oep8_erc1155_pair(
    token_pair_name: &str,
    oep8_addr: &Address,
    erc1155_addr: &Address,
) -> bool {
    register_token_pair(token_pair_name, oep8_addr, erc1155_addr, false, false)
}

fn register_token_pair(
    token_pair_name: &str,
    oep_addr: &Address,
    erc_addr: &Address,
    is_oep5: bool,
    is_neovm: bool,
) -> bool {
    let admin = get_admin();
    assert!(check_witness(&admin), "need admin signature");
    assert!(!oep_addr.is_zero(), "invalid oep address");
    assert!(!erc_addr.is_zero(), "invalid erc address");

    let pair_key = if is_oep5 {
        gen_key(PREFIX_OEP5_ERC721_PAIR, token_pair_name)
    } else {
        gen_key(PREFIX_OEP8_ERC1155_PAIR, token_pair_name)
    };
    let token_pair: Option<TokenPair> = get(pair_key.as_slice());
    assert!(token_pair.is_none(), "token pair name has registered");

    let mut names = get_all_token_pair_name();
    names.push(token_pair_name.to_string());
    put(KEY_TOKEN_PAIR_NAME, names);

    put(
        pair_key.as_slice(),
        TokenPair {
            owner: admin,
            erc: *erc_addr,
            oep: *oep_addr,
            is_oep5_neovm: is_neovm,
        },
    );
    register_token_pair_evt(token_pair_name, oep_addr, erc_addr, is_oep5);
    true
}

//new_owner can be zero address, it means close update function
pub fn transfer_token_pair_owner(token_pair_name: &[u8], new_owner: &Address) -> bool {
    let (pair, pair_key) = get_token_pair_by_name(token_pair_name);
    let mut pair: TokenPair = pair.expect("token pair name has not registered");
    let old = pair.owner.clone();
    assert!(
        check_witness(&get_admin()) || check_witness(&old),
        "need admin or owner signature"
    );
    pair.owner = *new_owner;
    put(pair_key, pair);
    transfer_token_pair_owner_evt(&old, new_owner);
    true
}

fn get_token_pair_by_name(token_name: &[u8]) -> (Option<TokenPair>, Vec<u8>) {
    let key = gen_token_pair_key_oep5(token_name);
    let pair: Option<TokenPair> = get(key.as_slice());
    if pair.is_none() {
        let key = gen_token_pair_key_oep8(token_name);
        (get(key.as_slice()), key)
    } else {
        (pair, key)
    }
}

pub fn get_oep5_neovm_receivers() -> Vec<Address> {
    get(KEY_RECEIVERS).unwrap_or_default()
}

fn gen_token_pair_key_oep5(token_name: &[u8]) -> Vec<u8> {
    gen_key(PREFIX_OEP5_ERC721_PAIR, token_name)
}

fn gen_token_pair_key_oep8(token_name: &[u8]) -> Vec<u8> {
    gen_key(PREFIX_OEP8_ERC1155_PAIR, token_name)
}

pub fn get_token_pair(token_name: &[u8]) -> TokenPair {
    let (pair, _) = get_token_pair_by_name(token_name);
    pair.expect("non-exist token pair")
}

pub fn add_oep5_neovm_receiver(receivers: &[Address]) {
    check_admin();
    let mut addrs = get_oep5_neovm_receivers();
    if addrs.is_empty() {
        put(KEY_RECEIVERS, receivers);
    } else {
        for item in receivers.iter() {
            if !addrs.contains(item) {
                addrs.push(*item);
            }
        }
        put(KEY_RECEIVERS, addrs);
    }
}

pub fn del_oep5_neovm_receiver(receiver: &Address) {
    check_admin();
    let mut addrs = get_oep5_neovm_receivers();
    let index = addrs
        .iter()
        .position(|x| x == receiver)
        .expect("non-exist ");
    addrs.remove(index);
    put(KEY_RECEIVERS, addrs);
}

pub fn oep5_to_erc721(
    ont_acct: &Address,
    eth_acct: &Address,
    token_id: U128,
    token_pair_name: &[u8],
) -> bool {
    assert!(check_witness(ont_acct));
    let key = gen_token_pair_key_oep5(token_pair_name);
    let pair: TokenPair = get(key.as_slice()).expect("non-existed token pair name");
    let this = &address();
    let (receiver, before) = find_receiver_addr(&pair.oep, pair.is_oep5_neovm);
    let owner = owner_of(&pair.oep, token_id, pair.is_oep5_neovm);
    assert_eq!(ont_acct, &owner, "invalid owner");
    lock_oep5(&receiver, &pair.oep, token_id, pair.is_oep5_neovm);
    let after = balance_of_oep5(&pair.oep, &receiver, pair.is_oep5_neovm);
    let delta = after - before;
    if !delta.is_zero() {
        let before = balance_of_erc721(this, &pair.erc, eth_acct);
        mint_erc721(this, &pair.erc, eth_acct, token_id);
        let after = balance_of_erc721(this, &pair.erc, eth_acct);
        assert_eq!(after - before, U128::new(1), "mint failed");
        oep5_to_erc721_event(ont_acct, eth_acct, token_id, &pair.oep, &pair.erc);
    }
    true
}

fn find_receiver_addr(oep5: &Address, oep5_is_neovm: bool) -> (Address, U128) {
    let receivers = get_oep5_neovm_receivers();
    for item in receivers.iter() {
        let before = balance_of_oep5(oep5, item, oep5_is_neovm);
        if before < U128::new(1000) {
            return (*item, before);
        }
    }
    panic!("non-exist address")
}

pub fn oep8_to_erc1155(
    ont_acct: &Address,
    eth_acct: &Address,
    token_id: U128,
    token_pair_name: &[u8],
    amount: U128,
) -> bool {
    assert!(check_witness(ont_acct));
    assert!(!amount.is_zero(), "amount should be more than 0");
    let key = gen_token_pair_key_oep8(token_pair_name);
    let pair: TokenPair = get(key.as_slice()).expect("non-existed token pair name");
    let this = &address();
    let before = balance_of_oep8(&pair.oep, this, token_id);
    transfer_oep8(&pair.oep, ont_acct, this, token_id, amount);
    let after = balance_of_oep8(&pair.oep, this, token_id);
    push_oep8_id(token_id, &pair.oep);
    let delta = after - before;
    if !delta.is_zero() {
        let before = balance_of_erc1155(this, &pair.erc, eth_acct, token_id);
        mint_erc1155(this, &pair.erc, eth_acct, token_id, amount);
        let after = balance_of_erc1155(this, &pair.erc, eth_acct, token_id);
        assert_eq!(after - before, amount, "mint failed");
    }
    oep8_to_erc1155_event(ont_acct, eth_acct, token_id, amount, &pair.oep, &pair.erc);
    true
}

fn push_oep8_id(id: U128, oep8: &Address) {
    let mut ids = get_oep8_ids(oep8);
    if !ids.contains(&id) {
        ids.push(id);
        put(gen_key(PREFIX_OEP8_IDS, oep8), ids);
    }
}

fn get_oep8_ids(oep8: &Address) -> Vec<U128> {
    let key = gen_key(PREFIX_OEP8_IDS, oep8);
    get(key.as_slice()).unwrap_or_default()
}

fn check_admin() {
    assert!(check_witness(&get_admin()), "check admin signature failed");
}

fn gen_key<T: Encoder>(prefix: &[u8], post: T) -> Vec<u8> {
    let mut sink = Sink::new(64);
    sink.write(prefix);
    sink.write(post);
    sink.bytes().to_vec()
}
