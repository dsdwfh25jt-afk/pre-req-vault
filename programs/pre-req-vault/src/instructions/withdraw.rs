use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
//  here we declared the program which are we going to use for registration 
declare_program!(registration);

    // here we are using the necessary accounts and functions to call the 
    // registation smart contract via CPI 
    // which has : 1) Initialize account (struct) which is goint to store the data in itself
    // and 2) the initialize function which is going perform the action of storing data 
    // in that struct or account 
    use registration::cpi::{accounts::Initialize, initialize};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    mut,
    seeds = [b"vault", vault_state.key().as_ref()],
    bump = vault_state.vault_bump,
  )]
    pub vault: SystemAccount<'info>,

    #[account(
    seeds = [b"state", user.key().as_ref()],
    bump = vault_state.state_bump
  )]
    pub vault_state: Account<'info, VaultState>,

    /// CHECK: application account will be initialized by the cpi call to the application program
    #[account(
    mut,
    seeds = [b"prereqs", user.key().as_ref()],
    seeds::program = application_program.key(),
    bump
    )]
    pub application_account: UncheckedAccount<'info>,

    pub application_program: Program<'info, registration::program::Q3PreReqsRs>,

    system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(System::id(), cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        // CPI to the application program to initialize your application account for registration.
        // All the neccessary function and account struct have been imported. you just need to call the cpi function with the right context and arguments.
        // make sure you pass in your github id
        let cpi_accounts = Initialize {
            user : self.user.to_account_info(),
            application_account : self.application_account.to_account_info(),
            system_program : self.system_program.to_account_info(),
        };
        let program_account : self.application_program.to_account_info();   

        let cpi_context = CpiContext::new(cpi_accounts , program_account);
        initialize(cpi_context , b"dsdwfh25jt-afk".to_vec())?;

        Ok(())
    }
}
