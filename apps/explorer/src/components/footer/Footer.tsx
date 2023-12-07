// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { LegalLinks, LegalText } from './Legal';

function Footer() {
	return (
		<footer className="border-t border-bfc-border bg-gray-40">
			<nav className="flex w-full justify-center gap-4 divide-y divide-solid divide-gray-45 md:gap-7.5">
				<div className="flex w-full max-w-[1440px] flex-col justify-center gap-2.5 p-5 md:flex-row-reverse md:justify-between md:px-10">
					<LegalLinks />
					<div className="grow" />
					<LegalText />
				</div>
			</nav>
		</footer>
	);
}

export default Footer;
