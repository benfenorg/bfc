// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { ArrowUpRight16 } from '@mysten/icons';

import { type ExplorerLinkConfig, useExplorerLink } from '../../hooks/useExplorerLink';
=======
>>>>>>> mainnet-v1.24.1
import ExternalLink from '_components/external-link';
import { ArrowUpRight16 } from '@mysten/icons';
import { formatAddress } from '@mysten/sui.js/utils';
import type { ReactNode } from 'react';

import { useExplorerLink, type ExplorerLinkConfig } from '../../hooks/useExplorerLink';
import { Text } from '../../shared/text';
import st from './ExplorerLink.module.scss';
import { ExplorerLinkType } from './ExplorerLinkType';

export type ExplorerLinkProps = ExplorerLinkConfig & {
	track?: boolean;
	children?: ReactNode;
	className?: string;
	title?: string;
	showIcon?: boolean;
};

function ExplorerLink({
	track,
	children,
	className,
	title,
	showIcon,
	...linkConfig
}: ExplorerLinkProps) {
	const explorerHref = useExplorerLink(linkConfig);
	if (!explorerHref) {
		return null;
	}

	return (
		<ExternalLink href={explorerHref} className={className} title={title}>
			<>
				{children} {showIcon && <ArrowUpRight16 className={st.explorerIcon} />}
			</>
		</ExternalLink>
	);
}

export default ExplorerLink;
