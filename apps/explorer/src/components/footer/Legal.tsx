// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Text } from '@mysten/ui';

import { Link } from '~/ui/Link';

export function LegalText() {
	return (
		<div className="flex justify-center md:justify-start">
			<Text color="steel-darker" variant="pSubtitleSmall/medium">
				&copy;
				{`${new Date().getFullYear()} X Chain Explorer. All
  rights reserved.`}
			</Text>
		</div>
	);
}

export function LegalLinks() {
	return (
		<ul className="flex flex-col gap-3 md:flex-row md:gap-8">
			<li className="flex items-center justify-center">
				<Link variant="text" href="href">
					<Text variant="subtitleSmall/medium" color="steel-darker">
						Whitepaper
					</Text>
				</Link>
			</li>
			<li className="flex items-center justify-center">
				<Link variant="text" href="href">
					<Text variant="subtitleSmall/medium" color="steel-darker">
						Github
					</Text>
				</Link>
			</li>
			{/* {legalLinks.map(({ title, href }) => (
				<li className="flex items-center justify-center" key={href}>
					<Link variant="text" href={href}>
						<Text variant="subtitleSmall/medium" color="steel-darker">
							{title}
						</Text>
					</Link>
				</li>
			))}
			{productAnalyticsConfig?.mustProvideCookieConsent && (
				<li className="flex items-center justify-center">
					<Link variant="text" data-cc="c-settings">
						<Text variant="subtitleSmall/medium" color="steel-darker">
							Manage Cookies
						</Text>
					</Link>
				</li>
			)} */}
		</ul>
	);
}
