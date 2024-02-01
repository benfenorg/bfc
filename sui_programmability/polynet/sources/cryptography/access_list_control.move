module polynet::acl {
    use polynet::acl;

    //todo: implement ACL access control list
    struct ACL{

    }

    public fun empty(): acl::ACL{
       ACL{}
    }

    public fun contains(acl: &ACL, addr: address): bool{

        return false
    }
    public fun add(acl: &ACL, addr: address){

    }
    public fun remove(acl: &ACL, addr: address){

    }
}
