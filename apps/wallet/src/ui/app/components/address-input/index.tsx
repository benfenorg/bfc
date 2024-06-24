// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_app/shared/text';
import Alert from '_src/ui/app/components/alert';
import { useSuiClient } from '@benfen/bfc.js/dapp-kit';
import { isValidSuiAddress } from '@benfen/bfc.js/utils';
import { QrCode, X12 } from '@mysten/icons';
import { useQuery } from '@tanstack/react-query';
import { cx } from 'class-variance-authority';
import { useField, useFormikContext } from 'formik';
import { useCallback, useMemo } from 'react';
import type { ChangeEventHandler } from 'react';
import TextareaAutosize from 'react-textarea-autosize';

import { useSuiAddressValidation } from './validation';

export interface AddressInputProps {
	disabled?: boolean;
	placeholder?: string;
	name: string;
}

enum RecipientWarningType {
	OBJECT = 'OBJECT',
	EMPTY = 'EMPTY',
}

export function AddressInput({
	disabled: forcedDisabled,
	placeholder = '0x...',
	name = 'to',
}: AddressInputProps) {
	const [field, meta] = useField(name);

	const rpc = useSuiClient();
	const { data: warningData } = useQuery({
		queryKey: ['address-input-warning', field.value],
		queryFn: async () => {
			// We assume this validation will happen elsewhere:
			if (!isValidSuiAddress(field.value)) {
				return null;
			}

			const object = await rpc.getObject({ id: field.value });

			if (object && 'data' in object) {
				return RecipientWarningType.OBJECT;
			}

			const [fromAddr, toAddr] = await Promise.all([
				rpc.queryTransactionBlocks({
					filter: { FromAddress: field.value },
					limit: 1,
				}),
				rpc.queryTransactionBlocks({
					filter: { ToAddress: field.value },
					limit: 1,
				}),
			]);

			if (fromAddr.data?.length === 0 && toAddr.data?.length === 0) {
				return RecipientWarningType.EMPTY;
			}

			return null;
		},
		enabled: !!field.value,
		gcTime: 10 * 1000,
		refetchOnMount: false,
		refetchOnWindowFocus: false,
		refetchInterval: false,
	});

	const { isSubmitting, setFieldValue } = useFormikContext();
	const suiAddressValidation = useSuiAddressValidation();

	const disabled = forcedDisabled !== undefined ? forcedDisabled : isSubmitting;
	const handleOnChange = useCallback<ChangeEventHandler<HTMLTextAreaElement>>(
		(e) => {
			const address = e.currentTarget.value;
			setFieldValue(name, suiAddressValidation.cast(address));
		},
		[setFieldValue, name, suiAddressValidation],
	);
	const formattedValue = useMemo(
		() => suiAddressValidation.cast(field?.value),
		[field?.value, suiAddressValidation],
	);

	const clearAddress = useCallback(() => {
		setFieldValue('to', '');
	}, [setFieldValue]);

	return (
		<>
			<div
				className={cx(
					'flex h-max w-full overflow-hidden border border-solid border-bfc-border rounded-lg bg-bfc-card text-bfc-text3 focus-within:bg-white focus-within:border-bfc-text1',
				)}
			>
				<div className="h-10 w-full flex items-center pl-2.5">
					<TextareaAutosize
						data-testid="address-input"
						maxRows={3}
						minRows={1}
						disabled={disabled}
						placeholder={placeholder}
						value={formattedValue}
						onChange={handleOnChange}
						onBlur={field.onBlur}
						className={cx(
							'address bg-transparent w-full text-body/[18px] leading-100 font-normal placeholder:text-bfc-text3 border-none resize-none focus:text-bfc',
						)}
						name={name}
					/>
				</div>

				<div
					onClick={clearAddress}
					className="flex bg-bfc-border items-center justify-center w-11 right-0 ml-1.25 cursor-pointer"
				>
					{meta.touched && field.value ? (
						<X12 className="h-3 w-3 text-bfc" />
					) : (
						<QrCode className="h-5 w-5 text-bfc-text3" />
					)}
				</div>
			</div>

			{meta.touched ? (
				<div className="mt-2.5 w-full">
					<Alert noBorder rounded="lg" mode={meta.error || warningData ? 'issue' : 'success'}>
						{warningData === RecipientWarningType.OBJECT ? (
							<>
								<Text variant="body" weight="normal">
									This address is an Object
								</Text>
								<Text variant="body" weight="normal">
									Once sent, the funds cannot be recovered. Please make sure you want to send coins
									to this address.
								</Text>
							</>
						) : warningData === RecipientWarningType.EMPTY ? (
							<>
								<Text variant="body" weight="normal">
									This address has no prior transactions
								</Text>
								<Text variant="body" weight="normal">
									Please make sure you want to send coins to this address.
								</Text>
							</>
						) : (
							<Text variant="body" weight="normal">
								{meta.error || 'Valid address'}
							</Text>
						)}
					</Alert>
				</div>
			) : null}
		</>
	);
}
