// Copyright (c) Benfen
// SPDX-License-Identifier: Apache-2.0

import { ErrorBoundary } from '_components/error-boundary';
import { ampli } from '_src/shared/analytics/ampli';
import { NFTDisplayCard } from '_src/ui/app/components/nft-display';
import { type SuiObjectData } from '@benfen/bfc.js/client';
import { Link } from 'react-router-dom';

export default function VisualAssets({ items }: { items: SuiObjectData[] }) {
	return (
		<div className="grid w-full grid-cols-2 gap-x-3.5 gap-y-4">
			{items.map((object) => (
				<Link
					to={`/nft-details?${new URLSearchParams({
						objectId: object.objectId,
					}).toString()}`}
					onClick={() => {
						ampli.clickedCollectibleCard({
							objectId: object.objectId,
							collectibleType: object.type!,
						});
					}}
					key={object.objectId}
					className="no-underline relative"
				>
					<div className="group">
						<div className="w-full h-full justify-center z-10 absolute pointer-events-auto text-gray-60 transition-colors duration-200 p-0"></div>
						<ErrorBoundary>
							<NFTDisplayCard
								hideLabel={false}
								objectId={object.objectId}
								size="lg"
								animateHover
								borderRadius="xl"
							/>
						</ErrorBoundary>
					</div>
				</Link>
			))}
		</div>
	);
}
