// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { BenFenLogoLight, BenFenTextLight } from '@mysten/icons';

const Logo = () => {
	return (
		<div className="flex items-center gap-1.25">
			<BenFenLogoLight className="h-7" />
			<BenFenTextLight className="w-[50px]" />
		</div>
	);
};

export default Logo;
