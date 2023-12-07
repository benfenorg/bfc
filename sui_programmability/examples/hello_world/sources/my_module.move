
module hello_world::counter {
    // Part 1: imports
    use std::string;
    use sui::event;
    use sui::transfer;
    use sui::object::{Self, UID};
    use sui::tx_context::{Self, TxContext};
    #[test_only]
    use std::debug;
    

    // Part 2: struct definition
    struct Counter has key {
        id: UID,
        value: u64,
    }


    //
    struct CountEvent has copy, drop, store {
        name: string::String,
    }



    entry public fun createEvent(ctx: &mut TxContext) {
        let data: vector<u8> = b"hello world";
        event::emit(
            CountEvent{
                name: string::utf8(data),
            }
        );
    }

    // Part 3: transfer the counter object to the sender
    entry public fun getCounter(ctx: &mut TxContext) {
        // sender address
        let sender = tx_context::sender(ctx);
        let counter_obj = Counter {
            id: object::new(ctx),
            value: 0
        };
        transfer::transfer(counter_obj, sender);
    }

    // part 4: public/ entry functions
    public entry fun incr(counter: &mut Counter) {
        counter.value = counter.value + 1;
    } 
    
    
    #[test]
    public fun test_module_init() {
        let data: vector<u8> = b"hello world";
        debug::print(&data);

        let num = 42;
        debug::print(&num);
    }


}




/* quick compile

sui move test test_module_init --skip-fetch-latest-git-deps


*/