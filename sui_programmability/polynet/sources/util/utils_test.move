#[test_only]
module polynet::utils_test {

    #[test_only]
    use std::debug::print;
    use polynet::utils::{Self, to_address};
    use sui::address::to_bytes;
    use sui::table;
    use polynet::acl::{is_admin, default_admin_address};
    use sui::test_scenario as ts;


    #[test]
    public fun test_utils_fun(){
        let add = @0x01;
        let result =  is_admin(add);
        assert!(result == false, 1);

        let default_admin = default_admin_address();
        let result = is_admin(default_admin);
        assert!(result == true, 2);

        let u8_vec = to_bytes(default_admin);
        print(&u8_vec);

        let add_format = to_address(u8_vec);
        print(&add_format);

    }


    #[test]
    public fun test_table_upsert_test() {
        let sender = @0x0;
        let scenario = ts::begin(sender);
        let table = table::new(ts::ctx(&mut scenario));
        // add fields
        table::add(&mut table, b"hello", 0);
        table::add(&mut table, b"goodbye", 1);
        // check the values
        assert!(*table::borrow(&table, b"hello") == 0, 0);
        assert!(*table::borrow(&table, b"goodbye") == 1, 0);
        // update the values
        utils::upsert(&mut table, b"hello", 2);
        utils::upsert(&mut table, b"goodbye", 3);

        // remove the value and check it
        assert!(table::remove(&mut table, b"hello") == 2, 0);
        assert!(table::remove(&mut table, b"goodbye") == 3, 0);
        ts::end(scenario);
        table::destroy_empty(table);
    }

    #[test]
    public fun test_borrow_mut_with_default() {
        let sender = @0x0;
        let scenario = ts::begin(sender);
        let table = table::new(ts::ctx(&mut scenario));

        // check before
        assert!(table:: contains(&table, b"hello") == false, 0);
        assert!(table:: contains(&table, b"goodbye") == false, 0);

        // check the values
        assert!(*utils::borrow_mut_with_default(&mut table, b"hello", 1) == 1, 0);
        assert!(*utils::borrow_mut_with_default(&mut table, b"goodbye", 2) == 2, 0);

        // check after
        assert!(table:: contains(&table, b"hello"), 0);
        assert!(table:: contains(&table, b"goodbye"), 0);

        // remove the value and check it
        assert!(table::remove(&mut table, b"hello") == 1, 0);
        assert!(table::remove(&mut table, b"goodbye") == 2, 0);
        ts::end(scenario);
        table::destroy_empty(table);
    }

    #[test]
    public fun test_table_utils_test() {

    }
}