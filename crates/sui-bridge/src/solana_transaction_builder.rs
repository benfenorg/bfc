

use std::sync::Arc;
use sui_types::bridge::BridgeChainId;
use crate::types::{BridgeAction, VerifiedCertifiedBridgeAction};
use crate::types::{
    AddTokenOnSolanaAction,AssetPriceUpdateAction,
    SingleTransferLimitUpdateAction,LimitUpdateAction,
    SingleMinTransferLimitUpdateAction,
    ExtendProgramOnSolanaAction,   
    BridgeFeeInfoUpdateAction, 
    UpgradeProgramOnSolanaAction,
    BlocklistCommitteeAction,
    EmergencyAction,
};
use crate::error::{BridgeError, BridgeResult};
use crate::idl::SolanaMessage;
use crate::types::BridgeCommitteeValiditySignInfo; 
use crate::utils::SolanaSigner;

use crate::query_solana_account::{
    get_transfer_limit_account,get_add_token_account,
    get_update_token_price_account,
    get_extend_program_account,
    get_emergency_op_account,
    get_upgrade_program_account,
    get_bridge_fee_info_account,
};
use solana_sdk::{
    signature::Keypair,
    signer::Signer, 
    system_program,
    clock,
    rent,
    pubkey::Pubkey,
    instruction::Instruction,
};
anchor_lang::declare_program!(benfen_bridge);
use benfen_bridge::{
    client::accounts, 
    client::args, 
};
use anchor_lang;
use anchor_client::Program;
use spl_token;


pub async fn build_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: VerifiedCertifiedBridgeAction,
) -> BridgeResult<Instruction>
{
    if !action.is_governace_action() {
        return Err(BridgeError::ActionIsNotGovernanceAction(
            action.data().clone(),
        ));
    }
    let sigs = action.auth_sig();
    // TODO: Check chain id?
    match action.data() {
        BridgeAction::AddTokenOnSolanaAction(action) => {
            build_add_token_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        },
        BridgeAction::SingleTransferLimitUpdateAction(action) => {
            build_single_transfer_limit_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        },

        BridgeAction::SingleMinTransferLimitUpdateAction(action) => {
            build_single_min_transfer_limit_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        },

        BridgeAction::EthToSuiBridgeAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::EthSendBackBridgeAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::ExternalDepositStartBridgeAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::SuiToEthBridgeAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::BlocklistCommitteeAction(action) => {
            build_blocklist_committee_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        }
        BridgeAction::EmergencyAction(action) => {
            // It does not need a Sui tranaction to add tokens on EVM
            build_update_emergency_op_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
            
        }
        BridgeAction::LimitUpdateAction(action) => {
            build_limit_update_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        }
        BridgeAction::AssetPriceUpdateAction(action) => {
            build_update_price_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        }

        BridgeAction::ExtendProgramOnSolanaAction(action ) => {
            build_extend_program_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        }

        BridgeAction::UpgradeProgramOnSolanaAction(action) => {
            build_upgrade_program_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        }

        BridgeAction::BridgeFeeInfoUpdateAction(action) => {
            build_update_bridge_fee_info_on_solana_transaction(
                program,
                solana_chain_id,
                benfen_chain_id,
                signer,
                action.clone(),
                sigs,
            )
            .await
        }

        BridgeAction::EvmContractUpgradeAction(_) => {
            // It does not need a Sui tranaction to execute EVM contract upgrade
            unreachable!()
        }
        BridgeAction::AddExternalCoinAdminAction(_)=>  {
            unreachable!()
        }
        BridgeAction::RemoveExternalCoinAdminAction(_) =>  {
            unreachable!()
        }
        BridgeAction::AddExternalCoinWitnessAction(_) => {
            unreachable!()
        }
        BridgeAction::RemoveExternalCoinWitnessAction(_) => {
            unreachable!()
        }
        BridgeAction::AddExternalCoinTargetAction(_) =>  {
            unreachable!()
        }
        BridgeAction::RemoveExternalCoinTargetAction(_) =>  {
            unreachable!()
        }
        BridgeAction::AddTokenOnTokenListAction(_) =>  {
            unreachable!()
        }
        BridgeAction::RemoveTokenOnTokenListAction(_) =>  {
            unreachable!()
        }
        BridgeAction::UpdateBridgeFeeOnCrossOutAction(_) => {
            unreachable!()
        }
        BridgeAction::UpdateBridgeFeeOnCrossInAction(_) =>  {
            unreachable!()
        }
        BridgeAction::WithdrawBridgeFeeAction(_) =>  {
            unreachable!()
        }
        BridgeAction::AddTokensOnSuiAction(_) => {
            unreachable!()
        }
        BridgeAction::AddTokensOnEvmAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::SolanaToSuiBridgeAction(_) => {
            // Not a governance action handled on Solana side
            unreachable!()
        }
        BridgeAction::RefundAdminAction(_) => {
            unreachable!()
        }
        BridgeAction::FastPathLimitUpdateAction(_) => {
            // It does not need a Sui tranaction to update fast path limit
            unreachable!()
        }
        BridgeAction::SuiToEthDefiBridgeAction(sui_to_eth_defi_bridge_action) => todo!(),
        BridgeAction::EthToSuiDefiBridgeAction(eth_to_sui_defi_bridge_action) => todo!(),
        BridgeAction::UpdateInvestAddressAction(update_invest_address_action) => todo!(),
        BridgeAction::AddLpTokenIdAction(add_lp_token_id_action) => todo!(),
        _ => unreachable!(),
    }

}

