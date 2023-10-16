import { useMutation } from '@tanstack/react-query';
import { toast } from 'react-hot-toast';
import { useTransactionExecution } from '../hooks/useTransactionExecution';
import { SUI_TYPE_ARG,TransactionBlock } from '@mysten/sui.js';
import { normalizeSuiCoinType, getAllCoins } from '~/utils/utils';
import BigNumber from 'bignumber.js';

type MutationParams = {
	onSuccess?: () => void;
	onError?: (e: Error) => void;
};

const defaultOnError = (e: Error) => {
	if (typeof e === 'string') toast.error(e);
	else toast.error(e?.message);
};

export function useSwapMutation({ onSuccess, onError }: MutationParams) {
	const { signAndExecute, provider } = useTransactionExecution();
	return useMutation({
		mutationFn: async ({ address, type, amount }: any) => {
			if (!address) throw new Error('You need to connect your wallet!');

			const swapAmount =  new BigNumber(amount).shiftedBy(9).toFixed();

			const typeArg = type === 'mint' ? SUI_TYPE_ARG : '0xc8::usd::USD';

			const coinType = normalizeSuiCoinType(typeArg);

			const coinsData = await getAllCoins(provider, address, coinType);

			// const coins = coinsData?.filter(({ lockedUntilEpoch: lock }) => !lock);
            
			// 3000000000

			const tx = new TransactionBlock();
			let coin;

			if (type === 'mint') {
				coin = tx.splitCoins(tx.gas, [tx.pure(swapAmount)]);
			} else {
				const [primaryCoin, ...mergeCoins] = coinsData.filter(
					(coin) => normalizeSuiCoinType(coin.coinType) === normalizeSuiCoinType(coinType),
				);
				const primaryCoinInput = tx.object(primaryCoin.coinObjectId);
				if (mergeCoins.length) {
					tx.mergeCoins(
						primaryCoinInput,
						mergeCoins.map((coin) => tx.object(coin.coinObjectId)),
					);
				}
				coin = tx.splitCoins(primaryCoinInput, [tx.pure(swapAmount)]);
			}

			const functionName = type === 'mint' ? 'swap_obc_to_stablecoin' : 'swap_stablecoin_to_obc';

			tx.moveCall({
				target: `0xc8::obc_system::${functionName}`,
				typeArguments: ['0xc8::usd::USD'],
				arguments: [tx.object('0xc9'), coin, tx.pure(Number.parseInt(swapAmount))],
			});
			return signAndExecute({ tx });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}
