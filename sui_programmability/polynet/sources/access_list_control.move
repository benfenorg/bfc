module polynet::acl {
    use polynet::acl;

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
