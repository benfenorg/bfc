// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { createBrowserRouter } from 'react-router-dom';

import Root from '../Root';
import Home from './Home';
import SingleKiosk from './SingleKiosk';
import Station from '../pages/station/Station';

export const router = createBrowserRouter([
	{
		path: '/',
		element: <Root />,
		children: [
			{
				path: '',
				element: <Station />,
			},
			{
				path: '/kiosk/:id',
				element: <SingleKiosk />,
			},
		],
	},
]);
