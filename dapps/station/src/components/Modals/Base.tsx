// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Fragment, ReactNode } from 'react';
import { Dialog, Transition } from '@headlessui/react';
import { ReactComponent as CloseIcon } from '~/assets/close_icon.svg';
import { ReactComponent as TitleLoadingIcon } from '~/assets/title_loading_icon.svg';

export function ModalBase({
	isOpen,
	titleLoading = false,
	maskClose = false,
	iconClose = false,
	closeModal,
	children,
	title = 'A modal',
}: {
	isOpen: boolean;
	closeModal: () => void;
	children: ReactNode;
	title: string;
	maskClose?: boolean;
	iconClose?: boolean;
	titleLoading?: boolean;
}) {
	return (
		<Transition appear show={isOpen} as={Fragment}>
			<Dialog as="div" className="fixed inset-0 z-10 overflow-y-auto" onClose={maskClose ? closeModal : ()=>{}}>
				<div className="min-h-screen px-4 text-center bg-primary bg-opacity-60">
					<Transition.Child
						as={Fragment}
						enter="ease-out duration-300"
						enterFrom="opacity-0"
						enterTo="opacity-100"
						leave="ease-in duration-200"
						leaveFrom="opacity-100"
						leaveTo="opacity-0"
					>
						<Dialog.Overlay className="fixed inset-0" />
					</Transition.Child>

					<span className="inline-block h-screen align-middle" aria-hidden="true">
						&#8203;
					</span>
					<Transition.Child
						as={Fragment}
						enter="ease-out duration-300"
						enterFrom="opacity-0 scale-95"
						enterTo="opacity-100 scale-100"
						leave="ease-in duration-200"
						leaveFrom="opacity-100 scale-100"
						leaveTo="opacity-0 scale-95"
					>
						<div className="inline-block w-full max-w-[400px] my-8 overflow-hidden text-left align-middle transition-all transform bg-white shadow-xl rounded-lg">
							<Dialog.Title
								as="h3"
								className="flex items-center justify-between py-3 px-2.5 bg-bf-card font-medium text-bf-text1"
							>
								<div className="flex items-center">
									{titleLoading && <TitleLoadingIcon className="mr-1 animate-spin"/>}
									{title}
								</div>
								{iconClose && <div className="cursor-pointer" onClick={closeModal}>
									<CloseIcon />
								</div>}
							</Dialog.Title>
							<div className="text-bf-text1">{children}</div>
						</div>
					</Transition.Child>
				</div>
			</Dialog>
		</Transition>
	);
}
