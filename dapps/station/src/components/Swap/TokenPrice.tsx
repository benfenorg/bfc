import classnames from 'classnames';
import { Token } from '~/utils/constants';
import { ReactComponent as ArrowDropDownIcon } from '~/assets/arrow_drop_down.svg';

interface TokenPriceProps {
	fromToken: Token;
	toToken: Token;
	showChart: boolean;
	setShowChart: (showChart: boolean) => void;
}

export function TokenPrice({ fromToken, toToken, showChart, setShowChart }: TokenPriceProps) {
	return (
		<div
			className={classnames(
				'flex justify-between p-2.5  border border-bf-border rounded-lg cursor-pointer',
				showChart ? 'bg-bf-card' : '',
			)}
            onClick={()=>setShowChart(!showChart)}
		>
			<div className="flex items-center font-semibold text-sm">
				<img className="w-4 h-4 rounded-full" src={fromToken.logo} alt={fromToken.name} />
				<img className="w-4 h-4 rounded-full ml-[-2px]" src={toToken.logo} alt={toToken.name} />
				<span className="ml-1">
					{fromToken.name}/{toToken.name}
				</span>
			</div>
			<div className="flex items-center font-semibold">
				<span>20,000,00.56</span>
				<span className="ml-1 text-bf-green">+0.05%</span>
				<ArrowDropDownIcon className={classnames(showChart ? "rotate-90" : "-rotate-90")} />
			</div>
		</div>
	);
}
