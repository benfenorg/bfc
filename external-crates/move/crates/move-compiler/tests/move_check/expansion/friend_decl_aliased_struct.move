address 0x42 {
module A {
    struct A {}
}

module M {
    use 0x42::A::A;
    friend A;

    public(package) fun m(_a: A) {}
}
}
