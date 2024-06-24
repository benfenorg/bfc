// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { AppThunkConfig } from '_redux/store/thunk-extras';
import { createAsyncThunk } from '@reduxjs/toolkit';

export const unlockWallet = createAsyncThunk<void, { password: string }, AppThunkConfig>(
	'wallet-unlock-wallet',
	async ({ password }, { extra: { background } }) => {
		await background.unlockWallet(password);
	},
);

export const lockWallet = createAsyncThunk<void, void, AppThunkConfig>(
	'wallet-lock-wallet',
	async (_, { extra: { background } }) => {
		await background.lockWallet();
	},
);

export const setKeyringLockTimeout = createAsyncThunk<void, { timeout: number }, AppThunkConfig>(
	'wallet-set-keyring-lock-timeout',
	async ({ timeout }, { extra: { background } }) => {
		await background.setKeyringLockTimeout(timeout);
	},
);
