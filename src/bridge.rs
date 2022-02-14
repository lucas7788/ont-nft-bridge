use crate::erc721and1155::*;
use crate::events::*;
use crate::oep5and8::*;
use ontio_std::abi::{Decoder, Encoder, Sink};
use ontio_std::database::{delete, get, put};
use ontio_std::prelude::*;
use ontio_std::runtime::{address, check_witness};

const KEY_ADMIN: &[u8] = b"1";
const KEY_PENDING_ADMIN: &[u8] = b"2";
const PREFIX_OEP5_ERC721_PAIR: &[u8] = b"3";
const PREFIX_OEP8_ERC1155_PAIR: &[u8] = b"4";
const KEY_TOKEN_PAIR_NAME: &[u8] = b"5";

#[derive(Encoder, Decoder, Default)]
pub struct TokenPair {
    //must be ontology address
    owner: Address,
    erc: Address,
    oep: Address,
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

pub fn get_all_token_pair_name() -> Vec<String> {
    get(KEY_TOKEN_PAIR_NAME).unwrap_or_default()
}

pub fn register_oep5_erc721_pair(
    token_pair_name: &str,
    oep5_addr: &Address,
    erc721_addr: &Address,
) -> bool {
    register_token_pair(token_pair_name, oep5_addr, erc721_addr, true)
}

pub fn register_oep8_erc1155_pair(
    token_pair_name: &str,
    oep8_addr: &Address,
    erc1155_addr: &Address,
) -> bool {
    register_token_pair(token_pair_name, oep8_addr, erc1155_addr, false)
}

fn register_token_pair(
    token_pair_name: &str,
    oep_addr: &Address,
    erc_addr: &Address,
    is_oep5: bool,
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

pub fn oep5_to_erc721(
    ont_acct: &Address,
    eth_acct: &Address,
    token_id: U128,
    token_pair_name: &[u8],
) -> bool {
    assert!(check_witness(ont_acct));
    let (pair, _) = get_token_pair_by_name(token_pair_name);
    let pair: TokenPair = pair.expect("amount should be more than 0");
    let this = &address();
    let before = balance_of_oep5(&pair.oep, this);
    transfer_oep5(&pair.oep, ont_acct, this, token_id);
    let after = balance_of_oep5(&pair.oep, this);
    let delta = after - before;
    if !delta.is_zero() {
        mint_erc721(this, &pair.erc, eth_acct, token_id);
    }
    oep5_to_erc721_event(ont_acct, eth_acct, token_id, &pair.oep, &pair.erc);
    true
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
    let (pair, _) = get_token_pair_by_name(token_pair_name);
    let pair: TokenPair = pair.expect("amount should be more than 0");
    let this = &address();
    let before = balance_of_oep8(&pair.oep, this, token_id);
    transfer_oep8(&pair.oep, ont_acct, this, token_id, amount);
    let after = balance_of_oep8(&pair.oep, this, token_id);
    let delta = after - before;
    if !delta.is_zero() {
        mint_erc1155(this, &pair.erc, eth_acct, token_id, amount);
    }
    oep8_to_erc1155_event(ont_acct, eth_acct, token_id, amount, &pair.oep, &pair.erc);
    true
}

fn gen_key<T: Encoder>(prefix: &[u8], post: T) -> Vec<u8> {
    let mut sink = Sink::new(64);
    sink.write(prefix);
    sink.write(post);
    sink.bytes().to_vec()
}
