
import { useMutation } from '@tanstack/react-query';
import { toast } from 'react-hot-toast';
import { TransactionBlock } from '@mysten/sui.js/transactions';
import { useTransactionExecution } from '../hooks/useTransactionExecution';
type MutationParams = {
	onSuccess?: () => void;
	onError?: (e: Error) => void;
};

const defaultOnError = (e: Error) => {
	if (typeof e === 'string') toast.error(e);
	else toast.error(e?.message);
};

export function useSwapMutation({ onSuccess, onError }:MutationParams){
    const { signAndExecute } = useTransactionExecution();
    return useMutation({
		mutationFn: ({
            address,
            type, 
            amount
        }:any) => {
			if (!address) throw new Error('You need to connect your wallet!');
			const tx = new TransactionBlock();
            const coin = tx.splitCoins(tx.gas, [tx.pure(amount)]);

            const functionName = type === 'mint' ? 'swap_obc_to_stablecoin' : 'swap_stablecoin_to_obc';

            tx.moveCall({
				target: `0xc8::obc_system::${functionName}`,
				typeArguments: ['0xc8::usd::USD'],
				arguments: [
					tx.object('0xc9'),
					coin,
                    tx.pure(Number.parseInt(amount)),
				],
			});
			return signAndExecute({ tx });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}