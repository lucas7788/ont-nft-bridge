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
        "ONTdToWONT" => {
            let (contract, to, token_id) = source.read().unwrap();
            sink.write(ontd_to_wont(contract, to, token_id));
        }
        _ => panic!("unsupported action!"),
    }
    ret(sink.bytes())
}
