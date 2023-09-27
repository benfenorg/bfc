// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { EyeClose16, EyeOpen16 } from '@mysten/icons';
import { useField } from 'formik';
import { type ComponentProps, useState } from 'react';

export interface PasswordInputProps
	extends Omit<ComponentProps<'input'>, 'className' | 'type' | 'name'> {
	name: string;
}

export function PasswordInputField({ ...props }: PasswordInputProps) {
	const [passwordShown, setPasswordShown] = useState(false);
	const [field] = useField(props.name);
	const IconComponent = passwordShown ? EyeOpen16 : EyeClose16;
	return (
		<div className="flex w-full relative items-center">
			<input
				type={passwordShown ? 'text' : 'password'}
				placeholder="Password"
				{...props}
				{...field}
				className={
					'peer h-10 w-full text-bodySmall text-obc-text1 flex items-center gap-5 bg-obc-card px-2.5 border border-solid  border-obc-border rounded-lg focus:bg-transparent focus:border-obc-text1 placeholder-obc-text3'
				}
			/>
			<IconComponent
				className="absolute text-heading6 font-normal text-obc-text1 cursor-pointer right-3"
				onClick={() => setPasswordShown(!passwordShown)}
			/>
		</div>
	);
}
