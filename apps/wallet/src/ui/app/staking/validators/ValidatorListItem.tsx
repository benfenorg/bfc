// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_app/shared/text';
import { cx } from 'class-variance-authority';
import { AnimatePresence } from 'framer-motion';

import { ValidatorLogo } from './ValidatorLogo';

type ValidatorListItemProp = {
	selected?: boolean;
	value: string | number;
	validatorAddress: string;
};
export function ValidatorListItem({ selected, value, validatorAddress }: ValidatorListItemProp) {
	return (
		<AnimatePresence>
			<div
				className={cx(
					selected ? 'bg-bfc-card' : '',
					'flex justify-between w-full hover:bg-bfc-card/50 p-2.5 rounded-lg group items-center',
				)}
				role="button"
			>
				<div className="relative flex items-center justify-start w-full">
					<ValidatorLogo
						validatorAddress={validatorAddress}
						showAddress
						iconSize="sm"
						size="body"
						showActiveStatus
					/>
				</div>
				<div className="flex gap-0.5 items-center">
					<div className="flex gap-0.5 leading-none">
						<Text variant="body" weight="medium" color="bfc-text1">
							{value}
						</Text>
					</div>
				</div>
			</div>
		</AnimatePresence>
	);
}
