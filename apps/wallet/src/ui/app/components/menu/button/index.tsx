// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useMenuIsOpen, useNextMenuUrl } from '_components/menu/hooks';
import { Close, Menu } from '@mysten/icons';
import cl from 'classnames';
import { memo } from 'react';
import { Link } from 'react-router-dom';

import st from './MenuButton.module.scss';

export type MenuButtonProps = {
	className?: string;
};

function MenuButton({ className }: MenuButtonProps) {
	const isOpen = useMenuIsOpen();
	const menuUrl = useNextMenuUrl(!isOpen, '/');
	return (
		<Link data-testid="menu" className={cl(st.button, className)} to={menuUrl}>
			{isOpen ? <Close className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
		</Link>
	);
}

export default memo(MenuButton);
