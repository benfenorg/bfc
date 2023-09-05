module obc_system::stable_coin {

    use sui::balance::{Self, Balance};
    use sui::object;
    use sui::object::UID;
    use sui::tx_context::{Self, TxContext};

    struct DummyCoin<phantom T> has key,store {
        id: UID,
        balance: Balance<T>,
        creator: address
    }

    public fun creator<T>(c: &DummyCoin<T>): address {
        c.creator
    }

    /// Get the `DummyCoin.balance.value` field;
    public fun value<T>(c: &DummyCoin<T>): u64 {
        balance::value(&c.balance)
    }

    // === Necessary set of Methods (provide security guarantees and balance access) ===

    /// Get an immutable reference to the Balance of a DummyCoin;
    public fun borrow<T:drop>(_: T, coin: &DummyCoin<T>): &Balance<T> {
        &coin.balance
    }

    /// Get a mutable reference to the Balance of a DummyCoin;
    public fun borrow_mut<T>(coin: &mut DummyCoin<T>): &mut Balance<T> {
        &mut coin.balance
    }

    /// Author of the currency can restrict who is allowed to create new balances;
    public fun zero<T:drop>(_: T,creator: address,ctx: &mut TxContext): DummyCoin<T> {
        DummyCoin<T> {  id: object::new(ctx), balance: balance::zero(), creator}
    }

    /// Build a transferable `DummyCoin` from a `Balance`;
    public fun from_balance<T:drop>(
        _: T, balance: Balance<T>, creator: address,ctx: &mut TxContext
    ): DummyCoin<T> {
        DummyCoin { id: object::new(ctx), balance ,creator}
    }

    /// Destroy `DummyCoin` and return its `Balance`;
    public fun into_balance<T>(dcoin: DummyCoin<T>): Balance<T> {
        let DummyCoin<T> { balance, creator: _, id } = dcoin;
        sui::object::delete(id);
        balance
    }

    // === Optional Methods (can be used for simpler implementation of basic operations) ===

    /// Join Balances of a `DummyCoin` c1 and `DummyCoin` c2.
    public fun join<T>(c1: &mut DummyCoin<T>, c2: DummyCoin<T>) {
        balance::join(borrow_mut(c1), into_balance(c2));
    }

    public fun take<T>(
        balance: &mut Balance<T>, value: u64, ctx: &mut TxContext,
    ): DummyCoin<T> {
        let sender = tx_context::sender(ctx);
        DummyCoin {
            id: object::new(ctx),
            balance: balance::split(balance, value),
            creator:sender
        }
    }

    /// Subtract `DummyCoin` with `value` from `DummyCoin`.
    ///
    /// This method does not provide any checks by default and can possibly lead to mocking
    /// behavior of `DummyCoin::zero()` when a value is 0. So in case empty balances
    /// should not be allowed, this method should be additionally protected against zero value.
    public fun split<T:drop>(
        witness: T, c1: &mut DummyCoin<T>, creator: address,value: u64, ctx: &mut TxContext
    ): DummyCoin<T> {
        let balance = balance::split(borrow_mut(c1), value);
        from_balance(witness, balance,creator, ctx)
    }

    public fun new_dummy<T>(ctx: &mut TxContext): DummyCoin<T> {
        let sender = tx_context::sender(ctx);
        DummyCoin {
            id: object::new(ctx),
            balance:  balance::zero(),
            creator:sender}
    }

}

module obc_system::usdx{
    use obc_system::stable_coin::{Self as rcoin,DummyCoin};
    use sui::tx_context::{Self, TxContext};
    use sui::balance::{Self, Supply, Balance};
    use sui::object::{Self, UID};
    use sui::coin::{Self, Coin};
    use sui::transfer;
    use std::vector;
    use obc_system::stable_coin;

    /// The ticker of Abc regulated token
    struct Usdx has drop {}

    /// A restricted transfer of Abc to another account.
    struct Transfer has key {
        id: UID,
        balance: Balance<Usdx>,
        to: address,
    }

    /// A registry of addresses banned from using the coin.
    struct Registry has key {
        id: UID,
        banned: vector<address>,
        swapped_amount: u64,
    }

    /// A AbcTreasuryCap for the balance::Supply.
    struct AbcTreasuryCap has key, store {
        id: UID,
        supply: Supply<Usdx>
    }

    /// For when an attempting to interact with another account's RegulatedCoin<Abc>.
    const ENotOwner: u64 = 1;

    /// For when address has been banned and someone is trying to access the balance
    const EAddressBanned: u64 = 2;

    #[allow(unused_function)]
    /// Create the Abc currency and send the AbcTreasuryCap to the creator
    /// as well as the first (and empty) balance of the RegulatedCoin<Abc>.
    ///
    /// Also creates a shared Registry which holds banned addresses.
    fun init(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        let treasury_cap = AbcTreasuryCap {
            id: object::new(ctx),
            supply: balance::create_supply(Usdx {})
        };

        transfer::public_transfer(zero(sender, ctx), sender);
        transfer::public_transfer(treasury_cap, sender);

        transfer::share_object(Registry {
            id: object::new(ctx),
            banned: vector::empty(),
            swapped_amount: 0,
        });
    }

    // === Getters section: Registry ===

    /// Get total amount of `Coin` from the `Registry`.
    public fun swapped_amount(r: &Registry): u64 {
        r.swapped_amount
    }

    /// Get vector of banned addresses from `Registry`.
    public fun banned(r: &Registry): &vector<address> {
        &r.banned
    }

    // === Admin actions: creating balances, minting coins and banning addresses ===

