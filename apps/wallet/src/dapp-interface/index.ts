// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { registerWallet } from '@benfen/wallet-standard';

import { SuiWallet } from './WalletStandardInterface';

registerWallet(new SuiWallet());
