// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { SUI_TYPE_ARG } from '@benfen/bfc.js/utils';
import { useSearchParams, useNavigate } from 'react-router-dom';

=======
>>>>>>> mainnet-v1.24.1
import { ActiveCoinsCard } from '_components/active-coins-card';
import Overlay from '_components/overlay';
import { useUnlockedGuard } from '_src/ui/app/hooks/useUnlockedGuard';
import { SUI_TYPE_ARG } from '@mysten/sui.js/utils';
import { useNavigate, useSearchParams } from 'react-router-dom';

function CoinsSelectorPage() {
	const [searchParams] = useSearchParams();
	const coinType = searchParams.get('type') || SUI_TYPE_ARG;
	const navigate = useNavigate();

	if (useUnlockedGuard()) {
		return null;
	}

	return (
		<Overlay
			showModal={true}
			title="Select Coin"
			closeOverlay={() =>
				navigate(
					`/send?${new URLSearchParams({
						type: coinType,
					}).toString()}`,
				)
			}
		>
			<ActiveCoinsCard activeCoinType={coinType} showActiveCoin={false} />
		</Overlay>
	);
}

export default CoinsSelectorPage;
