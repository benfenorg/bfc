use anchor_lang::prelude::*;
use crate::states::message_verifier::{self,MessageVerifier};
use crate::states::committee::Committee;
use crate::errors::AdminError;



#[derive(Accounts)]
pub struct InitializeMessageVerifier<'info> {
    #[account(
        mut,
        // address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = MessageVerifier::SPACE,
        seeds = [
            message_verifier::MESSAGE_VERIFIER_SEED.as_bytes(),
            committee.key().as_ref()
        ],
        bump
    )]
    pub verifier: AccountLoader<'info, MessageVerifier>,
    pub committee: AccountLoader<'info, Committee>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_message_verifier(
    ctx: Context<InitializeMessageVerifier>,
) -> Result<()> {
    let mut verifier_loader = ctx.accounts.verifier.load_init()?;
    let key = ctx.accounts.committee.key();
    verifier_loader.initialize(ctx.bumps.verifier,key)
}
