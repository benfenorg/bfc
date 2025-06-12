module sui::hfe_ops;


public native fun hfe_ops_add(
    input_1: u64,
    input_2: u64,
    input_3: u64,
    input_4: u64,
): vector<u64>;

public native fun hfe_ops_minus(
    input_1: u64,
    input_2: u64,
    input_3: u64,
    input_4: u64,
): vector<u64>;
public native fun hfe_ops_multiplied(
    input_1: u64,
    input_2: u64,
    input_3: u64,
    input_4: u64,
): vector<u64>;
public native fun hfe_ops_compare(
    input_1: u64,
    input_2: u64,
    input_3: u64,
    input_4: u64,
): u64;

public native fun split_data(
    data: &vector<u8>,
    n: u8,
    index: u8,
): vector<u8>;

public native fun hfe_ops_split_value(
    value: u64,
): vector<u64>;

public native fun hfe_ops_restore_value(
    value1: u64,
    value2: u64,
): u64;