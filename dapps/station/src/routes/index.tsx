// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { createBrowserRouter } from 'react-router-dom';

import Root from '../Root';
import Station from '../pages/station/Station';

export const router = createBrowserRouter([
	{
		path: '/',
		element: <Root />,
		children: [
			{
				path: '/',
				element: <Station />,
			},
			{
				path: '/swap',
				element: <div>swap</div>,
			},
			{
				path: '/pools',
				element:  <div>pools</div>,
			},
		],
	},
]);
