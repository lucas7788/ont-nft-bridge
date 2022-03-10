use ontio_std::abi::Source;
use ontio_std::contract::eth;
use ontio_std::prelude::*;
use ontio_std::types::U256;

const MINT_ID_WONT: [u8; 4] = [0x40, 0xc1, 0x0f, 0x19];
pub fn mint_wont(caller: &Address, target: &Address, to: &Address, amount: U128) {
    eth::evm_invoke(caller, target, gen_wont_mint_data(to, amount).as_slice());
}

fn gen_wont_mint_data(to_acct: &Address, amount: U128) -> Vec<u8> {
    [
        MINT_ID_WONT.as_ref(),
        format_addr(to_acct).as_ref(),
        format_amount(amount).as_ref(),
    ]
    .concat()
}

pub fn balance_of_erc721(caller: &Address, target: &Address, user: &Address) -> U128 {
    let res = eth::evm_invoke(caller, target, gen_erc721_balance_of_data(user).as_slice());
    if res.is_empty() {
        return U128::new(0);
    }
    let mut source = Source::new(res.as_slice());
    let h = source.read_h256().unwrap();
    let res = U256::from_big_endian(h.as_bytes());
    res.as_u128()
}

pub fn mint_erc721(caller: &Address, target: &Address, to: &Address, token_id: U128) {
    eth::evm_invoke(
        caller,
        target,
        gen_erc721_mint_data(to, token_id).as_slice(),
    );
}

pub fn mint_erc1155(
    caller: &Address,
    target: &Address,
    to: &Address,
    token_id: U128,
    amount: U128,
) {
    eth::evm_invoke(
        caller,
        target,
        gen_erc1155_mint_data(to, token_id, amount).as_slice(),
    );
}

// const TRANSFER_ID: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];
// const TRANSFER_FROM_ID: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];
const MINT_ID_ERC721: [u8; 4] = [0x40, 0xc1, 0x0f, 0x19];
const BALANCEOF_ID_ERC721: [u8; 4] = [0x70, 0xa0, 0x82, 0x31];
const BALANCEOF_ID_ERC1155: [u8; 4] = [0x00, 0xfd, 0xd5, 0x8e];
const MINT_ID_ERC1155: [u8; 4] = [0x15, 0x6e, 0x29, 0xf6];

fn gen_erc721_mint_data(to_acct: &Address, token_id: U128) -> Vec<u8> {
    [
        MINT_ID_ERC721.as_ref(),
        format_addr(to_acct).as_ref(),
        format_amount(token_id).as_ref(),
    ]
    .concat()
}

fn gen_erc1155_mint_data(to_acct: &Address, token_id: U128, amount: U128) -> Vec<u8> {
    [
        MINT_ID_ERC1155.as_ref(),
        format_addr(to_acct).as_ref(),
        format_amount(token_id).as_ref(),
        format_amount(amount).as_ref(),
    ]
    .concat()
}

fn gen_erc721_balance_of_data(addr: &Address) -> Vec<u8> {
    [BALANCEOF_ID_ERC721.as_ref(), format_addr(addr).as_ref()].concat()
}

pub fn format_addr(addr: &Address) -> [u8; 32] {
    let mut res = [0; 32];
    res[12..].copy_from_slice(addr.as_bytes());
    res
}

pub fn format_amount(amt: U128) -> [u8; 32] {
    U256::from(amt).to_be_bytes()
}

// pub const SAFE_TRANSFER_FROM_ID: [u8; 4] = [0xf2, 0x42, 0x43, 0x2a];

pub fn balance_of_erc1155(
    caller: &Address,
    target: &Address,
    user: &Address,
    token_id: U128,
) -> U128 {
    let res = eth::evm_invoke(
        caller,
        target,
        gen_erc1155_balance_of_data(user, token_id).as_slice(),
    );
    if res.is_empty() {
        return U128::new(0);
    }
    let mut source = Source::new(res.as_slice());
    let h = source.read_h256().unwrap();
    let res = U256::from_big_endian(h.as_bytes());
    res.as_u128()
}

fn gen_erc1155_balance_of_data(user: &Address, token_id: U128) -> Vec<u8> {
    [
        BALANCEOF_ID_ERC1155.as_ref(),
        format_addr(user).as_ref(),
        format_amount(token_id).as_ref(),
    ]
    .concat()
}

#[test]
fn test() {
    let addr = &Address::repeat_byte(1);
    println!("{:?}", format_addr(&addr).as_ref());
}
