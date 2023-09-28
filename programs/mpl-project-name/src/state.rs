use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankAccount;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::MplProjectNameError;

#[repr(u64)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Key {
    #[default]
    Uninitialized,
    MyAccount,
}

// Can't derive Pod/Zeroable for enums, so we have to do it ourselves manually.
// Our enum is aligned to 8 bytes, so we can safely use the unsafe impls.
unsafe impl Zeroable for Key {}
unsafe impl Pod for Key {}

// All fields in this are aligned to 8 bytes, and we use repr(C)
// so we can use the Pod and Zeroable macros.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct MyAccount {
    pub key: Key,
    pub authority: Pubkey,
    pub data: MyData,
}

impl MyAccount {
    pub const LEN: usize = std::mem::size_of::<MyAccount>();

    // This is the first seed for MyAccount, assuming it's a PDA.
    pub const PREFIX: &'static [u8] = b"my_account";

    pub fn from_bytes(bytes: &'_ mut [u8]) -> &'_ mut Self {
        bytemuck::from_bytes_mut::<MyAccount>(bytes)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct MyData {
    pub alice: u64,
    pub bob: [u8; 32],
}

impl MyData {
    pub const LEN: usize = std::mem::size_of::<MyData>();

    pub fn from_bytes(bytes: &'_ mut [u8]) -> &'_ mut Self {
        bytemuck::from_bytes_mut::<MyData>(bytes)
    }
}
