use ontio_std::abi::{Decoder, Encoder, Source, VmValueBuilder, VmValueParser};
use ontio_std::contract::{ong, ont, wasm};
use ontio_std::macros;
use ontio_std::prelude::Vec;
use ontio_std::runtime;
use ontio_std::types::{u128_from_neo_bytes, Address, U128};

pub const ONT_CONTRACT_ADDRESS: Address = macros::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhUMqNMV");
pub const ONG_CONTRACT_ADDRESS: Address = macros::base58!("AFmseVrdL9f9oyCzZefL9tG6UbvhfRZMHJ");

pub fn transfer_oep4(contract: &Address, from: &Address, to: &Address, amount: U128) {
    let mut builder = VmValueBuilder::new();
    builder.string("transfer");
    let mut nested = builder.list();
    nested.address(from);
    nested.address(to);
    nested.number(amount);
    nested.finish();
    assert!(
        call_neovm_bool(contract, builder.bytes().as_slice()),
        "oep4 transfer failed"
    );
}

pub fn ontd_to_ont(contract: &Address, from: &Address, amount: U128) {
    let mut builder = VmValueBuilder::new();
    builder.string("ontd2ont");
    let mut nested = builder.list();
    nested.address(from);
    nested.number(amount);
    nested.finish();
    assert!(
        call_neovm_bool(contract, builder.bytes().as_slice()),
        "ontd2ont failed"
    );
}

pub fn balance_of_oep4(contract: &Address, account: &Address) -> U128 {
    if contract == &ONT_CONTRACT_ADDRESS {
        return ont::balance_of(account);
    }
    if contract == &ONG_CONTRACT_ADDRESS {
        return ong::balance_of(account);
    }
    let mut builder = VmValueBuilder::new();
    builder.string("balanceOf");
    let mut nested = builder.list();
    nested.address(account);
    nested.finish();
    call_neovm_bytearray_num(contract, builder.bytes().as_slice())
}

pub fn balance_of_oep8(contract: &Address, account: &Address, token_id: U128) -> U128 {
    call_wasm_contract(contract, ("balanceOf", account, token_id))
}

pub fn balance_of_oep5(contract: &Address, account: &Address, oep5_is_neovm: bool) -> U128 {
    if contract == &ONT_CONTRACT_ADDRESS {
        return ont::balance_of(account);
    }
    if contract == &ONG_CONTRACT_ADDRESS {
        return ong::balance_of(account);
    }
    if oep5_is_neovm {
        let mut builder = VmValueBuilder::new();
        builder.string("balanceOf");
        let mut nested = builder.list();
        nested.address(account);
        nested.finish();
        call_neovm_bytearray_num(contract, builder.bytes().as_slice())
    } else {
        call_wasm_contract(contract, ("balanceOf", account))
    }
}

pub fn owner_of(contract: &Address, token_id: U128, oep5_is_neovm: bool) -> Address {
    if oep5_is_neovm {
        let mut builder = VmValueBuilder::new();
        builder.string("ownerOf");
        let mut nested = builder.list();
        nested.number(token_id);
        nested.finish();
        call_neovm_address(contract, builder.bytes().as_slice())
    } else {
        call_wasm_contract(contract, ("ownerOf", token_id))
    }
}

pub fn transfer_oep8(
    contract: &Address,
    from: &Address,
    to: &Address,
    token_id: U128,
    amount: U128,
) {
    if contract == &ONT_CONTRACT_ADDRESS {
        assert!(ont::transfer(from, to, amount), "ont transfer failed");
    } else if contract == &ONG_CONTRACT_ADDRESS {
        assert!(ong::transfer(from, to, amount), "ong transfer failed");
    } else {
        let r: bool = call_wasm_contract(contract, ("transfer", from, to, token_id, amount));
        assert!(r, "oep8 transfer failed");
    }
}

pub fn transfer_oep5(contract: &Address, to: &Address, token_id: U128, oep5_is_neovm: bool) {
    if oep5_is_neovm {
        let mut builder = VmValueBuilder::new();
        builder.string("transfer");
        let mut nested = builder.list();
        nested.address(to);
        nested.number(token_id);
        nested.finish();
        assert!(
            call_neovm_bool(contract, builder.bytes().as_slice()),
            "oep5 transfer failed"
        );
    } else {
        let b: bool = call_wasm_contract(contract, ("transfer", to, token_id));
        assert!(b, "oep5 transfer failed oep5");
    }
}

pub fn lock_oep5(contract: &Address, oep5: &Address, token_id: U128, oep5_is_neovm: bool) {
    let b: bool = call_wasm_contract(contract, ("lockOep5", oep5, token_id, oep5_is_neovm));
    assert!(b, "lockOep5 failed oep5");
}

#[track_caller]
pub fn call_neovm_num(address: &Address, param: &[u8]) -> U128 {
    let result = runtime::call_contract(address, param);
    let mut source = VmValueParser::new(result.as_slice());
    source.number().unwrap()
}

#[track_caller]
pub fn call_neovm_bytearray_num(address: &Address, param: &[u8]) -> U128 {
    let result = runtime::call_contract(address, param);
    let mut source = VmValueParser::new(result.as_slice());
    let data = source.bytearray().unwrap();
    u128_from_neo_bytes(data)
}

#[track_caller]
pub fn call_neovm_address(address: &Address, param: &[u8]) -> Address {
    let data = call_neovm_bytearray(address, param);
    Address::from_slice(data.as_slice())
}

#[track_caller]
pub fn call_neovm_bytearray(address: &Address, param: &[u8]) -> Vec<u8> {
    let result = runtime::call_contract(address, param);
    let mut source = VmValueParser::new(result.as_slice());
    source.bytearray().unwrap().to_vec()
}

#[track_caller]
pub fn call_wasm_contract<T: Encoder, R>(address: &Address, param: T) -> R
where
    for<'a> R: Decoder<'a>,
{
    let result = wasm::call_contract(address, param);
    let mut source = Source::new(result.as_slice());
    return source.read().unwrap();
}

#[track_caller]
pub fn call_neovm_bool(address: &Address, param: &[u8]) -> bool {
    let result = runtime::call_contract(address, param);
    let mut source = VmValueParser::new(result.as_slice());
    source.bool().unwrap()
}
