module polynet::utils_test {

    #[test_only]
    use polynet::utils::{is_admin, get_default_admin_address};

    #[test]
    public fun test_utils_fun(){
        let add = @0x01;
        let result =  is_admin(add);
        assert!(result == false, 1);

        let default_admin = get_default_admin_address();
        let result = is_admin(default_admin);
        assert!(result == true, 2);
    }


    #[test]
    public fun test_table_utils_test() {

    }

}