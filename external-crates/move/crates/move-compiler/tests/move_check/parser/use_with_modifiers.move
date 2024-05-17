#[allow(unused)]
module a::m {
    public use a::m as m1;
    public(package) use a::m as m2;
    entry use a::m as m3;
    native use a::m as m4;
    public native entry use a::m as m5;
}
