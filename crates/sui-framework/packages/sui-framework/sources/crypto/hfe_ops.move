module sui::hfe_ops;


public native fun hfe_ops_add(
    input_1: vector<u8>,
    input_2: vector<u8>,
    input_3: vector<u8>,
    input_4: vector<u8>,
): (vector<u8>, vector<u8>);

public native fun hfe_ops_minus(
    input_1: vector<u8>,
    input_2: vector<u8>,
    input_3: vector<u8>,
    input_4: vector<u8>,
): (vector<u8>, vector<u8>);

public native fun hfe_ops_multiplied(
    input_1: vector<u8>,
    input_2: vector<u8>,
    input_3: vector<u8>,
    input_4: vector<u8>,
): (vector<u8>, vector<u8>);

public native fun hfe_ops_compare_value(
    input_1: vector<u8>,
    input_2: vector<u8>,
    input_3: u64,
): u8;

public native fun hfe_ops_encode_data(
    value: u64,
): (vector<u8>, vector<u8>);

public native fun hfe_ops_restore_value(
    value1: vector<u8>,
    value2: vector<u8>,
    signature: vector<u8>,
    id: address,
    publickey: vector<u8>,
): u64;