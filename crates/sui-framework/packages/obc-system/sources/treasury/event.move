module obc_system::event {
    use std::ascii::{Self, String};

    use sui::event::emit;
    use sui::object::ID;

    use obc_system::i32::I32;

    friend obc_system::treasury;
    friend obc_system::vault;

    struct InitTreasuryEvent has copy, drop {
        vaults_id: ID
    }

    spec module { pragma verify = false; }

    // Vault added event
    struct CreateVaultEvent has copy, drop {
        vault_id: ID,
        vault_key: String,
        coin_type_a: String,
        coin_type_b: String,
        tick_spacing: u32,
        spacing_times: u32,
        index: u64
    }

    // Position opened event
    struct OpenPositionEvent has copy, drop {
        vault: ID,
        position: u64,
        tick_lower: I32,
        tick_upper: I32
    }

    // Position closed event
    struct ClosePositionEvent has copy, drop {
        vault: ID,
        position: u64
    }

    // Liquidity event
    struct LiquidityEvent has copy, drop {
        vault: ID,
        position: u64,
        tick_lower: I32,
        tick_upper: I32,
        liquidity: u128,
        after_liquidity: u128,
        amount_a: u64,
        amount_b: u64,
        action: String
    }

    // Swap
    struct SwapEvent has copy, drop {
        atob: bool,
        vault: ID,
        amount_in: u64,
        amount_out: u64,
        vault_a_amount: u64,
        vault_b_amount: u64,
        before_sqrt_price: u128,
        after_sqrt_price: u128,
        steps: u64
    }

    // Deposit
    struct DepositEvent has copy, drop {
        amount: u64,
    }

    struct UpdateStateEvent has copy, drop {
        current_sqrt_price: u128,
        last_sqrt_price: u128,
        state: u8,
        state_counter: u32,
    }

    public(friend) fun init_treasury(vaults_id: ID) {
        emit(InitTreasuryEvent { vaults_id })
    }

    public(friend) fun create_vault(
        vault_id: ID,
        vault_key: String,
        coin_type_a: String,
        coin_type_b: String,
        tick_spacing: u32,
        spacing_times: u32,
        index: u64,
    ) {
        emit(CreateVaultEvent {
            vault_id,
            vault_key,
            coin_type_a,
            coin_type_b,
            tick_spacing,
            spacing_times,
            index,
        })
    }

    public(friend) fun open_position(
        vault_id: ID,
        position_id: u64,
        tick_lower: I32,
        tick_upper: I32
    ) {
        emit(
            OpenPositionEvent {
                vault: vault_id,
                position: position_id,
                tick_lower,
                tick_upper
            }
        )
    }

    public(friend) fun close_position(
        vault_id: ID,
        position_id: u64
    ) {
        emit(
            ClosePositionEvent {
                vault: vault_id,
                position: position_id
            }
        )
    }

    public(friend) fun add_liquidity(
        vault_id: ID,
        position_id: u64,
        tick_lower: I32,
        tick_upper: I32,
        liquidity: u128,
        after_liquidity: u128,
        amount_a: u64,
        amount_b: u64
    ) {
        emit(
            LiquidityEvent {
                vault: vault_id,
                position: position_id,
                tick_lower,
                tick_upper,
                liquidity,
                after_liquidity,
                amount_a,
                amount_b,
                action: ascii::string(b"add")
            }
        )
    }

    public(friend) fun remove_liquidity(
        vault_id: ID,
        position_id: u64,
        tick_lower: I32,
        tick_upper: I32,
        liquidity: u128,
        after_liquidity: u128,
        amount_a: u64,
        amount_b: u64
    ) {
        emit(
            LiquidityEvent {
                vault: vault_id,
                position: position_id,
                tick_lower,
                tick_upper,
                liquidity,
                after_liquidity,
                amount_a,
                amount_b,
                action: ascii::string(b"remove")
            }
        )
    }

    public(friend) fun swap(
        vault_id: ID,
        atob: bool, // true a->b false b->a
        amount_in: u64,
        amount_out: u64,
        vault_a_amount: u64, // current vault balance(A)
        vault_b_amount: u64, // current vault balance(B)
        before_sqrt_price: u128,
        after_sqrt_price: u128,
        steps: u64
    ) {
        emit(
            SwapEvent {
                vault: vault_id,
                atob,
                amount_in,
                amount_out,
                vault_a_amount,
                vault_b_amount,
                before_sqrt_price,
                after_sqrt_price,
                steps
            }
        )
    }

    public(friend) fun deposit(amount: u64) {
        emit(
            DepositEvent {
                amount
            }
        )
    }

    public(friend) fun update_state(
        current_sqrt_price: u128,
        last_sqrt_price: u128,
        state: u8,
        state_counter: u32,
    ) {
        emit(
            UpdateStateEvent {
                current_sqrt_price,
                last_sqrt_price,
                state,
                state_counter,
            }
        )
    }
}