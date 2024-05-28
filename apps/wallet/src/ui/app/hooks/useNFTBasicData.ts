// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { getObjectFields } from '@benfen/bfc.js';
=======
import type { SuiObjectData } from '@mysten/sui.js/client';
>>>>>>> mainnet-v1.24.1

import useFileExtensionType from './useFileExtensionType';
import useMediaUrl from './useMediaUrl';

<<<<<<< HEAD
import type { SuiObjectData } from '@benfen/bfc.js/client';

=======
>>>>>>> mainnet-v1.24.1
export default function useNFTBasicData(nftObj: SuiObjectData | null) {
	const nftObjectID = nftObj?.objectId || null;
	const filePath = useMediaUrl(nftObj?.content || null);
	let objType = null;
	let nftFields = null;
	if (nftObj && nftObj.content?.dataType === 'moveObject') {
		objType = nftObj.content?.type;
		nftFields = getObjectFields(nftObj);
	}
	const fileExtensionType = useFileExtensionType(filePath || '');
	return {
		nftObjectID,
		filePath,
		nftFields,
		fileExtensionType,
		objType,
	};
}
