// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatAddress } from '@benfen/bfc.js/utils';
import { CheckFill16, XFill16 } from '@mysten/icons';
import { cva, cx, type VariantProps } from 'class-variance-authority';
import { useEffect, useRef } from 'react';

import { Text } from '../shared/text';

const styles = cva('transition flex flex-row flex-nowrap items-center gap-3 py-2 cursor-pointer', {
	variants: {
		selected: {
			true: '',
			false: '',
		},
		mode: {
			select: '',
			disconnect: '',
		},
		disabled: {
			true: '',
			false: '',
		},
	},
	compoundVariants: [
		{ mode: 'select', selected: true, className: 'text-steel-darker' },
		{ mode: 'select', selected: false, className: 'text-steel-dark' },
		{
			mode: 'disconnect',
			selected: true,
			className: 'text-issue-dark',
		},
		{
			mode: 'disconnect',
			selected: false,
			className: 'text-steel-darker',
		},
	],
});
type StyleProps = VariantProps<typeof styles>;
export interface WalletListSelectItemProps extends Omit<StyleProps, 'mode' | 'selected'> {
	selected: NonNullable<StyleProps['selected']>;
	mode: NonNullable<StyleProps['mode']>;
	address: string;
	isNew?: boolean;
}

export function WalletListSelectItem({
	address,
	selected,
	mode,
	disabled = false,
	isNew = false,
}: WalletListSelectItemProps) {
	const elementRef = useRef<HTMLDivElement>(null);
	useEffect(() => {
		const timeout = setTimeout(() => {
			if (elementRef.current && isNew) {
				elementRef.current.scrollIntoView({
					behavior: 'smooth',
					block: 'center',
				});
			}
		}, 80);
		return () => {
			clearTimeout(timeout);
		};
	}, [isNew]);

	const isDisconnect = mode === 'disconnect';
	const isSelect = mode === 'select';

	return (
		<div ref={elementRef} className={styles({ selected, mode, disabled })}>
			{isSelect ? (
				<CheckFill16
					className={cx(
						selected ? 'text-bfc-text1' : 'text-bfc-text3',
						'transition text-body font-bold',
					)}
				/>
			) : null}
			{isDisconnect && selected ? <XFill16 className="text-bfc-red text-base font-bold" /> : null}
			<Text mono variant="body" weight="medium">
				{formatAddress(address)}
			</Text>
			{isDisconnect && !selected ? (
				<div className="flex flex-1 justify-end text-bfc-red">
					<Text variant="subtitle" weight="normal">
						Disconnect
					</Text>
				</div>
			) : null}
			{isSelect && isNew ? (
				<div className="flex-1 flex justify-end">
					<Text variant="body" color="bfc-text2">
						NEW
					</Text>
				</div>
			) : null}
		</div>
	);
}
