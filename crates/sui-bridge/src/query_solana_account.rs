

use solana_sdk::pubkey::Pubkey;


const BRIDGE_CONFIG_SEED: &[u8] = b"bridge_config";
const BRIDGE_COMMITTEE_SEED: &[u8] = b"committee";
const MESSAGE_VERIFIER_SEED: &[u8] = b"message_verifier";
const TOKEN_CONFIG_SEED: &[u8] = b"token_config";
const CHAIN_LIMIT_SEED: &[u8] = b"chain_limit";
const BENFEN_BRIDGE_SEED: &[u8] = b"benfen_bridge";
const UPGRADE_AUTHORITY_SEED: &[u8] = b"upgrade_authority";
const VAULT_SEED: &[u8] = b"vault";
const MESSAGE_CONFIG_SEED: &[u8] = b"message_config";

#[derive(Debug, Clone)]
pub struct AddTokenAccounts {
    pub bridge_config: Pubkey,
    pub bridge_committee: Pubkey,
    pub message_verifier: Pubkey,
    pub bridge_limiter: Pubkey,
    pub benfen_bridge: Pubkey,
    pub token_config: Pubkey,
    pub message_config: Pubkey,
    pub vault: Pubkey,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InitAccounts{
    pub bridge_config: Pubkey,
    pub bridge_committee: Pubkey,
    pub message_verifier: Pubkey,
    pub bridge_limiter: Pubkey,
    pub benfen_bridge: Pubkey,
    pub upgrade_authority: Pubkey,
}

pub(crate) fn get_init_account(program_id: Pubkey,chain_id: u8) -> InitAccounts {
    // Derive PDAs to match Anchor seeds constraints exactly.
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(&[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()], &program_id).0;

    // MessageVerifier seeds: ["message_verifier", committee.key()]
    let message_verifier_pda = Pubkey::find_program_address(
        &[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()],
        &program_id,
    ).0;

    // ChainLimit seeds: ["chain_limit", &[chain_id], bridge_config.key()]
    let bridge_limiter_pda = Pubkey::find_program_address(
        &[CHAIN_LIMIT_SEED, &[chain_id], bridge_config_pda.as_ref()],
        &program_id,
    ).0;

    // BenfenBridge seeds: ["benfen_bridge", committee.key()]
    let benfen_bridge_pda = Pubkey::find_program_address(
        &[BENFEN_BRIDGE_SEED, committee_pda.as_ref()],
        &program_id,
    ).0;

    // UpgradeAuthority seeds: ["upgrade_authority", committee.key()]
    let upgrade_authority_pda = Pubkey::find_program_address(
        &[UPGRADE_AUTHORITY_SEED, committee_pda.as_ref()],
        &program_id,
    ).0;

    InitAccounts {
        bridge_config: bridge_config_pda,
        bridge_committee: committee_pda,
        message_verifier: message_verifier_pda,
        bridge_limiter: bridge_limiter_pda,
        benfen_bridge: benfen_bridge_pda,
        upgrade_authority: upgrade_authority_pda,
    }
}

pub(crate) fn get_add_token_account(
    program_id: Pubkey,
    benfen_chain_id: u8, 
    token_id: u64, 
    message_type: u8
) -> AddTokenAccounts {
    let base = get_init_account(program_id,benfen_chain_id);
    let token_id_bytes = token_id.to_be_bytes();

    let token_config = Pubkey::find_program_address(
        &[TOKEN_CONFIG_SEED, &token_id_bytes],
        &program_id,
    ).0;

    let vault = Pubkey::find_program_address(
        &[VAULT_SEED, &token_id_bytes],
        &program_id,
    ).0;

    let message_config = Pubkey::find_program_address(
        &[MESSAGE_CONFIG_SEED, &[message_type], base.message_verifier.as_ref()],
        &program_id,
    ).0;


    AddTokenAccounts {
        bridge_config: base.bridge_config,
        bridge_committee: base.bridge_committee,
        message_verifier: base.message_verifier,
        bridge_limiter: base.bridge_limiter,
        benfen_bridge: base.benfen_bridge,
        token_config,
        message_config,
        vault,
    }
}