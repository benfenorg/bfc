// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { ReactComponent as DaoAgree } from '../../assets/DaoAgree.svg';
import { ReactComponent as DaoReject } from '../../assets/DaoReject.svg';

export function AgreeSpan() {
	return (
		<div className="flex h-5 items-center gap-0.5 rounded border border-bfc-green bg-bfc-green_10p px-1 text-body font-medium text-bfc-green">
			<DaoAgree />
			同意
		</div>
	);
}

export function RejectSpan() {
	return (
		<div className="flex h-5 items-center gap-0.5 rounded border border-bfc-green bg-bfc-green_10p px-1 text-body font-medium text-bfc-green">
			<DaoReject />
			反对
		</div>
	);
}

export function StatusSpan({ text }: { text: string }) {
	return (
		<div className="flex h-5 items-center rounded border border-bfc-border  px-1 text-body font-medium text-bfc-text1">
			{text}
		</div>
	);
}
