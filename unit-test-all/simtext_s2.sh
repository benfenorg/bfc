//need close watchdog, easy abort  with watchdog
MSIM_DISABLE_WATCHDOG=1 cargo simtest sim_test_map_reducer
MSIM_DISABLE_WATCHDOG=1 cargo simtest test_quorum_map_and_reduce_timeout
MSIM_DISABLE_WATCHDOG=1 cargo simtest sim_test_quorum_driver_handling_overload_and_retry
MSIM_DISABLE_WATCHDOG=1 cargo simtest test_validator_tx_finalizer_basic_flow

MSIM_DISABLE_WATCHDOG=1 cargo simtest sim_test_process_certificate_fault_fail
MSIM_DISABLE_WATCHDOG=1 cargo simtest sim_test_process_transaction_fault_fail
MSIM_DISABLE_WATCHDOG=1 cargo simtest sim_test_process_transaction_fault_success
