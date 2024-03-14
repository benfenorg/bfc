module polynet::events {
    // use std::vector;
    use sui::event::{emit, Self};
    use std::type_name::{TypeName};

    friend polynet::config;
    friend polynet::controller;
    friend polynet::wrapper_v1;
    friend polynet::lock_proxy;
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

    struct LockEvent has store, drop, copy {
        from_asset: TypeName,
        from_address: address,
        to_chain_id: u64,
        to_asset_hash: vector<u8>,
        to_address: vector<u8>,
        amount: u64,
        target_chain_amount: u128
    }

    struct UpdatePolyIdEvent has store, drop, copy {
        poly_id: u64,
        sender: address
    }

    struct UpdateStartHeightEvent has store, drop, copy {
        height: u64,
        sender: address
    }

    struct BlacklistEvent has store, drop, copy {
        license_id: vector<u8>,
        access_level: u8, 
        sender: address
    }

    struct ReadAssetEvent has store, drop, copy {
        to_asset: vector<u8>,
        from_chain_id: u64,
        decimals: u8
    }

    struct CertificateEvent has store, drop, copy {
        from_asset: vector<u8>,
        to_chain_id: u64,
        target_license_id: vector<u8>,
        method: vector<u8>,
        args: vector<u8>

    }

     struct IssueLicenseEvent has store, drop, copy {
        module_name: vector<u8>,
        contract: address,
        license_id: vector<u8>
    }

    public(friend) fun issue_license(
        _module_name: vector<u8>,
        _contract: address,
        _license_id: vector<u8>
    ) {
        emit(
            IssueLicenseEvent{
                    module_name: _module_name,
                    contract: _contract,
                    license_id: _license_id
                }
        );
    }

    public(friend) fun read_certificate(
        _from_contract: vector<u8>,
        _from_chain_id: u64,
        _target_license_id: vector<u8>,
        _method: vector<u8>,
        _args: vector<u8>
    ) {
        emit(
            CertificateEvent{
                    from_asset: _from_contract,
                    to_chain_id: _from_chain_id,
                    target_license_id: _target_license_id,
                    method: _method,
                    args: _args
            }
        );
    }

     public(friend) fun read_asset(
        _to_asset: vector<u8>,
        _from_chain_id: u64,
        _decimals: u8
    ) {
        emit(
            ReadAssetEvent{
                    to_asset: _to_asset,
                    from_chain_id: _from_chain_id,
                    decimals: _decimals
            }
         );

    }

    public(friend) fun set_blacklist_event(
        _license_id: vector<u8>,
        _access_level: u8, 
        _sender: address
    ) {
        emit (
            BlacklistEvent{
                license_id: _license_id,
                access_level: _access_level,
                sender: _sender
            }
        )
    }

    // struct UpdateBookKeeperEvent has store, drop, copy {
    //     height: u64,
    //     sender: address,
    //     keepers: vector<vector<u8>>,
    //     poly_id: u64
    // }

    public(friend) fun update_poly_id_event(
        _poly_id: u64,
        _sender: address
    ) {
        emit (
            UpdatePolyIdEvent{
                poly_id: _poly_id,
                sender: _sender
            })
    }

    public(friend) fun update_start_height_event(
        _height: u64,
        _sender: address
    ) {
        emit (
            UpdateStartHeightEvent{
                height: _height,
                sender: _sender
            })
    }

     public(friend) fun update_book_keeper_event(
        _height: u64,
        _sender: address,
        _keepers: vector<vector<u8>>,
        _poly_id: u64
    ) {
        emit (
            UpdateBookKeeperEvent{
                height: _height,
                sender: _sender,
                keepers: _keepers,
                poly_id: _poly_id
            })
    }


     public(friend) fun lock_event(
        _from_asset: TypeName,
        _from_address: address,
        _to_chain_id: u64,
        _to_asset_hash: vector<u8>,
        _to_address: vector<u8>,
        _amount: u64,
        _target_chain_amount: u128
    ) {
        emit (
            LockEvent{
                from_asset: _from_asset,
                from_address: _from_address,
                to_chain_id: _to_chain_id,
                to_asset_hash: _to_asset_hash,
                to_address: _to_address,
                amount: _amount,
                target_chain_amount: _target_chain_amount

            })
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