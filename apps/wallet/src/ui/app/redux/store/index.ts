// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import rootReducer from '_redux/RootReducer';
import { configureStore } from '@reduxjs/toolkit';

import { thunkExtras } from './thunk-extras';
<<<<<<< HEAD
import { amplitudePersistenceMiddleware } from '../slices/account';
import rootReducer from '_redux/RootReducer';
=======
>>>>>>> mainnet-v1.24.1

const store = configureStore({
	reducer: rootReducer,
	middleware: (getDefaultMiddleware) =>
		getDefaultMiddleware({
			thunk: {
				extraArgument: thunkExtras,
			},
		}).prepend(amplitudePersistenceMiddleware.middleware),
});

export default store;

export type AppDispatch = typeof store.dispatch;
