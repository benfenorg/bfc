// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CheckFill12 } from '@mysten/icons';

import { Text } from '../shared/text';

import type { PermissionType } from '_messages/payloads/permissions';

export type DAppPermissionsListProps = {
	permissions: PermissionType[];
};

const permissionTypeToTxt: Record<PermissionType, string> = {
	viewAccount: 'Share wallet address',
	suggestTransactions: 'Suggest transactions to approve',
};

export function DAppPermissionsList({ permissions }: DAppPermissionsListProps) {
	return (
		<ul className="py-2.5 list-none m-0 p-0 flex flex-col gap-2.5">
			{permissions.map((aPermission) => (
				<li key={aPermission} className="flex flex-row flex-nowrap items-center gap-2">
					<CheckFill12 className="text-bfc-text2" />
					<Text variant="body" weight="medium" color="bfc-text1">
						{permissionTypeToTxt[aPermission]}
					</Text>
				</li>
			))}
		</ul>
	);
}
