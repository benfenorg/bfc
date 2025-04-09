// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { BackgroundClient } from '_app/background-client';
import type { RootState } from '_redux/RootReducer';
import type { AppDispatch } from '_store';

export const thunkExtras = {
	background: new BackgroundClient(),
};

type ThunkExtras = typeof thunkExtras;

export interface AppThunkConfig {
	extra: ThunkExtras;
	state: RootState;
	dispatch: AppDispatch;
}
