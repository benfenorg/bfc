// report unused mutable ref with immutable usage
module a::m {
    struct S has drop { f: u64 }

    public(package) fun param(x: &mut S): &S {
        let r: &S;
        freeze(x);
        let _: &S = x;
        let _: &u64 = &x.f;
        ignore(x);
        r = x;
        r;
        x
    }

    public(package) fun local(s: S) {
        let x = &mut s;
        let r: &S;
        freeze(x);
        let _: &S = x;
        let _: &u64 = &x.f;
        ignore(x);
        r = x;
        r;
    }

    fun ignore<T>(_: &T) {}
}
