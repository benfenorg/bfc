// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useNavigate } from 'react-router-dom';
import { ConnectButton } from '../ConnectButton';
import { ReactComponent as BenfenHeaderLogo } from '../../assets/benfen_header_logo.svg';
import {HeaderTab} from './HeaderTab'
export function Header() {
	const navigate = useNavigate();

	return (
		<div className="">
			<div className="flex px-10 py-5 items-center gap-7 border-b border-bf-hover">
				<div onClick={() => navigate('/')}>
					<BenfenHeaderLogo />
				</div>
				<div className="font-semibold text-xs">BenFen Station</div>
				<div className="ml-auto">
					<ConnectButton />
				</div>
			</div>
			<div className="flex px-10 pt-10 pb-5">
				<div className="text-xl font-bold">BenFen Station</div>
				
			</div>
			<div className="flex px-10">
				<HeaderTab />
			</div>
			
		</div>
	);
}
