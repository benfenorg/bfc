// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/**
 * This is an App UI Component, which is responsible for network selection.
 * It's as context un-aware as it reasonably can be, being a controlled component.
 * In the future, this should move outside of the base `~/ui/` directory, but for
 * now we are including App UI Components in the base UI component directory.
 */

import { autoUpdate, flip, FloatingPortal, offset, shift, useFloating } from '@floating-ui/react';
import { Popover } from '@headlessui/react';
import { useZodForm } from '@mysten/core';
import { TriangleDown14 } from '@mysten/icons';
import clsx from 'clsx';
import { AnimatePresence, motion } from 'framer-motion';
import { useEffect, useState } from 'react';
import { z } from 'zod';

import { NavItem } from './NavItem';
import { ReactComponent as ChevronDownIcon } from '../icons/chevron_down.svg';
import { ReactComponent as MenuIcon } from '../icons/menu.svg';

import type { ComponentProps, ReactNode } from 'react';

export interface NetworkOption {
	id: string;
	label: string;
}

export interface NetworkSelectProps {
	networks: NetworkOption[];
	value: string;
	version?: number | string;
	binaryVersion?: string;
	isDarker?: boolean;
	onChange(networkId: string): void;
}

enum NetworkState {
	UNSELECTED = 'UNSELECTED',
	PENDING = 'PENDING',
	SELECTED = 'SELECTED',
}

interface SelectableNetworkProps extends ComponentProps<'div'> {
	state: NetworkState;
	children: ReactNode;
	onClick(): void;
}

function SelectableNetwork({ state, children, onClick, ...props }: SelectableNetworkProps) {
	return (
		<div
			role="button"
			onClick={onClick}
			className={clsx(
				'flex items-center rounded px-1.5 py-[7px] text-body/[18px] font-normal text-bfc-text2 hover:bg-bfc-card ui-active:bg-bfc-card',
				state !== NetworkState.UNSELECTED ? 'text-bfc-text1' : 'text-bfc-text2',
			)}
			{...props}
		>
			<div
				className={clsx(
					'grow text-body/[18px]',
					state === NetworkState.SELECTED ? 'text-bfc-text1' : 'text-bfc-text2',
				)}
			>
				{children}
			</div>
			{state === NetworkState.SELECTED && (
				<svg
					className="ml-1"
					width="14"
					height="14"
					viewBox="0 0 14 14"
					fill="none"
					xmlns="http://www.w3.org/2000/svg"
				>
					<g clipPath="url(#clip0_3216_2411)">
						<path
							d="M5.83343 8.85041L11.1954 3.48782L12.0208 4.31266L5.83343 10.5001L2.12109 6.78774L2.94593 5.96291L5.83343 8.85041Z"
							fill="#171719"
						/>
					</g>
					<defs>
						<clipPath id="clip0_3216_2411">
							<rect width="14" height="14" fill="white" />
						</clipPath>
					</defs>
				</svg>
			)}
		</div>
	);
}

const CustomRPCSchema = z.object({
	url: z.string().url(),
});

function CustomRPCInput({
	value,
	onChange,
}: {
	value: string;
	onChange(networkUrl: string): void;
}) {
	const { register, handleSubmit, formState } = useZodForm({
		schema: CustomRPCSchema,
		mode: 'all',
		defaultValues: {
			url: value,
		},
	});

	const { errors, isDirty, isValid } = formState;

	return (
		<form
			onSubmit={handleSubmit((values) => {
				onChange(values.url);
			})}
			className="relative flex items-center rounded-md"
		>
			<input
				{...register('url')}
				type="text"
				className={clsx(
					'block w-full rounded-md border p-3 pr-16 shadow-sm outline-none',
					errors.url ? 'border-issue-dark text-issue-dark' : 'border-gray-65 text-gray-90',
				)}
			/>

			<div className="absolute inset-y-0 right-0 flex flex-col justify-center px-3">
				<button
					disabled={!isDirty || !isValid}
					type="submit"
					className="flex items-center justify-center rounded-full bg-gray-90 px-2 py-1 text-captionSmall font-semibold uppercase text-white transition disabled:bg-gray-45 disabled:text-gray-65"
				>
					Save
				</button>
			</div>
		</form>
	);
}

