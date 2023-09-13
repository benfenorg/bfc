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

import { Link } from '~/ui/Link';
import { NavItem } from './NavItem';
import { ReactComponent as ChevronDownIcon } from '../icons/chevron_down.svg';
import { ReactComponent as MenuIcon } from '../icons/menu.svg';


function ResponsiveIcon() {
	return (
		<div>
			<ChevronDownIcon className="hidden md:block" />
			<MenuIcon className="block md:hidden" />
		</div>
	);
}

export function PageSelect({
	isDarker,
}: any) {
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
    ]

	return (
		<Popover>
			{({ open, close }) => (
				<>
					<Popover.Button ref={refs.setReference} as={NavItem} isDarker={isDarker} afterIcon={<ResponsiveIcon />}>
						<span className="hidden md:block">Blockchain</span>
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
									className="z-20 flex w-52 flex-col gap-2 rounded-lg border border-steel-dark border-opacity-10 bg-white px-3 py-4 shadow-lg focus:outline-none"
									style={{
										position: strategy,
										top: y ?? 0,
										left: x ?? 0,
									}}
								>
                                    {pages.map((item)=>{
                                        return(
                                            <div
                                                key={item.id}
                                                role="button"
                                                onClick={close}
                                                className={clsx(
                                                    'flex items-start gap-3 rounded-md px-1.25 py-2 text-body font-semibold hover:bg-gray-40 ui-active:bg-gray-40 text-steel-dark'
                                                )}
                                            >  
                                                <Link to={item.id} className="w-full">
                                                    <div className="mt-px">
                                                    
                                                            {item.label}
                                                    
                                                    </div>
                                                </Link>
                                            </div>
                                        )
                                    })}
								</Popover.Panel>
							)}
						</AnimatePresence>
					</FloatingPortal>
				</>
			)}
		</Popover>
	);
}
