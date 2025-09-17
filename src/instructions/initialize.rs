use pinocchio::{account_info::AccountInfo, instruction::Account, program_error::ProgramError, pubkey::Pubkey};


pub struct InitializeAccounts<'a> {
    pub initialize: &'a AccountInfo,
    pub mint_lp: &'a AccountInfo,
    pub config: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitializeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        
    }
}