function NetworkVersion({
	label,
	version,
	binaryVersion,
}: {
	label: string;
	version: number | string;
	binaryVersion: string;
}) {
	return (
		<div className="flex flex-col justify-between border-t border-bfc-border px-1.5 py-[7px] text-body/[18px] text-bfc-text3">
			BFC {label}
			<br />v{binaryVersion} (Protocol {version})
		</div>
	);
}

function NetworkSelectPanel({ networks, onChange, value }: Omit<NetworkSelectProps, 'version'>) {
	const isCustomNetwork = !networks.find(({ id }) => id === value);
	const [customOpen, setCustomOpen] = useState(isCustomNetwork);

	useEffect(() => {
		setCustomOpen(isCustomNetwork);
	}, [isCustomNetwork]);

	return (
		<>
			{networks.map((network) => (
				<SelectableNetwork
					key={network.id}
					state={
						!customOpen && value === network.id ? NetworkState.SELECTED : NetworkState.UNSELECTED
					}
					onClick={() => {
						onChange(network.id);
					}}
				>
					{network.label}
				</SelectableNetwork>
			))}

			<SelectableNetwork
				state={
					isCustomNetwork
						? NetworkState.SELECTED
						: customOpen
						? NetworkState.PENDING
						: NetworkState.UNSELECTED
				}
				onClick={() => setCustomOpen(true)}
			>
				Custom RPC URL
				{customOpen && (
					<div className="mt-3">
						<CustomRPCInput value={isCustomNetwork ? value : ''} onChange={onChange} />
					</div>
				)}
			</SelectableNetwork>
		</>
	);
}

function ResponsiveIcon() {
	return (
		<div>
			<ChevronDownIcon className="hidden md:block" />
			<MenuIcon className="block md:hidden" />
		</div>
	);
}

export function NetworkSelect({
	networks,
	value,
	version,
	binaryVersion,
	onChange,
	isDarker,
}: NetworkSelectProps) {
	const { x, y, refs, strategy } = useFloating({
		placement: 'bottom-end',
		middleware: [offset(5), flip(), shift()],
		whileElementsMounted: autoUpdate,
	});

	const selected = networks.find(({ id }) => id === value);

	return (
		<Popover>
			{({ open, close }) => (
				<>
					<Popover.Button ref={refs.setReference} className="w-full">
						<div className="flex w-full items-center justify-center gap-1 rounded-lg bg-bfc-card p-2.5 text-bodyLarge/[20px] font-semibold text-bfc md:hidden">
							{selected?.label || 'Custom'}
							<TriangleDown14 className={clsx(open ? 'rotate-180' : '')} />
						</div>
						<NavItem isDarker={isDarker} className="hidden md:flex">
							<div className="flex items-center gap-1">
								{selected?.label || 'Custom'}
								<ResponsiveIcon />
							</div>
						</NavItem>
					</Popover.Button>
					<FloatingPortal>
						<AnimatePresence>
							{open && (
								<Popover.Panel
									static
									ref={refs.setFloating}
									as={motion.div}
									initial={{
										opacity: 0,
										scale: 0.95,
									}}
									animate={{
										opacity: 1,
										scale: 1,
									}}
									exit={{
										opacity: 0,
										scale: 0.95,
									}}
									transition={{ duration: 0.15 }}
									className="z-20 flex w-[calc(100%-40px)] flex-col rounded-lg border border-bfc-border bg-white p-1 shadow-[0px_16px_16px_0px_rgba(20,21,24,0.05)] md:w-auto"
									style={{
										position: strategy,
										top: y ?? 0,
										left: x ?? 0,
									}}
								>
									<NetworkSelectPanel
										networks={networks}
										value={value}
										onChange={(network) => {
											onChange(network);
											close();
										}}
									/>
									{!!value && version && binaryVersion ? (
										<NetworkVersion
											label={selected?.label ?? 'Custom RPC'}
											binaryVersion={binaryVersion}
											version={version}
										/>
									) : null}
								</Popover.Panel>
							)}
						</AnimatePresence>
					</FloatingPortal>
				</>
			)}
		</Popover>
	);
}
