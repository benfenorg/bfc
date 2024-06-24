// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import NumberInput from '_components/number-input';
import { cva, type VariantProps } from 'class-variance-authority';
import cl from 'classnames';
import { useField, useFormikContext } from 'formik';
import type { ComponentProps } from 'react';

import Alert from '../components/alert';
import { Pill, type PillProps } from './Pill';

const styles = cva(
	[
		'transition flex flex-row items-center h-10 w-full text-bodySmall text-bfc-text1 gap-5 bg-bfc-card px-2.5 border border-solid border-bfc-border rounded-lg',
		'placeholder-bfc-text3 w-full pr-[calc(20%_+_24px)]',
		'focus:bg-transparent focus:border-bfc-text1',
		'disabled:bg-white disabled:text-bfc-text2',
	],
	{
		variants: {
			rounded: {
				lg: 'rounded-2lg',
				md: 'rounded-md',
			},
			// TODO: handle dark outline Pill
			dark: {
				true: '',
				false: '',
			},
		},
		defaultVariants: {
			rounded: 'lg',
			dark: false,
		},
	},
);

export type InputWithActionProps = VariantProps<typeof styles> &
	(
		| (Omit<ComponentProps<'input'>, 'className' | 'type'> & {
				type?: 'text' | 'number' | 'password' | 'email';
		  })
		| (Omit<ComponentProps<typeof NumberInput>, 'form' | 'field' | 'meta'> & {
				type: 'numberInput';
		  })
	) & {
		actionText: string;
		actionClass?: string;
		onActionClicked?: PillProps['onClick'];
		actionType?: PillProps['type'];
		name: string;
		actionDisabled?: boolean | 'auto';
	};

export function InputWithAction({
	actionText,
	actionClass,
	onActionClicked,
	actionType = 'submit',
	type,
	disabled = false,
	actionDisabled = false,
	name,
	dark,
	rounded,
	...props
}: InputWithActionProps) {
	const [field, meta] = useField(name);
	const form = useFormikContext();
	const { isSubmitting } = form;
	const isInputDisabled = isSubmitting || disabled;
	const isActionDisabled =
		actionDisabled === 'auto'
			? isInputDisabled || meta?.initialValue === meta?.value || !!meta?.error
			: actionDisabled;

	return (
		<>
			<div className="flex flex-row flex-nowrap items-center relative">
				{type === 'numberInput' ? (
					<NumberInput
						className={styles({ rounded })}
						allowNegative
						{...props}
						form={form}
						field={field}
						meta={meta}
						disabled={isInputDisabled}
					/>
				) : (
					<input
						type={type}
						disabled={isInputDisabled}
						{...field}
						{...props}
						className={styles({ rounded })}
					/>
				)}
				<div
					className={cl(
						'flex items-center justify-end absolute right-0 max-w-[20%] mx-2.5 overflow-hidden',
						actionClass,
					)}
				>
					<Pill
						text={actionText}
						type={actionType}
						disabled={isActionDisabled}
						loading={isSubmitting}
						onClick={onActionClicked}
						dark={dark}
					/>
				</div>
			</div>

			{(meta?.touched && meta?.error) || (meta.value !== '' && meta.error) ? (
				<div className="mt-3">
					<Alert>{meta?.error}</Alert>
				</div>
			) : null}
		</>
	);
}
