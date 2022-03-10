#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]

extern crate ontio_std as ostd;

use ostd::abi::{Sink, Source};
use ostd::prelude::*;
use ostd::runtime::{input, ret};
use tool::*;

extern crate alloc;
extern crate common;

mod events;
mod tool;

#[no_mangle]
pub fn invoke() {
    let input = input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"init" => {
            let admin = source.read().unwrap();
            sink.write(initialize(admin))
        }
        b"getAdmin" => {
            sink.write(get_admin());
        }
        b"setPendingAdmin" => {
            let new_admin = source.read().unwrap();
            sink.write(set_pending_admin(new_admin));
        }
        b"getPendingAdmin" => {
            sink.write(get_pending_admin());
        }
        b"acceptAdmin" => {
            sink.write(accept_admin());
        }
        b"migrate" => {
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
        b"setBridge" => {
            let bridge = source.read().unwrap();
            sink.write(set_bridge(bridge));
        }
        b"getBridge" => {
            sink.write(get_bridge());
        }
        b"setTokenPairName" => {
            let name = source.read().unwrap();
            sink.write(set_token_pair_name(name));
        }
        b"getTokenPairName" => {
            sink.write(get_token_pair_name());
        }
        b"setOntd" => {
            let wont = source.read().unwrap();
            sink.write(set_ontd(wont));
        }
        b"getOntd" => {
            sink.write(get_ontd());
        }
        b"oep4ToOrc20" => {
            let (ont_acct, eth_acct, amount) = source.read().unwrap();
            sink.write(ontd_to_wont(ont_acct, eth_acct, amount));
        }
        _ => panic!("unsupported action2!"),
    }
    ret(sink.bytes())
}
