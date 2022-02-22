#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use crate::bridge::*;
use common::oep5and8::balance_of_oep5;
use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime::{input, ret, address};
use erc721and1155::{balance_of_erc721, mint_erc721, balance_of_erc1155, mint_erc1155};

extern crate alloc;
extern crate common;

mod bridge;
mod erc721and1155;
mod events;

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
        "migrate" => {
            let (code, vm_type, name, version, author, email, desc) = source.read().unwrap();
            let vm_type: U128 = vm_type;
            sink.write(migrate(
                code,
                vm_type.raw() as u32,
                name,
                version,
                author,
                email,
                desc,
            ));
        }
        "addOep5NeovmReceiver" => {
            let receivers: Vec<Address> = source.read().unwrap();
            sink.write(add_oep5_neovm_receiver(receivers.as_slice()))
        }
        "delOep5NeovmReceiver" => {
            let receiver = source.read().unwrap();
            sink.write(del_oep5_neovm_receiver(receiver))
        }
        "getOep5NeovmReceivers" => sink.write(get_oep5_neovm_receivers()),
        "registerOep5Erc721Pair" => {
            let (token_pair_name, oep5_addr, erc721_addr, is_neovm) = source.read().unwrap();
            sink.write(register_oep5_erc721_pair(
                token_pair_name,
                oep5_addr,
                erc721_addr,
                is_neovm,
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
        "mintErc721" => {
            let (erc721, eth_acct, token_id) = source.read().unwrap();
            sink.write(mint_erc721(
                &address(),
                erc721,
                eth_acct,
                token_id,
            ));
        }
        "balanceOfOep5" => {
            let (ont_acct, eth_acct, is_neovm) = source.read().unwrap();
            sink.write(balance_of_oep5(ont_acct, eth_acct, is_neovm));
        }
        "balanceOfErc721" => {
            let (caller, target, user) = source.read().unwrap();
            sink.write(balance_of_erc721(caller, target, user));
        }
        "balanceOfErc1155" => {
            let (caller, target, user, token_id) = source.read().unwrap();
            sink.write(balance_of_erc1155(caller, target, user, token_id));
        }
        "mintErc1155" => {
            let (caller, target, user, token_id, amount) = source.read().unwrap();
            sink.write(mint_erc1155(caller, target, user, token_id, amount));
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
        _ => panic!("unsupported action2!"),
    }

    ret(sink.bytes())
}
