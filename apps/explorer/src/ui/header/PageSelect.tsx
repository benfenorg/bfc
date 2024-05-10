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
import clsx from 'clsx';
import { AnimatePresence, motion } from 'framer-motion';

import { NavItem } from './NavItem';
import { ReactComponent as ChevronDownIcon } from '../icons/chevron_down.svg';
import { ReactComponent as MenuIcon } from '../icons/menu.svg';
import { Link } from '~/ui/Link';

export interface Props {
	isDarker: boolean;
	highlight: boolean;
}

function ResponsiveIcon() {
	return (
		<div>
			<ChevronDownIcon className="hidden md:block" />
			<MenuIcon className="block md:hidden" />
		</div>
	);
}

export function PageSelect({ isDarker, highlight }: Props) {
	const { x, y, refs, strategy } = useFloating({
		placement: 'bottom-end',
		middleware: [offset(5), flip(), shift()],
		whileElementsMounted: autoUpdate,
	});

	const pages = [
		{ id: '/recent?tab=checkpoints', label: 'Checkpoints' },
		{ id: '/recent', label: 'Transaction Blocks' },
		{ id: '/recent?tab=epochs', label: 'Epochs' },
		{ id: '/validators', label: 'Validators' },
		{ id: '/packages', label: 'Packages' },
	];

	return (
		<Popover>
			{({ open, close }) => (
				<>
					<Popover.Button
						ref={refs.setReference}
						as={NavItem}
						isDarker={isDarker}
						afterIcon={<ResponsiveIcon />}
					>
						<span
							className={clsx(
								'hidden pl-[3px] md:block',
								highlight
									? isDarker
										? 'text-white'
										: 'text-bfc'
									: isDarker
									? 'text-white/[0.72]'
									: 'text-bfc-text2',
							)}
						>
							Blockchain
						</span>
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
									className="z-20 flex flex-col rounded-lg border border-bfc-border bg-white p-1 shadow-[0px_16px_16px_0px_rgba(20,21,24,0.05)]"
									style={{
										position: strategy,
										top: y ?? 0,
										left: x ?? 0,
									}}
								>
									{pages.map((item) => (
										<Link
											key={item.id}
											className={clsx(
												'flex h-8 items-center rounded px-1.5 text-body/[18px] font-normal text-bfc-text2 hover:bg-bfc-card ui-active:bg-bfc-card',
											)}
											to={item.id}
											onClick={close}
										>
											<div className="">{item.label}</div>
										</Link>
									))}
								</Popover.Panel>
							)}
						</AnimatePresence>
					</FloatingPortal>
				</>
			)}
		</Popover>
	);
}
