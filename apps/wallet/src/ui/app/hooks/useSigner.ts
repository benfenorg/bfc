// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { useAccounts } from './useAccounts';
import { useActiveAccount } from './useActiveAccount';
import useAppSelector from './useAppSelector';
import { useQredoAPI } from './useQredoAPI';
import { LedgerSigner } from '../LedgerSigner';
import { QredoSigner } from '../QredoSigner';
import { type WalletSigner } from '../WalletSigner';
import { useSuiLedgerClient } from '../components/ledger/SuiLedgerClientProvider';
import { thunkExtras } from '_redux/store/thunk-extras';
import { AccountType } from '_src/background/keyring/Account';

export function useSigner(address?: string): WalletSigner | null {
	const activeAccount = useActiveAccount();
	const existingAccounts = useAccounts();
	const signerAccount = address
		? existingAccounts.find((account) => account.address === address)
		: activeAccount;
=======
import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { isLedgerAccountSerializedUI } from '_src/background/accounts/LedgerAccount';
import { isQredoAccountSerializedUI } from '_src/background/accounts/QredoAccount';
import { useSuiClient } from '@mysten/dapp-kit';

import { walletApiProvider } from '../ApiProvider';
import { useSuiLedgerClient } from '../components/ledger/SuiLedgerClientProvider';
import { LedgerSigner } from '../LedgerSigner';
import { QredoSigner } from '../QredoSigner';
import { type WalletSigner } from '../WalletSigner';
import useAppSelector from './useAppSelector';
import { useBackgroundClient } from './useBackgroundClient';
import { useQredoAPI } from './useQredoAPI';
>>>>>>> mainnet-v1.24.1

	const { connectToLedger } = useSuiLedgerClient();
	const { api, background } = thunkExtras;
	const [qredoAPI] = useQredoAPI(
		signerAccount?.type === AccountType.QREDO ? signerAccount.qredoConnectionID : undefined,
	);
	const networkName = useAppSelector(({ app: { apiEnv } }) => apiEnv);
	if (!signerAccount) {
		throw new Error("Can't find account for the signer address");
	}

	if (signerAccount.type === AccountType.LEDGER) {
		return new LedgerSigner(connectToLedger, signerAccount.derivationPath, api.instance.fullNode);
	}
	if (signerAccount.type === AccountType.QREDO) {
		return qredoAPI
			? new QredoSigner(api.instance.fullNode, signerAccount, qredoAPI, networkName)
			: null;
	}
<<<<<<< HEAD
	return api.getSignerInstance(signerAccount, background);
=======
	return walletApiProvider.getSignerInstance(account, background);
>>>>>>> mainnet-v1.24.1
}
