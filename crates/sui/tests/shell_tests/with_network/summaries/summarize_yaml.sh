# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
bfc client switch --env devnet
bfc move  summary --package-id 0x2 -o yaml --bytecode
ls -1 package_summaries
ls -1 package_summaries/0x0000000000000000000000000000000000000000000000000000000000000002
ls -1 package_summaries/0x0000000000000000000000000000000000000000000000000000000000000001
cat package_summaries/root_package_metadata.yaml
cat package_summaries/address_mapping.yaml
bfc client switch --env localnet
