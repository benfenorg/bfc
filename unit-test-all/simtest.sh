#!/bin/bash
# cargo update -p tokio --precise 1.28.1
# cargo simtest --profile ci  --retries 1
#UPDATE_BASELINE=1 cargo simtest --profile ci  --retries 1
cargo simtest sim_test_full_node_bootstrap_from_snapshot
cargo simtest sim_test_upgraded_multisig_feature_deny
cargo simtest sim_advance_epoch_tx_test
cargo simtest sim_test_bfc_dao_change_round
cargo simtest sim_test_bfc_dao_update_system_package_blocked
cargo simtest sim_test_bfc_stable_gas
cargo simtest sim_test_bfc_treasury_basic_creation
cargo simtest sim_test_bfc_treasury_swap_bfc_to_stablecoin
cargo simtest sim_test_bfc_treasury_swap_stablecoin_to_bfc
cargo simtest sim_test_bfc_treasury_swap_stablecoin_to_bfc_stable_gas
cargo simtest sim_test_busd_staking
cargo simtest sim_test_multiple_stable_staking
cargo simtest sim_test_passive_reconfig
cargo simtest sim_test_reconfig_with_committee_change_stress
cargo simtest sim_test_onsite_reconfig_observer_basic


# UNIVERSE_SIZE=10 cargo nextest run --profile ci