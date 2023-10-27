// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { createBrowserRouter } from 'react-router-dom';

import Root from '../Root';
import PageContent from '../components/Base/PageContent'
import Station from '../pages/station/Station';
import SwapDexPage from '../pages/swap';
import TokenPage from '../pages/swap/tokenPage';

export const router = createBrowserRouter([
	{
		path: '/',
		element: <Root />,
		children: [
			{
				path: '/',
				element: <PageContent id="/"/>,
				children: [
					{
						path: '/',
						element: <Station />,
					},
				]
			},
			{
				path: '/swap',
				element: <PageContent id="/swap"/>,
				children: [
					{
						path: '/swap',
						element: <SwapDexPage />,
					},
					{
						path: '/swap/tokens',
						element: <TokenPage />,
					},
				],
			},
			{
				path: '/pool',
				element: <PageContent id="/pool"/>,
				children: [
					{
						path: '/pool',
						element: <div>pools</div>,
					},
				]
			},
		],
	},
]);
