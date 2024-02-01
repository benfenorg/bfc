module polynet::acl {
    use polynet::acl;

    //todo: implement ACL access control list
    struct Access_control_list has store{

    }

    public fun empty(): acl::Access_control_list{
        Access_control_list{}
    }

    public fun contains(acl: &Access_control_list, addr: address): bool{

        return false
    }
    public fun add(acl: &Access_control_list, addr: address){

    }
    public fun remove(acl: &Access_control_list, addr: address){

    }
}
