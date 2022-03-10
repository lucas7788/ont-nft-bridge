use crate::events::{new_admin_event, new_pending_admin_event};
use common::oep5and8::{ontd_to_ont, call_wasm_contract};
use ostd::contract::ont;
use ostd::database::{delete, get, put};
use ostd::prelude::*;
use ostd::runtime::{check_witness, contract_migrate};

const KEY_ADMIN: &[u8] = b"1";
const KEY_PENDING_ADMIN: &[u8] = b"2";
const KEY_ONTD: &[u8] = b"3";
const KEY_BRIDGE: &[u8] = b"4";
const KEY_TOKEN_PAIR_NAME: &[u8] = b"5";

pub fn initialize(admin: &Address) -> bool {
    assert!(get_admin().is_zero(), "has inited");
    assert!(check_witness(admin), "check admin signature failed");
    put(KEY_ADMIN, admin);
    true
}

pub fn set_ontd(bridge: &Address) {
    check_admin();
    put(KEY_ONTD, bridge);
}

pub fn get_ontd() -> Address {
    get(KEY_ONTD).unwrap_or_default()
}

pub fn set_token_pair_name(token_pair_name: &[u8]) {
    check_admin();
    put(KEY_TOKEN_PAIR_NAME, token_pair_name);
}

pub fn get_token_pair_name() -> Vec<u8> {
    get(KEY_TOKEN_PAIR_NAME).unwrap_or_default()
}


pub fn set_bridge(wont: &Address) {
    check_admin();
    put(KEY_BRIDGE, wont);
}

pub fn get_bridge() -> Address {
    get(KEY_BRIDGE).unwrap_or_default()
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
    let new_addr = contract_migrate(code, vm_type, name, version, author, email, desc);
    assert!(!new_addr.is_zero(), "migrate failed");
    true
}

pub fn ontd_to_wont(from: &Address, eth_acct: &Address, amount: U128) -> bool {
    check_sig(from);
    let ontd = &get_ontd();
    //第一步  ontd -> ONT
    let from_bal_before = ont::v2::balance_of(from);
    ontd_to_ont(ontd, from, amount);
    let from_bal_after = ont::v2::balance_of(from);
    let delta = from_bal_after - from_bal_before;
    if delta.is_zero() {
        return false;
    }
    //第二步 ONT 打给该合约地址
    let bridge = &get_bridge();
    assert!(!bridge.is_zero(), "bridge is zero");
    let res: bool = call_wasm_contract(bridge, ("oep4ToOrc20", from, eth_acct, delta, get_token_pair_name()));
    assert!(res, "oep4ToOrc20 failed");
    true
}

fn check_admin() {
    assert!(check_witness(&get_admin()), "check admin signature failed");
}

fn check_sig(signer: &Address) {
    assert!(check_witness(signer), "invalid signature");
}
