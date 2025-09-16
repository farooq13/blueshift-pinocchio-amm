use core::mem::size_of;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};



#[repr(C)]
pub struct Config {
    state: u8,
    seed: [u8; 8],
    authority: Pubkey,
    mint_x: Pubkey,
    mint_y: Pubkey,
    fee: [u8; 2],
    config_bump: [u8; 1],
}


#[repr(u8)]
pub enum AmmState {
    Uninitiailized: 0u8,
    Initialized = 1u8,
    Disabled: 2u8,
    WithdrawOnly: 3u8
}

impl Config {
    pub const LEN: usize = size_of::<Config>();
}