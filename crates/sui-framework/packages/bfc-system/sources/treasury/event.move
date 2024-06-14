module bfc_system::event {
    use std::ascii::String;

    use sui::event::emit;
    use sui::object::ID;

    friend bfc_system::treasury;
    friend bfc_system::treasury_pool;
    friend bfc_system::vault;

    spec module { pragma verify = false; }


    struct InitTreasuryEvent has copy, drop {
        vaults_id: ID
    }

    struct InitTreasuryPoolEvent has copy, drop {
        treasury_pool_id: ID
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

    struct PauseEvent has copy, drop {
        vault: ID,
        is_pause: bool
    }

    // Swap
    struct SwapEvent has copy, drop {
        atob: bool,
        vault: ID,
        coin_type_in: String,
        coin_type_out: String,
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
        coin_type: String,
        current_sqrt_price: u128,
        last_sqrt_price: u128,
        state: u8,
        state_counter: u32,
    }

    struct RebalanceEvent has copy, drop {
        coin_type: String,
    }

    public(friend) fun init_treasury(vaults_id: ID) {
        emit(InitTreasuryEvent { vaults_id })
    }

    public(friend) fun init_treasury_pool(treasury_pool_id: ID) {
        emit(InitTreasuryPoolEvent { treasury_pool_id })
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



    public(friend) fun swap(
        vault_id: ID,
        atob: bool, // true a->b false b->a
        coin_type_in: String,
        coin_type_out: String,
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
                coin_type_in,
                coin_type_out,
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
        coin_type: String,
        current_sqrt_price: u128,
        last_sqrt_price: u128,
        state: u8,
        state_counter: u32,
    ) {
        emit(
            UpdateStateEvent {
                coin_type,
                current_sqrt_price,
                last_sqrt_price,
                state,
                state_counter,
            }
        )
    }

    public(friend) fun set_pause(vault_id: ID, is_pause: bool) {
        emit(
            PauseEvent {
                vault: vault_id,
                is_pause
            }
        )
    }

    public(friend) fun rebalance(coin_type: String) {
        emit(
            RebalanceEvent {
                coin_type,
            }
        )
    }
}
