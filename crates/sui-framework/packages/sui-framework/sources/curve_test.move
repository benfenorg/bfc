#[test_only]
module sui::curve_tests {
    use std::debug::print;
    use sui::curve::curve_dx;

    #[test]
    public fun test_curve_dx() {
        let ret = curve_dx(30000_000000000, 3000000000_000000000);
        print(&ret);
    }
}