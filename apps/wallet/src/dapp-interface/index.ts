// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { registerWallet } from '@benfen/bfc.js/wallet-standard';

import { SuiWallet } from './WalletStandardInterface';

registerWallet(new SuiWallet());
