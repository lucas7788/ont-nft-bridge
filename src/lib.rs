#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use crate::bridge::*;
use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime::{input, ret};

mod bridge;
mod erc721and1155;
mod events;
mod oep5and8;

#[no_mangle]
pub fn invoke() {
    let input = input();
    let mut source = Source::new(&input);
    let action = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        "init" => {
            let admin = source.read().unwrap();
            sink.write(initialize(admin))
        }
        "getAdmin" => {
            sink.write(get_admin());
        }
        "setPendingAdmin" => {
            let new_admin = source.read().unwrap();
            sink.write(set_pending_admin(new_admin));
        }
        "getPendingAdmin" => {
            sink.write(get_pending_admin());
        }
        "acceptAdmin" => {
            sink.write(accept_admin());
        }
        "registerOep5Erc721Pair" => {
            let (token_pair_name, oep5_addr, erc721_addr) = source.read().unwrap();
            sink.write(register_oep5_erc721_pair(
                token_pair_name,
                oep5_addr,
                erc721_addr,
            ))
        }
        "registerOep8Erc1155Pair" => {
            let (token_pair_name, oep8_addr, erc1155_addr) = source.read().unwrap();
            sink.write(register_oep8_erc1155_pair(
                token_pair_name,
                oep8_addr,
                erc1155_addr,
            ))
        }
        "transferTokenPairOwner" => {
            let (token_pair_name, new_owner) = source.read().unwrap();
            sink.write(transfer_token_pair_owner(token_pair_name, new_owner))
        }
        "getAllTokenPairName" => {
            sink.write(get_all_token_pair_name());
        }
        "getTokenPair" => {
            let token_pair_name = source.read().unwrap();
            sink.write(get_token_pair(token_pair_name));
        }
        "oep5ToErc721" => {
            let (ont_acct, eth_acct, token_id, token_pair_name) = source.read().unwrap();
            sink.write(oep5_to_erc721(
                ont_acct,
                eth_acct,
                token_id,
                token_pair_name,
            ));
        }
        "oep8ToErc1155" => {
            let (ont_acct, eth_acct, token_id, amount, token_pair_name) = source.read().unwrap();
            sink.write(oep8_to_erc1155(
                ont_acct,
                eth_acct,
                token_id,
                token_pair_name,
                amount,
            ));
        }
        _ => panic!("unsupported action!"),
    }

    ret(sink.bytes())
}
