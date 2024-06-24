// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ampli } from '_src/shared/analytics/ampli';
import { formatAddress, sui2BfcAddress } from '@benfen/bfc.js/utils';
import { Popover, Transition } from '@headlessui/react';
import { useResolveSuiNSName } from '@mysten/core';
import { ChevronDown12, Copy12 } from '@mysten/icons';

import { useAccounts } from '../hooks/useAccounts';
import { useActiveAddress } from '../hooks/useActiveAddress';
import { useBackgroundClient } from '../hooks/useBackgroundClient';
import { useCopyToClipboard } from '../hooks/useCopyToClipboard';
import { ButtonConnectedTo } from '../shared/ButtonConnectedTo';
import { Text } from '../shared/text';
import { AccountList } from './AccountList';

export function AccountSelector() {
	const allAccounts = useAccounts();
	const activeAddress = useActiveAddress();
	const copyToAddress = useCopyToClipboard(activeAddress ? sui2BfcAddress(activeAddress) : '', {
		copySuccessMessage: 'Address copied',
	});
	const backgroundClient = useBackgroundClient();
	const { data: domainName } = useResolveSuiNSName(activeAddress);
	if (!allAccounts.length) {
		return null;
	}

	const buttonText = (
		<Text mono variant="bodySmall" truncate>
			{domainName ?? (activeAddress ? formatAddress(activeAddress) : '')}
		</Text>
	);

	if (allAccounts.length === 1) {
		return (
			<div
				className="flex items-center justify-center gap-1.25 rounded-[20px] border border-solid border-bfc-border py-1.25 px-2.5 cursor-pointer"
				onClick={copyToAddress}
			>
				{buttonText}
				<Copy12 data-testid="copy-address" />
			</div>
		);
	}
	return (
		<Popover className="relative z-10 max-w-full px-5">
			{({ close }) => (
				<>
					<Popover.Button as={ButtonConnectedTo} text={buttonText} iconAfter={<ChevronDown12 />} />
					<Transition
						enter="transition duration-200 ease-out"
						enterFrom="transform scale-95 opacity-0"
						enterTo="transform scale-100 opacity-100"
						leave="transition duration-200 ease-out"
						leaveFrom="transform scale-100 opacity-100"
						leaveTo="transform scale-75 opacity-0"
					>
						<Popover.Panel className="absolute left-1/2 -translate-x-1/2 w-[240px] mt-2 z-0 rounded-xl bg-white border border-solid border-bfc-border shadow-[0px_16px_24px_0px_rgba(0_0_0_0.08)]">
							<div className="absolute w-3 h-3 bg-white -top-1 left-1/2 -translate-x-1/2 rotate-45" />
							<div className="relative max-h-80 overflow-y-auto max-w-full z-10">
								<AccountList
									onAccountSelected={async ({ address, type }) => {
										if (address !== activeAddress) {
											ampli.switchedAccount({
												toAccountType: type,
											});
											await backgroundClient.selectAccount(address);
										}
										close();
									}}
								/>
							</div>
						</Popover.Panel>
					</Transition>
				</>
			)}
		</Popover>
	);
}
