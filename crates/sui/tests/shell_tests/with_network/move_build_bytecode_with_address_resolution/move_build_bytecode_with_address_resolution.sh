# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

bfc client --client.config $CONFIG \
  publish simple --verify-deps \
  --json | jq '.effects.status'

bfc move --client.config $CONFIG \
  build --path depends_on_simple
