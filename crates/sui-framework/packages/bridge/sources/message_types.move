// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module bridge::message_types {
    // message types
    const TOKEN: u8 = 0;
    const COMMITTEE_BLOCKLIST: u8 = 1;
    const EMERGENCY_OP: u8 = 2;
    const UPDATE_BRIDGE_LIMIT: u8 = 3;
    const UPDATE_ASSET_PRICE: u8 = 4;
    const ADD_TOKENS_ON_SUI: u8 = 6;
    const REFUND_ADMIN_OPERATE: u8 = 8;

    // for external coins
    const ADD_EXTERNAL_COIN_ADMIN: u8 = 11;
    const REMOVE_EXTERNAL_COIN_ADMIN: u8 = 12;
    const ADD_EXTERNAL_COIN_WITNESS: u8 =13;
    const REMOVE_EXTERNAL_COIN_WITNESS: u8 =14;
    const ADD_EXTERNAL_COIN_TARGET: u8 =15;
    const REMOVE_EXTERNAL_COIN_TARGET: u8 =16;
    const ADD_TOKEN_ON_TOKEN_LIST: u8 =17;
    const REMOVE_TOKEN_ON_TOKEN_LIST: u8 =18;

    const SET_CROSS_OUT_BRIDGE_FEE: u8=20;
    const SET_CROSS_IN_BRIDGE_FEE: u8=21;

    const WITHDRAW_BRIDGE_FEE: u8=22;

    public fun token(): u8 { TOKEN }

    public fun committee_blocklist(): u8 { COMMITTEE_BLOCKLIST }

    public fun emergency_op(): u8 { EMERGENCY_OP }

    public fun update_bridge_limit(): u8 { UPDATE_BRIDGE_LIMIT }

    public fun update_asset_price(): u8 { UPDATE_ASSET_PRICE }

    public fun add_tokens_on_sui(): u8 { ADD_TOKENS_ON_SUI }


    // for external coins
    public fun add_external_coin_admin(): u8 { ADD_EXTERNAL_COIN_ADMIN }

    public fun remove_external_coin_admin(): u8 { REMOVE_EXTERNAL_COIN_ADMIN }

    public fun refund_admin_operate(): u8 { REFUND_ADMIN_OPERATE }

    public fun add_external_coin_witness(): u8 { ADD_EXTERNAL_COIN_WITNESS }

    public fun remove_external_coin_witness(): u8 { REMOVE_EXTERNAL_COIN_WITNESS }

    public fun add_external_coin_target(): u8{
        ADD_EXTERNAL_COIN_TARGET
    }
    public fun remove_external_coin_target(): u8{
        REMOVE_EXTERNAL_COIN_TARGET
    }

    public fun add_token_on_token_list(): u8{
        ADD_TOKEN_ON_TOKEN_LIST
    }

    public fun remove_token_on_token_list(): u8{
        REMOVE_TOKEN_ON_TOKEN_LIST
    }

    public fun set_cross_out_bridge_fee(): u8{
        SET_CROSS_OUT_BRIDGE_FEE
    }

    public fun set_cross_in_bridge_fee(): u8{
        SET_CROSS_IN_BRIDGE_FEE
    }

    public fun withdraw_bridge_fee(): u8{
        WITHDRAW_BRIDGE_FEE
    }

}
