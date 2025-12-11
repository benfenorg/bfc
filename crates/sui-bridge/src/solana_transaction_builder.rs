

use std::sync::Arc;
use sui_types::bridge::BridgeChainId;
use crate::types::{BridgeAction, VerifiedCertifiedBridgeAction};
use crate::types::{AddTokenOnSolanaAction};
use crate::error::{BridgeError, BridgeResult};
use crate::idl::SolanaMessage;
use crate::types::BridgeCommitteeValiditySignInfo; 
use crate::utils::SolanaSigner;

use crate::query_solana_account::{get_init_account, get_add_token_account};
use solana_sdk::{
    signature::Keypair,
    signer::Signer, 
    system_program,
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
        BridgeAction::SingleTransferLimitUpdateAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
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
        BridgeAction::BlocklistCommitteeAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::EmergencyAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::LimitUpdateAction(_) => {
            // It does not need a Sui tranaction to add tokens on EVM
            unreachable!()
        }
        BridgeAction::AssetPriceUpdateAction(_) => {
            unreachable!()
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
        _ => unreachable!(),
    }

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