pub async fn build_limit_update_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: LimitUpdateAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
    let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
    let payload = message.payload.clone();
    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let limit_accounts = get_transfer_limit_account(
        program_id,
        benfen_chain_id as u8,
        message.message_type,
    );
    let chain_id = action.chain_id as u8;
    let ix = program
        .request()
        .accounts(accounts::UpdateBridgeLimiter {
            payer: signer.pubkey(),
            message_config: limit_accounts.message_config,
            bridge_config: limit_accounts.bridge_config,
            chain_limit: limit_accounts.chain_limit,
            verifier: limit_accounts.message_verifier,
            committee: limit_accounts.bridge_committee,
            system_program: system_program::ID,
        })
        .args(args::UpdateBridgeLimiter {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}

pub async fn build_update_emergency_op_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    _benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: EmergencyAction,
    sigs: &BridgeCommitteeValiditySignInfo,
) -> BridgeResult<Instruction> {
    let program_id = program.id();
    let message: SolanaMessage = action.clone().into();
    let payload = message.payload.clone();

    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let accounts = get_emergency_op_account(program_id);

    let chain_id = action.chain_id as u8;

    let ix = program
        .request()
        .accounts(accounts::UpdateEmergencyOp {
            signer: signer.pubkey(),
            message_config: accounts.message_config,
            bridge_config: accounts.bridge_config,
            verifier: accounts.message_verifier,
            committee: accounts.bridge_committee,
            bridge: accounts.benfen_bridge,
            system_program: system_program::ID,
        })
        .args(args::UpdateEmergencyOp {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}


pub async  fn build_update_price_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: AssetPriceUpdateAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
    let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
    let payload = message.payload.clone();

    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let update_token_price_accounts = get_update_token_price_account(
        program_id,
        benfen_chain_id as u8,
        action.token_id,
        message.message_type,
    );


    let chain_id = action.chain_id as u8;

    let ix = program
        .request()
        .accounts(accounts::UpdateTokenPrice {
            payer: signer.pubkey(),
            message_config: update_token_price_accounts.message_config,
            token_config: update_token_price_accounts.token_config,
            bridge_config: update_token_price_accounts.bridge_config,
            verifier: update_token_price_accounts.message_verifier,
            committee: update_token_price_accounts.bridge_committee,
            system_program: system_program::ID,
        })
        .args(args::UpdateTokenPrice {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}


pub async fn build_update_bridge_fee_info_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: BridgeFeeInfoUpdateAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
    let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
    let payload = message.payload.clone();
    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let limit_accounts = get_bridge_fee_info_account(
        program_id,
        action.sending_chain_id as u8,
        action.token_id,
        message.message_type,
    );

    let chain_id = action.chain_id as u8;

    let ix = program
        .request()
        .accounts(accounts::UpdateTokenFeeInfo {
            payer: signer.pubkey(),
            message_config: limit_accounts.message_config,
            token_config: limit_accounts.token_config,
            chain_limit: limit_accounts.chain_limit,
            bridge_config: limit_accounts.bridge_config,
            verifier: limit_accounts.message_verifier,
            committee: limit_accounts.bridge_committee,
            system_program: system_program::ID,
        })
        .args(args::UpdateTokenFeeInfo {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}

pub async fn build_single_min_transfer_limit_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: SingleMinTransferLimitUpdateAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
     let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
    let payload = message.payload.clone();
    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let limit_accounts = get_transfer_limit_account(
        program_id,
        benfen_chain_id as u8,
        message.message_type,
    );

    let chain_id = action.chain_id as u8;

    let ix = program
        .request()
        .accounts(accounts::UpdateMinSingleTransferLimit {
            payer: signer.pubkey(),
            message_config: limit_accounts.message_config,
            bridge_config: limit_accounts.bridge_config,
            chain_limit: limit_accounts.chain_limit,
            verifier: limit_accounts.message_verifier,
            committee: limit_accounts.bridge_committee,
            system_program: system_program::ID,
        })
        .args(args::UpdateMinSingleTransferLimit {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}

pub async fn build_single_transfer_limit_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: SingleTransferLimitUpdateAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
    let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
    let payload = message.payload.clone();


    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let limit_accounts = get_transfer_limit_account(
        program_id,
        benfen_chain_id as u8,
        message.message_type,
    );

    let chain_id = action.chain_id as u8;

    let ix = program
        .request()
        .accounts(accounts::UpdateSingleTransferLimit {
            payer: signer.pubkey(),
            message_config: limit_accounts.message_config,
            bridge_config: limit_accounts.bridge_config,
            chain_limit: limit_accounts.chain_limit,
            verifier: limit_accounts.message_verifier,
            committee: limit_accounts.bridge_committee,
            system_program: system_program::ID,
        })
        .args(args::UpdateSingleTransferLimit {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}


pub async  fn build_blocklist_committee_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: BlocklistCommitteeAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
    let program_id = program.id();
    let message: SolanaMessage = action.clone().into();
    let payload = message.payload.clone();

    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let accounts = crate::query_solana_account::get_update_committee_blocklist_account(
        program_id,
        benfen_chain_id as u8,
    );

    let chain_id = action.chain_id as u8;

    let ix = program
        .request()
        .accounts(accounts::UpdateBlockList {
            payer: signer.pubkey(),
            message_config: accounts.message_config,
            bridge_config: accounts.bridge_config,
            verifier: accounts.message_verifier,
            committee: accounts.bridge_committee,
            system_program: system_program::ID,
        })
        .args(args::UpdateBlockList {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}


pub async  fn build_extend_program_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: ExtendProgramOnSolanaAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
    let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
    let payload = message.payload.clone();

    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let extend_program_accounts = get_extend_program_account(program_id);

    let chain_id = action.chain_id as u8;

    let ix = program
        .request()
        .accounts(accounts::ExtendProgram {
            payer: signer.pubkey(),
            bridge_config: extend_program_accounts.bridge_config,
            message_config: extend_program_accounts.message_config,
            verifier: extend_program_accounts.message_verifier,
            upgrade_authority: extend_program_accounts.upgrade_authority,
            committee: extend_program_accounts.bridge_committee,
            program: extend_program_accounts.program,
            program_data: extend_program_accounts.program_data,
            system_program: system_program::ID,
            bpf_loader: extend_program_accounts.bpf_loader_upgradeable,
        })
        .args(args::ExtendProgram {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
    Ok(ix)
}

pub async  fn build_upgrade_program_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: UpgradeProgramOnSolanaAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
)-> BridgeResult<Instruction>{
    let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
    let payload = message.payload.clone();
    let buffer = action.implementation.clone();

    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let upgrade_program_accounts = get_upgrade_program_account(program_id);


     let ix=program.request()
        .accounts(accounts::UpgradeProgram {
            payer: signer.pubkey(),
            spill: signer.pubkey(),
            upgrade_authority: upgrade_program_accounts.upgrade_authority,
            buffer,
            program: upgrade_program_accounts.program,
            bridge_config: upgrade_program_accounts.bridge_config,
            verifier: upgrade_program_accounts.message_verifier,
            message_config: upgrade_program_accounts.message_config,
            program_data: upgrade_program_accounts.program_data,
            committee: upgrade_program_accounts.bridge_committee,
            bpf_loader: upgrade_program_accounts.bpf_loader_upgradeable,
            system_program: system_program::ID,
            clock: upgrade_program_accounts.clock,
            rent: upgrade_program_accounts.rent,

        })
        .args(args::UpgradeProgram {
            message_type: message.message_type,
            version: message.version,
            nonce: action.nonce,
            chain_id: action.chain_id as u8,
            payload,
            signatures,
        })
        .instructions()?
        .remove(0);
     Ok(ix)
}


pub async fn build_add_token_on_solana_transaction(
    program: Arc<Program<Arc<Keypair>>>,
    solana_chain_id: BridgeChainId,
    benfen_chain_id: BridgeChainId,
    signer: &SolanaSigner,
    action: AddTokenOnSolanaAction,
    sigs:  &BridgeCommitteeValiditySignInfo,
) -> BridgeResult<Instruction>
{
    let program_id = program.id();
    let message: SolanaMessage =action.clone().into();
   
    let payload = message.payload.clone();

    let (chain_id,nonce,native,token_id,token_address,benfen_decimal,token_price) = (action.clone().chain_id,action.clone().nonce,action.clone().native,action.clone().token_id,action.clone().token_address,action.clone().benfen_decimal,action.clone().token_price);

    let signatures = sigs
        .signatures
        .values()
        .map(|sig| sig.as_ref().to_vec())
        .collect::<Vec<Vec<u8>>>();

    // let init_pdas = get_init_solana_pda(program_id, target_chain_id as u8);

    let add_token_accounts = get_add_token_account(
        program_id,
        benfen_chain_id as u8,
        token_id,
        message.message_type,
    );

    let add_token_ix = program
        .request()
        .accounts(accounts::AddTokenToBridge {
            payer: signer.pubkey(),
            message_config: add_token_accounts.message_config,
            verifier: add_token_accounts.message_verifier,
            token_vault: add_token_accounts.vault,
            token_config: add_token_accounts.token_config,
            chain_limit: add_token_accounts.bridge_limiter,
            bridge_config: add_token_accounts.bridge_config,
            benfen_bridge: add_token_accounts.benfen_bridge,
            token_mint: token_address,
            committee: add_token_accounts.bridge_committee,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        })
        .args(args::AddTokenToBridge {
            token_id,
            message_type: message.message_type,
            version: message.version,
            nonce,
            chain_id: chain_id as u8,
            payload,
            signatures: signatures,
        })
        .instructions()?
        .remove(0);
    Ok(add_token_ix)
}






