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

    #[inline(always)]
    pub fn load(account_info: &AccountInfo) -> Result<Ref<Self>, ProgramError> {
        if account_info.data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(Ref::map(account_info.try_borrow_data()?, |data| unsafe {
            Self::from_bytes_unchecked(data)
        }))
    }

    #[inline(always)]
    pub unsafe fn load_unchecked(account_info: &AccountInfo) -> Result<&Self, ProgramError> {
        if account_info.data.len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(Self::from_bytes_unchecked(
            account_info.borrow_data_unchecked(),
        ))
    }

    /// Return a `Config` from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `Config`, and
    /// it is properly aligned to be interpreted as an instance of `Config`.
    /// At the moment `Config` has an alignment of 1 byte.
    /// This method does not perform a length validation.
    #[inline(always)]
    pub unsafe  fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        &*(bytes.as_ptr() as *const Config)
    }

    /// Return a mutable `Config` reference from the given bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `bytes` contains a valid representation of `Config`.
    #[inline(always)]
    pub unsafe  fn from_bytes_unchecked_mut(bytes: &mut [u8]) -> &mut Self {
        &mut *(bytes.as_mut_ptr() as *mut Config)
    }

    // Getter methods for safe field access
    #[inline(always)]
    pub fn state(&self) -> u8 { self.state }

    #[inline(always)]
    pub fn seed(&self) -> u64 { u64::from_le_bytes(self.seed) }

    #[inline(always)]
    pub fn authority(&self) -> &Pubkey { &self.authority }

    #[inline(always)]
    pub fn mint_x(&self) -> &Pubkey { &self.mint_x }

    #[inline(always)]
    pub fn mint_y(&self) -> &Pubkey { &self.mint_y }

    #[inline(always)]
    pub fn fee(&self) -> u16 { u16::from_le_bytes(self.fee) }

    #[inline(always)]
    pub fn config_bump(&self) -> [u8; 1] { self.config_bump }
    

}