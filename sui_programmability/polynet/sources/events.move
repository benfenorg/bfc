module polynet::events {
    use std::vector;
    use sui::event::{emit, Self};
    use std::type_name::{Self, TypeName};

    friend polynet::config;
    friend polynet::controller;
    friend polynet::wrapper_v1;
    friend polynet::cross_chain_manager;

    struct InitBookKeeperEvent has store, drop, copy {
        height: u64,
        sender: address,
        keepers: vector<vector<u8>>
    }

    struct LockWithFeeEvent has store, drop, copy{
        from_asset: TypeName,
        from_address: address,
        to_chain_id: u64,
        to_address: vector<u8>,
        amount: u64,
        fee_amount: u64
    }

    struct Migrate has copy,drop {
        sender: address
    }

    struct MigrateBookKeeperEvent has store, drop, copy {
        height: u64,
        sender: address,
        poly_id: u64,
        keepers: vector<vector<u8>>
    }

    struct UpdateBookKeeperEvent has store, drop, copy {
        height: u64,
        sender: address,
        poly_id: u64,
        keepers: vector<vector<u8>>
    }

    struct VerifyHeaderAndExecuteTxEvent has store, drop, copy {
        from_chain_id: u64,
        to_contract: vector<u8>,
        cross_chain_tx_hash: vector<u8>,
        from_chain_tx_hash: vector<u8>,
    }


    public(friend) fun lock_with_fee_event(
        _from_asset: TypeName,
        _from_address: address,
        _to_chain_id: u64,
        _to_address: vector<u8>,
        _amount: u64,
        _fee_amount: u64

    ) {
        emit (
            LockWithFeeEvent{
                from_asset: _from_asset,
                from_address: _from_address,
                to_chain_id: _to_chain_id,
                to_address: _to_address,
                amount: _amount,
                fee_amount: _fee_amount

            })
    }


    public(friend) fun migrate(
        _sender: address
    ) {
        emit(
            Migrate{
                sender: _sender
            })
    }

    public(friend) fun migrate_book_keeper_event(
        _keepers: vector<vector<u8>>,
        _startHeight: u64,
        _polyId: u64,
        _sender: address
    ) {

          event::emit(
            MigrateBookKeeperEvent{
                height: _startHeight,
                sender: _sender,
                poly_id:_polyId,
                keepers: _keepers
            },
        );
    }

    public(friend) fun update_book_keeper_event(
        _keepers: vector<vector<u8>>,
        _startHeight: u64,
        _polyId: u64,
        _sender: address
    ) {

          event::emit(
            UpdateBookKeeperEvent{
                height: _startHeight,
                sender: _sender,
                poly_id:_polyId,
                keepers: _keepers
            },
        );
    }

    public(friend) fun init_book_keeper_event(
        _keepers: vector<vector<u8>>,
        _startHeight: u64,
        _sender: address
    ) {

          event::emit(
            InitBookKeeperEvent{
                height: _startHeight,
                sender: _sender,
                keepers: _keepers
            },
        );
    }

    public(friend) fun verify_header_and_execute_tx_event(
        _from_chain_id: u64,
        _to_contract: vector<u8>,
        _cross_chain_tx_hash: vector<u8>,
        _from_chain_tx_hash: vector<u8>
    ) {

          event::emit(
            VerifyHeaderAndExecuteTxEvent{
                from_chain_id: _from_chain_id,
                to_contract: _to_contract,
                cross_chain_tx_hash: _cross_chain_tx_hash,
                from_chain_tx_hash: _from_chain_tx_hash,
            },
        );
    }










}