// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SuiTransactionBlockResponse } from '@benfen/bfc.js/client';
import {
	parseSerializedSignature,
	type SignatureScheme,
	type PublicKey,
} from '@benfen/bfc.js/cryptography';
import { parsePartialSignatures } from '@benfen/bfc.js/multisig';
import { toB64, normalizeSuiAddress } from '@benfen/bfc.js/utils';
import { publicKeyFromRawBytes } from '@benfen/bfc.js/verify';
import { Text } from '@mysten/ui';

import { DescriptionItem, DescriptionList } from '~/ui/DescriptionList';
import { AddressLink } from '~/ui/InternalLink';

interface SignaturePubkeyPair {
	signatureScheme: SignatureScheme;
	publicKey: PublicKey;
	signature: Uint8Array;
}

function SignaturePanel({ title, signature }: { title: string; signature: SignaturePubkeyPair }) {
	return (
		<div className="rounded-md border border-bfc-border px-2 py-5">
			<div className="text-heading6 font-semibold">{title}</div>
			<DescriptionList>
				<DescriptionItem title="Scheme" align="start" labelWidth="sm">
					<Text variant="pBody/medium" color="steel-darker">
						{signature.signatureScheme}
					</Text>
				</DescriptionItem>
				<DescriptionItem title="Address" align="start" labelWidth="sm">
					<AddressLink noTruncate address={signature.publicKey.toSuiAddress()} />
				</DescriptionItem>
				<DescriptionItem title="BenFen Public Key" align="start" labelWidth="sm">
					<Text variant="pBody/medium" color="steel-darker">
						{signature.publicKey.toSuiPublicKey()}
					</Text>
				</DescriptionItem>
				<DescriptionItem title="Signature" align="start" labelWidth="sm">
					<Text variant="pBody/medium" color="steel-darker">
						{toB64(signature.signature)}
					</Text>
				</DescriptionItem>
			</DescriptionList>
		</div>
	);
}

function getSignatureFromAddress(signatures: SignaturePubkeyPair[], suiAddress: string) {
	return signatures.find(
		(signature) => signature.publicKey.toSuiAddress() === normalizeSuiAddress(suiAddress),
	);
}

function getSignaturesExcludingAddress(
	signatures: SignaturePubkeyPair[],
	suiAddress: string,
): SignaturePubkeyPair[] {
	return signatures.filter(
		(signature) => signature.publicKey.toSuiAddress() !== normalizeSuiAddress(suiAddress),
	);
}
interface Props {
	transaction: SuiTransactionBlockResponse;
}

export function Signatures({ transaction }: Props) {
	const sender = transaction.transaction?.data.sender;
	const gasData = transaction.transaction?.data.gasData;
	const transactionSignatures = transaction.transaction?.txSignatures;

	if (!transactionSignatures) return null;

	const isSponsoredTransaction = gasData?.owner !== sender;

	const deserializedTransactionSignatures = transactionSignatures
		.map((signature) => {
			const parsed = parseSerializedSignature(signature);

			if (parsed.signatureScheme === 'MultiSig') {
				return parsePartialSignatures(parsed.multisig);
			}

			return {
				...parsed,
				publicKey: publicKeyFromRawBytes(parsed.signatureScheme, parsed.publicKey),
			};
		})
		.flat();

	const userSignatures = isSponsoredTransaction
		? getSignaturesExcludingAddress(deserializedTransactionSignatures, gasData!.owner)
		: deserializedTransactionSignatures;

	const sponsorSignature = isSponsoredTransaction
		? getSignatureFromAddress(deserializedTransactionSignatures, gasData!.owner)
		: null;

	return (
		<div className="flex flex-col gap-8">
			{userSignatures.length > 0 && (
				<div className="flex flex-col gap-8">
					{userSignatures.map((signature, index) => (
						<div key={index}>
							<SignaturePanel title="User Signature" signature={signature} />
						</div>
					))}
				</div>
			)}

			{sponsorSignature && (
				<SignaturePanel title="Sponsor Signature" signature={sponsorSignature} />
			)}
		</div>
	);
}
