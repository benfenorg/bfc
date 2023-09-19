// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { LegalLinks, LegalText } from './Legal';

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
