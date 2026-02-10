

use solana_sdk::pubkey::Pubkey;
use crate::types::BridgeActionType;


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


#[derive(Debug, Clone)]
pub struct UpdateTokenPriceAccounts {
    pub bridge_config: Pubkey,
    pub bridge_committee: Pubkey,
    pub chain_limit: Pubkey,
    pub message_verifier: Pubkey,
    pub token_config: Pubkey,
    pub message_config: Pubkey,
}

#[derive(Debug, Clone)]
pub struct TransferLimitAccounts {
    pub bridge_config: Pubkey,
    pub bridge_committee: Pubkey,
    pub chain_limit: Pubkey,
    pub message_verifier: Pubkey,
    pub message_config: Pubkey,
}

#[derive(Debug, Clone)]
pub struct ExtendProgramAccounts {
    pub bridge_config: Pubkey,
    pub message_verifier: Pubkey,
    pub message_config: Pubkey,
    pub upgrade_authority: Pubkey,
    pub bridge_committee: Pubkey,
    pub program : Pubkey,
    pub program_data: Pubkey,
    pub bpf_loader_upgradeable: Pubkey,
}

#[derive(Debug, Clone)]
pub struct TransferUpgradeAuthorityAccounts {
    pub bridge_config: Pubkey,
    pub bridge_committee: Pubkey,
    pub upgrade_authority: Pubkey,
    pub program_data: Pubkey,
    pub bpf_loader_upgradeable: Pubkey,
}

#[derive(Debug, Clone)]
pub struct EmergencyOpAccounts {
    pub bridge_config: Pubkey,
    pub bridge_committee: Pubkey,
    pub message_verifier: Pubkey,
    pub message_config: Pubkey,
    pub benfen_bridge: Pubkey,
}

#[derive(Debug, Clone)]
pub struct UpgradeProgramAccounts {
    pub upgrade_authority: Pubkey,
    //pub buffer: Pubkey,
    pub program: Pubkey,
    pub bridge_config: Pubkey,
    pub message_verifier: Pubkey,
    pub message_config: Pubkey,
    pub program_data: Pubkey,
    pub bridge_committee: Pubkey,
    pub bpf_loader_upgradeable: Pubkey,
    pub system_program: Pubkey,
    pub clock: Pubkey,
    pub rent: Pubkey, 
}

