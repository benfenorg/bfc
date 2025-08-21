# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# tests that sui move new followed by sui move build succeeds

bfc move new example


cd example && bfc move build
