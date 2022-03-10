#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime::{input, ret};
use receive::*;

extern crate alloc;
extern crate common;

mod events;
mod receive;

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
        "getNftBridge" => {
            sink.write(get_nft_bridge());
        }
        "setNftBridge" => {
            let bridge = source.read().unwrap();
            sink.write(set_nft_bridge(bridge));
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
        "withdrawOep5" => {
            let (contract, to, token_id, oep5_is_neovm) = source.read().unwrap();
            sink.write(withdraw_oep5(contract, to, token_id, oep5_is_neovm));
        }
        "lockOep5" => {
            let (contract, token_id, oep5_is_neovm) = source.read().unwrap();
            sink.write(lock_oep5(contract, token_id, oep5_is_neovm));
        }
        "getOep5Ids" => {
            let contract = source.read().unwrap();
            sink.write(get_oep5_ids(contract));
        }
        _ => panic!("unsupported action2!"),
    }
    ret(sink.bytes())
}
