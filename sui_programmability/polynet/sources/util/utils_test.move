module polynet::utils_test {

    #[test_only]
    use std::debug::print;
    #[test_only]
    use polynet::utils::{is_admin, get_default_admin_address, to_address};
    #[test_only]
    use sui::address::to_bytes;

    #[test]
    public fun test_utils_fun(){
        let add = @0x01;
        let result =  is_admin(add);
        assert!(result == false, 1);

        let default_admin = get_default_admin_address();
        let result = is_admin(default_admin);
        assert!(result == true, 2);

        let u8_vec = to_bytes(default_admin);
        print(&u8_vec);

        let add_format = to_address(u8_vec);
        print(&add_format);

    }


    #[test]
    public fun test_table_utils_test() {

    }

}