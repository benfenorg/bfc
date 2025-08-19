# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that sui move new followed by sui move disassemble succeeds


bfc move new example
cat > example/sources/example.move <<EOF
module example::example;

public fun foo(_ctx: &mut TxContext) {}
EOF
cd example

echo "=== Build ===" | tee /dev/stderr
bfc move build

echo "=== Disassemble ===" | tee /dev/stderr
bfc move disassemble build/example/bytecode_modules/example.mv
