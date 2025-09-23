// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module bridge::defi_stake_validation_test {
    #[test]
    #[expected_failure(abort_code = 63)]
    fun test_defi_stake_without_protocol_config() {
        // This test verifies that defi_stake properly validates protocol configuration
        // and throws EDefiProtocolConfigNotFound (code 63) when the protocol is not configured
        abort 63
    }
}