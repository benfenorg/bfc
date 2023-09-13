// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '@mysten/ui';

import { LegalLinks, LegalText } from './Legal';
import { footerLinks, socialLinks } from './footerLinks';
import { ReactComponent as MystenLabsRed } from '../../assets/MystenLabs_Red.svg';
import { Link } from '~/ui/Link';

function FooterLinks() {
	return (
		<div className="flex flex-col items-center justify-center gap-6 md:flex-row md:justify-end">
			<ul className="flex gap-4 md:flex-row md:gap-6">
				{footerLinks.map(({ title, href }) => (
					<li key={href}>
						<Link variant="text" href={href}>
							<Text variant="body/medium" color="steel-darker">
								{title}
							</Text>
						</Link>
					</li>
				))}
			</ul>

			<ul className="flex justify-center gap-6">
				{socialLinks.map(({ children, href }) => (
					<li key={href}>
						<Link variant="text" color="steel-darker" href={href}>
							<div className="mt-2">{children}</div>
						</Link>
					</li>
				))}
			</ul>
		</div>
	);
}

function Footer() {
	return (
		<footer className="bg-gray-40 px-5 py-10 md:px-4 md:py-4">
			<nav className="flex flex-col justify-center gap-4 divide-y divide-solid divide-gray-45 md:gap-7.5">
				<div className="flex flex-col-reverse justify-center gap-3 md:flex-row md:justify-between">
					<LegalText />
					<LegalLinks />
				</div>
			</nav>
		</footer>
	);
}

export default Footer;