pub(crate) fn get_upgrade_program_account(
    program_id: Pubkey,
) -> UpgradeProgramAccounts{
    let bpf_loader_upgradeable = Pubkey::try_from("BPFLoaderUpgradeab1e11111111111111111111111").unwrap();
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(
        &[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;

    let message_verifier_pda = Pubkey::find_program_address(
        &[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;

    let message_config_pda = Pubkey::find_program_address(
        &[
            MESSAGE_CONFIG_SEED,
            &[BridgeActionType::UpgradeProgramOnSolana as u8],
            message_verifier_pda.as_ref(),
        ],
        &program_id,
    )
    .0;

    let upgrade_authority_pda = Pubkey::find_program_address(
        &[UPGRADE_AUTHORITY_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;
    let program_data_pda = Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable).0;

    UpgradeProgramAccounts {
        upgrade_authority: upgrade_authority_pda,
        //buffer: buffer_pda,
        program: program_id,
        bridge_config: bridge_config_pda,
        message_verifier: message_verifier_pda,
        message_config: message_config_pda,
        program_data: program_data_pda,
        bridge_committee: committee_pda,
        bpf_loader_upgradeable,
        system_program: Pubkey::try_from("Sysvar1111111111111111111111111111111111111").unwrap(),
        clock: Pubkey::try_from("SysvarC1ock11111111111111111111111111111111").unwrap(),
        rent: Pubkey::try_from("SysvarRent111111111111111111111111111111111").unwrap(),
    }
}


pub(crate) fn get_extend_program_account(
    program_id: Pubkey,
) -> ExtendProgramAccounts {
    let bpf_loader_upgradeable = Pubkey::try_from("BPFLoaderUpgradeab1e11111111111111111111111").unwrap();
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(
        &[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;

    let message_verifier_pda = Pubkey::find_program_address(
        &[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;

    let message_config_pda = Pubkey::find_program_address(
        &[
            MESSAGE_CONFIG_SEED,
            &[BridgeActionType::ExtendProgramOnSolana as u8],
            message_verifier_pda.as_ref(),
        ],
        &program_id,
    )
    .0;

    let upgrade_authority_pda = Pubkey::find_program_address(
        &[UPGRADE_AUTHORITY_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;
    let program_data_pda = Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable).0;

    ExtendProgramAccounts {
        bridge_config: bridge_config_pda,
        message_verifier: message_verifier_pda,
        message_config: message_config_pda,
        upgrade_authority: upgrade_authority_pda,
        bridge_committee: committee_pda,
        program: program_id,
        program_data: program_data_pda,
        bpf_loader_upgradeable,
    }
}

#[derive(Debug, Clone)]
pub struct UpdateCommitteeBlocklistAccounts {
    pub bridge_config: Pubkey,
    pub bridge_committee: Pubkey,
    pub chain_limit: Pubkey,
    pub message_verifier: Pubkey,
    pub message_config: Pubkey,
}


pub(crate) fn get_emergency_op_account(
    program_id: Pubkey,
) -> EmergencyOpAccounts {
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(
        &[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;
    let message_verifier_pda = Pubkey::find_program_address(
        &[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;
    let message_config_pda = Pubkey::find_program_address(
        &[
            MESSAGE_CONFIG_SEED,
            &[BridgeActionType::EmergencyButton as u8],
            message_verifier_pda.as_ref(),
        ],
        &program_id,
    )
    .0;
    let benfen_bridge_pda = Pubkey::find_program_address(
        &[BENFEN_BRIDGE_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;

    EmergencyOpAccounts {
        bridge_config: bridge_config_pda,
        bridge_committee: committee_pda,
        message_verifier: message_verifier_pda,
        message_config: message_config_pda,
        benfen_bridge: benfen_bridge_pda,
    }
}

pub(crate) fn get_update_committee_blocklist_account(
    program_id: Pubkey,
    benfen_chain_id: u8,
) -> UpdateCommitteeBlocklistAccounts {
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(
        &[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;
    let chain_limit_pda = Pubkey::find_program_address(
        &[CHAIN_LIMIT_SEED, &[benfen_chain_id], bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;
    let message_verifier_pda = Pubkey::find_program_address(
        &[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;
    let message_config_pda = Pubkey::find_program_address(
        &[
            MESSAGE_CONFIG_SEED,
            &[BridgeActionType::UpdateCommitteeBlocklist as u8],
            message_verifier_pda.as_ref(),
        ],
        &program_id,
    )
    .0;
    UpdateCommitteeBlocklistAccounts {
        bridge_config: bridge_config_pda,
        bridge_committee: committee_pda,
        chain_limit: chain_limit_pda,
        message_verifier: message_verifier_pda,
        message_config: message_config_pda,
    }
}
pub(crate) fn get_transfer_upgrade_authority_account(
    program_id: Pubkey,
) -> TransferUpgradeAuthorityAccounts {
    let bpf_loader_upgradeable = Pubkey::try_from("BPFLoaderUpgradeab1e11111111111111111111111").unwrap();
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(
        &[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;
    // 计算升级权限PDA
    let upgrade_authority_pda = Pubkey::find_program_address(
        &[UPGRADE_AUTHORITY_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;
    
    // 计算programDataAddress
    let program_data_pda = Pubkey::find_program_address(&[program_id.as_ref()], &bpf_loader_upgradeable).0;

    TransferUpgradeAuthorityAccounts {
        bridge_config: bridge_config_pda,
        bridge_committee: committee_pda,
        upgrade_authority: upgrade_authority_pda,
        program_data: program_data_pda,
        bpf_loader_upgradeable,
    }
}




pub(crate) fn get_update_token_price_account(
    program_id: Pubkey,
    benfen_chain_id: u8,
    token_id: u64,
    message_type: u8,
) -> UpdateTokenPriceAccounts {
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(
        &[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;
    let chain_limit_pda = Pubkey::find_program_address(
        &[CHAIN_LIMIT_SEED, &[benfen_chain_id], bridge_config_pda.as_ref()],
        &program_id,
    )
    .0;
    let message_verifier_pda = Pubkey::find_program_address(
        &[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()],
        &program_id,
    )
    .0;
    let token_id_bytes = token_id.to_be_bytes();
    let token_config_pda =
        Pubkey::find_program_address(&[TOKEN_CONFIG_SEED, &token_id_bytes], &program_id).0;
    let message_config_pda = Pubkey::find_program_address(
        &[
            MESSAGE_CONFIG_SEED,
            &[message_type],
            message_verifier_pda.as_ref(),
        ],
        &program_id,
    )
    .0;

    UpdateTokenPriceAccounts {
        bridge_config: bridge_config_pda,
        bridge_committee: committee_pda,
        chain_limit: chain_limit_pda,
        message_verifier: message_verifier_pda,
        token_config: token_config_pda,
        message_config: message_config_pda,
    }
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

pub(crate) fn get_chain_limit_pda(program_id: Pubkey, benfen_chain_id: u8) -> Pubkey {
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    Pubkey::find_program_address(&[CHAIN_LIMIT_SEED, &[benfen_chain_id], bridge_config_pda.as_ref()], &program_id).0
}

pub(crate) fn get_message_config_pda(program_id: Pubkey, message_type: u8) -> Pubkey {
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(&[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()], &program_id).0;
    let message_verifier_pda = Pubkey::find_program_address(&[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()], &program_id).0;
    Pubkey::find_program_address(&[MESSAGE_CONFIG_SEED, &[message_type], message_verifier_pda.as_ref()], &program_id).0
}

//单笔限额和24小时限额可以复用
pub(crate) fn get_transfer_limit_account(
    program_id: Pubkey,
    benfen_chain_id: u8,
    message_type: u8,
) -> TransferLimitAccounts {
    let bridge_config_pda = Pubkey::find_program_address(&[BRIDGE_CONFIG_SEED], &program_id).0;
    let committee_pda = Pubkey::find_program_address(&[BRIDGE_COMMITTEE_SEED, bridge_config_pda.as_ref()], &program_id).0;
    let chain_limit_pda = Pubkey::find_program_address(&[CHAIN_LIMIT_SEED, &[benfen_chain_id], bridge_config_pda.as_ref()], &program_id).0;
    let message_verifier_pda = Pubkey::find_program_address(&[MESSAGE_VERIFIER_SEED, committee_pda.as_ref()], &program_id).0;
    let message_config_pda = Pubkey::find_program_address(&[MESSAGE_CONFIG_SEED, &[message_type], message_verifier_pda.as_ref()], &program_id).0;
    TransferLimitAccounts {
        bridge_config: bridge_config_pda,
        bridge_committee: committee_pda,
        chain_limit: chain_limit_pda,
        message_verifier: message_verifier_pda,
        message_config: message_config_pda,
    }
}
