use anchor_lang::prelude::*;
use crate::states::committee::{COMMITTEE_SEED,Committee,MAX_COMMITTEE_MEMBERS,CommitteeMember};
use crate::states::bridge_config::BridgeConfig;
use crate::errors::AdminError;
use crate::errors::BridgeCommitteeError;

#[derive(Accounts)]
pub struct InitializeCommittee<'info> {
     #[account(
        mut,
        // address = crate::admin::id() @ AdminError::NotApproved
    )]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Committee::SPACE,
        seeds = [COMMITTEE_SEED.as_bytes(),bridge_config.key().as_ref()],
        bump,
    )]
    pub committee: AccountLoader<'info, Committee>,

    pub bridge_config: AccountLoader<'info, BridgeConfig>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_committee(
    ctx: Context<InitializeCommittee>,
    addresses: Vec<[u8; 20]>,
    stakes: Vec<u16>,
    // indexs:Vec<u8>,
    min_stake_required: u16,
) -> Result<()> {
    let committee_length = addresses.len();
    require!(committee_length <= MAX_COMMITTEE_MEMBERS,BridgeCommitteeError::CommitteeTooLarge);
    require!(stakes.len() == committee_length,BridgeCommitteeError::CommitteeMemberStakeMismatch);
    //require!(indexs.len() == committee_length,BridgeError::CommitteeMemberIndexMismatch);
    
    let mut members=vec![CommitteeMember::default();committee_length];

    for (i, address) in addresses.iter().enumerate() {
        members[i].address = *address;
        members[i].stake = stakes[i];
        members[i].index = i as u8;
    }
    let mut committee =  ctx.accounts.committee.load_init()?;
    committee.initialize(ctx.bumps.committee,&members,min_stake_required)?;
    committee.initialize_config(ctx.accounts.bridge_config.key())
}
