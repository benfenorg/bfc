// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { API_ENV_TO_INFO } from '_app/ApiProvider';
import { useNextMenuUrl } from '_components/menu/hooks';
import { useAppSelector } from '_hooks';
import { FAQ_LINK, ToS_LINK } from '_src/shared/constants';
import FaucetRequestButton from '_src/ui/app/shared/faucet/FaucetRequestButton';
import { Link } from '_src/ui/app/shared/Link';
import { Text } from '_src/ui/app/shared/text';
import {
	ArrowUpRight12,
	Clipboard16 as ClipboardIcon,
	Domain24,
	LockLocked16 as LockedLockIcon,
	More24 as MoreIcon,
} from '@mysten/icons';
import Browser from 'webextension-polyfill';

import { MenuLayout } from './MenuLayout';
import MenuListItem from './MenuListItem';

function MenuList() {
	const networkUrl = useNextMenuUrl(true, '/network');
	const passwordProtectUrl = useNextMenuUrl(true, '/password-protect');

	const moreOptionsUrl = useNextMenuUrl(true, '/more-options');

	const apiEnv = useAppSelector((state) => state.app.apiEnv);
	const networkName = API_ENV_TO_INFO[apiEnv].name;
	const version = Browser.runtime.getManifest().version;

	return (
		<>
			<MenuLayout title="Wallet Settings">
				<div className="flex flex-col divide-y divide-x-0 divide-solid divide-gray-45">
					<MenuListItem
						to={networkUrl}
						icon={<Domain24 />}
						title="Network"
						subtitle={networkName}
					/>
					<MenuListItem
						to={passwordProtectUrl}
						icon={<LockedLockIcon />}
						title={'Password Protect Accounts'}
					/>
					<MenuListItem icon={<ClipboardIcon />} title="FAQ" href={FAQ_LINK} />
					<MenuListItem
						icon={<MoreIcon className="text-steel-darker" />}
						title="More options"
						to={moreOptionsUrl}
					/>
				</div>
				<div className="flex-1" />
				<div className="flex flex-col items-stretch mt-2.5">
					<FaucetRequestButton variant="outline" />
				</div>
				<div className="px-2.5 flex flex-col items-center justify-center no-underline gap-3.75 mt-3.75">
					<Link
						href={ToS_LINK}
						text="Terms of service"
						after={<ArrowUpRight12 />}
						color="steelDark"
						weight="semibold"
					/>
					<Text variant="bodySmall" weight="medium" color="steel">
						On Sui Wallet version v{version}
					</Text>
				</div>
			</MenuLayout>
		</>
	);
}

export default MenuList;
