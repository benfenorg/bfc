# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# Default output format is JSON

bfc client switch --env devnet
bfc move summary --package-id 0x2 --bytecode
ls -1 package_summaries
ls -1 package_summaries/0x0000000000000000000000000000000000000000000000000000000000000002
ls -1 package_summaries/0x0000000000000000000000000000000000000000000000000000000000000001
cat package_summaries/root_package_metadata.json
echo
cat package_summaries/address_mapping.json
bfc client switch --env localnet