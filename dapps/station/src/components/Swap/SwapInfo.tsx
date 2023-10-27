import { useState } from 'react';
import CircularProgress from './CircularProgress';
import classNames from 'classnames';
import { Tooltip } from 'react-tooltip';
import { Question } from '../Base/Question';

import { ReactComponent as RateChangeIcon } from '~/assets/rate_change_icon.svg';
import { ReactComponent as ArrowDropDownIcon } from '~/assets/arrow_drop_down.svg';

interface SwapInfoProps {
	type: string;
}
export function SwapInfo({ type }: SwapInfoProps) {
	const [showDetail, setShowDetail] = useState(true);
	return (
		<div className="mb-2.5 border border-bf-border rounded-lg cursor-pointer overflow-hidden">
			<div
				className={classNames(
					'flex items-center p-2.5 cursor-pointer',
					type === 'home' ? ' justify-between' : 'justify-center',
				)}
				onClick={() => {
					setShowDetail(!showDetail);
				}}
			>
				<div className="flex items-center">
					<div className="w-3 h-3">
						<CircularProgress loading={false} updatedAt={0} />
					</div>
					<div className="ml-1 text-sm">1 BFC ≈ 2,563,255.40 BUSD</div>
					<RateChangeIcon className="ml-1" />
				</div>
				{type === 'home' && <ArrowDropDownIcon/>}
			</div>
			<div
				className={classNames(
					'grid gap-2.5 bg-bf-card text-xs',
					showDetail ? 'h-auto p-2.5' : 'h-0 p-0',
				)}
			>
				<div className="flex justify-between item-center">
					<div>Gas 费</div>
					<div>0.2323225 BFC</div>
				</div>
				<div className="flex justify-between item-center">
					<div className="flex items-center">
						价格影响&nbsp;
						<Question id="priceque" />
					</div>
					<div>0.2323225 BFC</div>
				</div>
				<div className="flex justify-between item-center">
					<div>Gas 费</div>
					<div>0.2323225 BFC</div>
				</div>
				<div className="flex justify-between item-center">
					<div>Gas 费</div>
					<div>0.2323225 BFC</div>
				</div>
			</div>
			<Tooltip anchorSelect="#priceque" content="Hello world! priceque" />
			<Tooltip id="my-tooltip" />
		</div>
	);
}
