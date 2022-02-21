use crate::events::{new_admin_event, new_pending_admin_event};
use alloc::collections::BTreeMap;
use common::oep5and8::{owner_of, transfer_oep5};
use ostd::abi::Sink;
use ostd::abi::{Decoder, Encoder};
use ostd::database::{delete, get, put};
use ostd::prelude::*;
use ostd::runtime::{address, check_witness, contract_migrate};

const KEY_ADMIN: &[u8] = b"1";
const KEY_PENDING_ADMIN: &[u8] = b"2";
const PREFIX_OEP5_IDS: &[u8] = b"3";
const KEY_OEP5_CONTRACTS: &[u8] = b"4";
const KEY_NFT_BRIDGE: &[u8] = b"5";

pub fn initialize(admin: &Address) -> bool {
    assert!(get_admin().is_zero(), "has inited");
    assert!(check_witness(admin), "check admin signature failed");
    put(KEY_ADMIN, admin);
    true
}

pub fn set_nft_bridge(bridge: &Address) {
    check_admin();
    put(KEY_NFT_BRIDGE, bridge);
}

pub fn get_nft_bridge() -> Address {
    get(KEY_NFT_BRIDGE).unwrap_or_default()
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
    let oep5_con = get_oep5_contracts();
    let mut oep5_id_map: BTreeMap<Address, (Vec<U128>, bool)> = BTreeMap::new();
    for con in oep5_con.iter() {
        let oep5_ids = get_oep5_ids(&con.addr);
        if !oep5_ids.is_empty() {
            oep5_id_map.insert(con.addr, (oep5_ids, con.oep5_is_neovm));
        }
    }

    let new_addr = contract_migrate(code, vm_type, name, version, author, email, desc);
    assert!(!new_addr.is_zero(), "migrate failed");
    for (contract, (oep5_ids, oep5_is_neovm)) in oep5_id_map.iter() {
        oep5_ids.iter().for_each(|&id| {
            let owner = owner_of(&contract, id, *oep5_is_neovm);
            if owner == address() {
                transfer_oep5(&contract, &new_addr, id, *oep5_is_neovm);
            }
        });
    }
    true
}

pub fn withdraw_oep5(contract: &Address, to: &Address, token_id: U128, oep5_is_neovm: bool) {
    let bridge = get_nft_bridge();
    let admin = get_admin();
    assert!(
        check_witness(&bridge) || check_witness(&admin),
        "only admin or bridge"
    );
    let this = address();
    let owner = owner_of(contract, token_id, oep5_is_neovm);
    assert_eq!(owner, this, "invalid owner");
    transfer_oep5(contract, to, token_id, oep5_is_neovm);
    let owner = owner_of(contract, token_id, oep5_is_neovm);
    assert_ne!(owner, this, "transfer oep5 failed");
    del_oep5_id(token_id, contract);
}

pub fn lock_oep5(contract: &Address, token_id: U128, oep5_is_neovm: bool) {
    let bridge = get_nft_bridge();
    let admin = get_admin();
    assert!(
        check_witness(&bridge) || check_witness(&admin),
        "only admin or bridge"
    );
    let this = address();
    transfer_oep5(contract, &this, token_id, oep5_is_neovm);
    push_oep5_id(token_id, contract);
    push_oep5_contract(contract, oep5_is_neovm);
}

fn push_oep5_id(id: U128, oep5: &Address) {
    let mut ids: Vec<U128> = get_oep5_ids(oep5);
    if !ids.contains(&id) {
        ids.push(id);
        put(gen_key(PREFIX_OEP5_IDS, oep5), ids);
    }
}

fn del_oep5_id(id: U128, oep5: &Address) {
    let mut ids: Vec<U128> = get_oep5_ids(oep5);
    if let Some(index) = ids.iter().position(|&x| x == id) {
        ids.remove(index);
        put(gen_key(PREFIX_OEP5_IDS, oep5), ids);
    }
}

pub fn get_oep5_ids(oep5: &Address) -> Vec<U128> {
    let key = gen_key(PREFIX_OEP5_IDS, oep5);
    get(key.as_slice()).unwrap_or_default()
}

#[derive(Encoder, Decoder, Default, Clone)]
pub struct Oep5Address {
    addr: Address,
    oep5_is_neovm: bool,
}

pub fn get_oep5_contracts() -> Vec<Oep5Address> {
    get(KEY_OEP5_CONTRACTS).unwrap_or_default()
}

pub fn push_oep5_contract(contract: &Address, oep5_is_neovm: bool) {
    let mut s = get_oep5_contracts();
    let index = s.iter().position(|x| &x.addr == contract);
    if index.is_none() {
        s.push(Oep5Address {
            addr: *contract,
            oep5_is_neovm,
        });
        put(KEY_OEP5_CONTRACTS, s);
    }
}

fn gen_key<T: Encoder>(prefix: &[u8], post: T) -> Vec<u8> {
    let mut sink = Sink::new(64);
    sink.write(prefix);
    sink.write(post);
    sink.bytes().to_vec()
}

fn check_admin() {
    assert!(check_witness(&get_admin()), "check admin signature failed");
}
