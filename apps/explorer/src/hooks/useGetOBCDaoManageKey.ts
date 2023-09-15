import { useQuery } from '@tanstack/react-query';
import { useWalletKit } from '@mysten/wallet-kit';
import {
	useRpcClient,
} from '@mysten/core';

export function useGetOBCDaoManageKey() {
	const rpc = useRpcClient();
	const { currentAccount } = useWalletKit();
	return useQuery({
		queryKey: ['dao', 'object', 'currentAccount81'],
		enabled: Boolean(currentAccount?.address),
		queryFn: () => rpc.getOwnedObjects({
			owner:currentAccount?.address,
			filter: {
				StructType: 
				"0x00000000000000000000000000000000000000000000000000000000000000c8::obc_dao_manager::OBCDaoManageKey",
			},
			options: {
				showType: true
			}
		}).then((res:any)=>{
			if(res?.data?.length > 0){
				return res.data[0]?.data?.objectId ?? ''
			}
			return ''
		})
	});
}