address 0x2 {

module X {
    public fun f_public() {}
}

module Y {
    friend 0x2::M;
    public(package) fun f_friend() {}
}

module M {
    use 0x2::X;
    use 0x2::Y;

    public fun f_public() {}
    public(package) fun f_friend() {}
    fun f_private() {}

    // a public(package) fun can call public funs in another module
    public(package) fun f_friend_call_public() { X::f_public() }

    // a public(package) fun can call private and public funs defined in its own module
    public(package) fun f_friend_call_self_private() { Self::f_private() }
    public(package) fun f_friend_call_self_public() { Self::f_public() }

    // a public functions can call a public(package) function defined in the same module
    // as well as friend functions defined in other modules (subject to friend list)
    public fun f_public_call_friend() { Y::f_friend() }
    public fun f_public_call_self_friend() { Self::f_friend() }

    // a public(package) functions can call a public(package) function defined in the same module
    // as well as friend functions defined in other modules (subject to friend list)
    public(package) fun f_friend_call_friend() { Y::f_friend() }
    public(package) fun f_friend_call_self_friend() { Self::f_friend() }

    // a private functions can call a public(package) function defined in the same module
    // as well as friend functions defined in other modules (subject to friend list)
    fun f_private_call_friend() { Y::f_friend() }
    fun f_private_call_self_friend() { Self::f_friend() }
}

}
