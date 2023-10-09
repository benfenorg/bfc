// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useNavigate } from 'react-router-dom';
import { SuiConnectButton } from './SuiConnectButton';
import { ReactComponent as BenfenHeaderLogo } from '../../assets/benfen_header_logo.svg';

export function Header() {
	const navigate = useNavigate();


	return (
		<div className="border-b border-gray-400">
			<div>
				<BenfenHeaderLogo />
				{/* <button
					className="text-lg font-bold text-center mr-3 bg-transparent ease-in-out duration-300 rounded border border-transparent py-2 px-4 bg-gray-200"
					onClick={() => navigate('/')}
				>
					Station demo
				</button> */}
				
				<div className="ml-auto my-3 md:my-1">
					<SuiConnectButton></SuiConnectButton>
				</div>
			</div>
		</div>
	);
}
