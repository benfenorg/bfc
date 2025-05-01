module sui::hfe_ops;


public native fun hfe_ops_add(
    input_1: u8,
    input_2: u8,
): u8;

public native fun split_data(
    data: &vector<u8>,
    n: u8,
    index: u8,
): vector<u8>;

