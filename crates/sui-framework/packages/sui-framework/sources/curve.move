module sui::curve {
    /// Calculate D(x) = 5 * e ^ (-10 * numerator / denominator) Base 2 ** 64
    public native fun curve_dx(numerator: u128, denominator: u128): u128;

    // spec curve_dx {
    //     pragma opaque;
    //     // TODO: stub to be replaced by actual abort conditions if any
    //     aborts_if [abstract] true;
    //     // TODO: specify actual function behavior
    // }
}