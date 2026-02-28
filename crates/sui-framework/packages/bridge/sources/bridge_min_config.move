module bridge::bridge_min_config{
    use sui::table::{Self, Table};
    use sui::dynamic_field;

    const KEY: vector<u8> = b"bridge_min_config";
    const EBridgeMinConfigRegistryAlreadyExists: u64=0;

    public struct BridgeMinConfig has store {
        min_limit_out: Table<u64, u64>,
        min_limit_in: Table<u64, u64>,
        min_fee_out: Table<u64, u64>,
        min_fee_in: Table<u64, u64>,
    }

    public(package) fun new_bridge_min_config_registry(parent_id: &mut UID,ctx: &mut TxContext) {
        assert!(
            !dynamic_field::exists_(parent_id, KEY),
            EBridgeMinConfigRegistryAlreadyExists
        );
        dynamic_field::add(
            parent_id,
            KEY,
            new(ctx),
        );
    }

    public(package) fun new(ctx: &mut TxContext): BridgeMinConfig {
        let (min_limit_out, min_limit_in, min_fee_out, min_fee_in) = empty(ctx);
        BridgeMinConfig {
            min_limit_out,
            min_limit_in,
            min_fee_out,
            min_fee_in,
        }
    }

    fun empty(ctx: &mut TxContext): (
        Table<u64, u64>,
        Table<u64, u64>,
        Table<u64, u64>,
        Table<u64, u64>,
    ) {
        let min_limit_out = table::new<u64, u64>(ctx);
        let min_limit_in = table::new<u64, u64>(ctx);
        let min_fee_out = table::new<u64, u64>(ctx);
        let min_fee_in = table::new<u64, u64>(ctx);
        (min_limit_out, min_limit_in, min_fee_out, min_fee_in)
    }

    public(package) fun borrow(parent_id: &UID): &BridgeMinConfig{
        dynamic_field::borrow<vector<u8>,BridgeMinConfig>(parent_id, KEY)
    }

    public(package) fun borrow_mut(parent_id: &mut UID): &mut BridgeMinConfig{
        dynamic_field::borrow_mut<vector<u8>,BridgeMinConfig>(parent_id, KEY)
    }

    public(package) fun set_min_limit_cross_out(
        parent_id: &mut UID,
        chain_id: u64,
        amount: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.min_limit_out.contains(chain_id)) {
            self.min_limit_out.add(chain_id, amount);
        }else{
            *self.min_limit_out.borrow_mut(chain_id)=amount
        }
    }

    public(package) fun set_min_limit_cross_in(
        parent_id: &mut UID,
        chain_id: u64,
        amount: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.min_limit_in.contains(chain_id)) {
            self.min_limit_in.add(chain_id, amount);
        }else{
            *self.min_limit_in.borrow_mut(chain_id)=amount
        }
    }

    public(package) fun set_min_fee_cross_out(
        parent_id: &mut UID,
        chain_id: u64,
        fee_amount: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.min_fee_out.contains(chain_id)) {
            self.min_fee_out.add(chain_id, fee_amount);
        }else{
            *self.min_fee_out.borrow_mut(chain_id)=fee_amount
        }
    }

    public(package) fun set_min_fee_cross_in(
        parent_id: &mut UID,
        chain_id: u64,
        fee_amount: u64
    ) {
        let self=borrow_mut(parent_id);
        if (!self.min_fee_in.contains(chain_id)) {
            self.min_fee_in.add(chain_id, fee_amount);
        }else{
            *self.min_fee_in.borrow_mut(chain_id)=fee_amount
        }
    }

    public fun get_min_limit_cross_out(parent_id: &UID,chain_id: u64): u64{
        let self=borrow(parent_id);
        if (!self.min_limit_out.contains(chain_id)) {
            return 0
        };
        *self.min_limit_out.borrow(chain_id)
    }

    public fun get_min_limit_cross_in(parent_id: &UID,chain_id: u64): u64{
        let self=borrow(parent_id);
        if (!self.min_limit_in.contains(chain_id)) {
            return 0
        };
        *self.min_limit_in.borrow(chain_id)
    }

    public fun get_min_fee_cross_out(parent_id: &UID,chain_id: u64): u64{
        let self=borrow(parent_id);
        if (!self.min_fee_out.contains(chain_id)) {
            return 0
        };
        *self.min_fee_out.borrow(chain_id)
    }

    public fun get_min_fee_cross_in(parent_id: &UID,chain_id: u64): u64{
        let self=borrow(parent_id);
        if (!self.min_fee_in.contains(chain_id)) {
            return 0
        };
        *self.min_fee_in.borrow(chain_id)
    }
}