    /// Create an empty `RCoin<Abc>` instance for account `for`. AbcTreasuryCap is passed for
    /// authentication purposes - only admin can create new accounts.
    public entry fun create(_: &AbcTreasuryCap, for: address, ctx: &mut TxContext) {
        transfer::public_transfer(zero(for, ctx), for)
    }

    /// Mint more Abc. Requires AbcTreasuryCap for authorization, so can only be done by admins.
    public entry fun mint(treasury: &mut AbcTreasuryCap, owned: &mut DummyCoin<Usdx>, value: u64) {
        balance::join(borrow_mut(owned), balance::increase_supply(&mut treasury.supply, value));
    }

    /// Burn `value` amount of `RCoin<Abc>`. Requires AbcTreasuryCap for authorization, so can only be done by admins.
    ///
    /// TODO: Make AbcTreasuryCap a part of Balance module instead of Coin.
    public entry fun burn(treasury: &mut AbcTreasuryCap, owned: &mut DummyCoin<Usdx>, value: u64) {
        balance::decrease_supply(
            &mut treasury.supply,
            balance::split(borrow_mut(owned), value)
        );
    }

    /// Ban some address and forbid making any transactions from or to this address.
    /// Only owner of the AbcTreasuryCap can perform this action.
    public entry fun ban(_cap: &AbcTreasuryCap, registry: &mut Registry, to_ban: address) {
        vector::push_back(&mut registry.banned, to_ban)
    }

    // === Public: Regulated transfers ===

    /// Transfer entrypoint - create a restricted `Transfer` instance and transfer it to the
    /// `to` account for being accepted later.
    /// Fails if sender is not an creator of the `RegulatedCoin` or if any of the parties is in
    /// the ban list in Registry.
    public entry fun transfer(r: &Registry, coin: &mut DummyCoin<Usdx>, value: u64, to: address, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);

        assert!(rcoin::creator(coin) == sender, ENotOwner);
        assert!(vector::contains(&r.banned, &to) == false, EAddressBanned);
        assert!(vector::contains(&r.banned, &sender) == false, EAddressBanned);

        transfer::transfer(Transfer {
            to,
            id: object::new(ctx),
            balance: balance::split(borrow_mut(coin), value),
        }, to)
    }

    /// Accept an incoming transfer by joining an incoming balance with an owned one.
    ///
    /// Fails if:
    /// 1. the `RegulatedCoin<Abc>.creator` does not match `Transfer.to`;
    /// 2. the address of the creator/recipient is banned;
    public entry fun accept_transfer(r: &Registry, coin: &mut DummyCoin<Usdx>, transfer: Transfer) {
        let Transfer { id, balance, to } = transfer;

        assert!(rcoin::creator(coin) == to, ENotOwner);
        assert!(vector::contains(&r.banned, &to) == false, EAddressBanned);

        balance::join(borrow_mut(coin), balance);
        object::delete(id)
    }

    // === Public: Swap RegulatedCoin <-> Coin ===

    /// Take `value` amount of `RegulatedCoin` and make it freely transferable by wrapping it into
    /// a `Coin`. Update `Registry` to keep track of the swapped amount.
    ///
    /// Fails if:
    /// 1. `RegulatedCoin<Abc>.creator` was banned;
    /// 2. `RegulatedCoin<Abc>` is not owned by the tx sender;
    public entry fun take(r: &mut Registry, coin: &mut DummyCoin<Usdx>, value: u64, ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);

        assert!(rcoin::creator(coin) == sender, ENotOwner);
        assert!(vector::contains(&r.banned, &sender) == false, EAddressBanned);

        // Update swapped amount for Registry to keep track of non-regulated amounts.
        r.swapped_amount = r.swapped_amount + value;

        transfer::public_transfer(coin::take(borrow_mut(coin), value, ctx), sender);
    }

    /// Take `Coin` and put to the `RegulatedCoin`'s balance.
    ///
    /// Fails if:
    /// 1. `RegulatedCoin<Abc>.creator` was banned;
    /// 2. `RegulatedCoin<Abc>` is not owned by the tx sender;
    public entry fun put_back(r: &mut Registry, rc_coin: &mut DummyCoin<Usdx>, coin: Coin<Usdx>, ctx: &TxContext) {
        let balance = coin::into_balance(coin);
        let sender = tx_context::sender(ctx);

        assert!(rcoin::creator(rc_coin) == sender, ENotOwner);
        assert!(vector::contains(&r.banned, &sender) == false, EAddressBanned);

        // Update swapped amount as in `swap_regulated`.
        r.swapped_amount = r.swapped_amount - balance::value(&balance);

        balance::join(borrow_mut(rc_coin), balance);
    }

    // === Private implementations accessors and type morphing ===

    #[allow(unused_function)]
    fun borrow(coin: &DummyCoin<Usdx>): &Balance<Usdx> { stable_coin::borrow(Usdx {}, coin) }
    fun borrow_mut(coin: &mut DummyCoin<Usdx>): &mut Balance<Usdx> { stable_coin::borrow_mut(coin) }
    fun zero(creator: address, ctx: &mut TxContext): DummyCoin<Usdx> { stable_coin::zero(Usdx {}, creator, ctx) }

    // === Testing utilities ===

    #[test_only] public fun init_for_testing(ctx: &mut TxContext) { init(ctx) }
    #[test_only] public fun borrow_for_testing(coin: &DummyCoin<Usdx>): &Balance<Usdx> { borrow(coin) }
    #[test_only] public fun borrow_mut_for_testing(coin: &mut DummyCoin<Usdx>): &Balance<Usdx> { borrow_mut(coin) }

}