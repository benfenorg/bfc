module polynet::acl {
    use polynet::acl;
    use std::vector;

    /// The ACL already contains the address.
    const ECONTAIN: u64 = 4000;
    /// The ACL does not contain the address.
    const ENOT_CONTAIN: u64 = 4001;


    struct Access_control_list has store, drop, copy {
        list: vector<address>
    }

    public fun empty(): acl::Access_control_list{

        Access_control_list{ list: vector::empty<address>() }
    }


    public fun contains(acl: &mut Access_control_list, addr: address): bool{
        vector::contains(&acl.list, &addr)
    }
    public fun add(acl: &mut Access_control_list, addr: address){

        assert!(!vector::contains(&mut acl.list, &addr), (ECONTAIN));
        vector::push_back(&mut acl.list, addr);
    }
    public fun remove(acl: &mut Access_control_list, addr: address){
        let (found, index) = vector::index_of(&mut acl.list, &addr);
        assert!(found, (ENOT_CONTAIN));
        vector::remove(&mut acl.list, index);
    }

}
