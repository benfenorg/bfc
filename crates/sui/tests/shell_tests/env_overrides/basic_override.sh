# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

bfc client --client.config config.yaml switch --env base

bfc client --client.config config.yaml envs
bfc client --client.config config.yaml --client.env one envs
bfc client --client.config config.yaml --client.env two envs

bfc client --client.config config.yaml active-env
bfc client --client.config config.yaml --client.env one active-env
bfc client --client.config config.yaml --client.env two active-env

# Unknown name -- Should give you None and nothing active
bfc client --client.config config.yaml --client.env not_an_env envs
bfc client --client.config config.yaml --client.env not_an_env active-env
