module 0x42::M {
    // visibility modifiers on structs fail during parsing
    struct Foo {}
    public(package) struct Foo {}
    public(package) struct Foo {}
}
