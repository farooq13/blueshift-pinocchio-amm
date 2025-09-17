use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};


pub struct DepositAccounts<'a> {
    pub user: &'a AccountInfo,
    pub mint_lp: &'a AccountInfo,
    pub vault_x: &'a AccountInfo,
    pub vault_y: &'a AccountInfo,
    pub user_x_ata: &'a AccountInfo,
    pub user_y_ata: &'a AccountInfo,
    pub user_lp_ata: &'a AccountInfo,
    pub config: &'a AccountInfo,
    pub token_program: &'a AccountInfo,
}
 
impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
  type Error = ProgramError;
 
  fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
    //..
  }
}