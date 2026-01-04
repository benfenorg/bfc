// use crate::states::vault::UniversalVault;
use crate::states::benfen_bridge::BenfenBridge;
use crate::errors::BridgeError;
use anchor_lang::{
    prelude::*,
};
use anchor_spl::token;



pub fn transfer_sol_from_user_to_bridge_vault<'info>(
    signer: &Signer<'info>,
    to_vault: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    rent: &Sysvar<Rent>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    let user_lamports = signer.lamports();
    
    // 获取账户租金豁免所需的最小余额（0字节账户）
    let rent_exempt_minimum = rent.minimum_balance(0);
    
    // 确保用户有足够的余额进行转账，同时保留租金豁免所需的最小余额
    require!(
        user_lamports >= amount.saturating_add(rent_exempt_minimum),
        BridgeError::InsufficientBalance
    );

    // 将SOL从用户转账到bridge vault
    let transfer_ix = anchor_lang::system_program::Transfer {
        from: signer.to_account_info(),
        to: to_vault.to_account_info(),
    };
    anchor_lang::system_program::transfer(
        CpiContext::new(
            system_program.to_account_info(),
            transfer_ix,
        ),
        amount,
    )?;

    Ok(())
}

/// 从bridge vault转账SOL给用户
pub fn transfer_sol_from_vault_to_user<'info>(
    vault: &Account<'info, BenfenBridge>,
    from_vault: &AccountInfo<'info>,
    to_user: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    // 将SOL从bridge vault转账到用户
    let transfer_ix = anchor_lang::system_program::Transfer {
        from: from_vault.to_account_info(),
        to: to_user.to_account_info(),
    };
    anchor_lang::system_program::transfer(
        CpiContext::new_with_signer(
            system_program.to_account_info(),
            transfer_ix,
            &[&vault.seeds()],
        ),
        amount,
    )?;

    Ok(())
}


pub fn transfer_from_user_to_bridge_vault<'info>(
    signer: &Signer<'info>,
    from: &AccountInfo<'info>, //用户持有token的 token account
    to_vault: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    
    token::transfer(
        CpiContext::new(
            token_program.to_account_info(),
            token::Transfer {
                from: from.to_account_info(),
                to: to_vault.to_account_info(),
                authority: signer.to_account_info(),
            },
        ),
        amount,
    )
}

pub fn transfer_from_vault_to_user<'info>(
    vault: &Account<'info, BenfenBridge>,
    from_vault: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }
    
    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token::Transfer {
                from: from_vault.to_account_info(),
                to: to.to_account_info(),
                authority: vault.to_account_info(),
            },
            &[&vault.seeds()],
        ),
        amount,
    )
}




