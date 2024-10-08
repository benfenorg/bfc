// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type TransactionBlock } from '@benfen/bfc.js/transactions';
import { Tab as HeadlessTab, type TabProps } from '@headlessui/react';

import { Command } from './Command';
import { Input } from './Input';
import { SummaryCard } from '../SummaryCard';
import { useTransactionData } from '_src/ui/app/hooks';

interface Props {
	sender?: string;
	transaction: TransactionBlock;
}

const Tab = (props: TabProps<'div'>) => (
	<HeadlessTab
		className="border-0 border-b border-transparent ui-selected:border-bfc-text1 text-bfc-text2 p-0 pb-2 -mb-px border-solid ui-selected:text-bfc-text1 text-body font-semibold bg-transparent outline-none cursor-pointer"
		{...props}
	/>
);

export function TransactionDetails({ sender, transaction }: Props) {
	const { data: transactionData, isLoading, isError } = useTransactionData(sender, transaction);
	if (transactionData?.transactions.length === 0 && transactionData.inputs.length === 0) {
		return null;
	}
	return (
		<SummaryCard header="Transaction Details" initialExpanded>
			{isLoading || isError ? (
				<div className="ml-0 text-bfc-text1 text-body font-medium">
					{isLoading ? 'Gathering data...' : "Couldn't gather data"}
				</div>
			) : transactionData ? (
				<div>
					<HeadlessTab.Group>
						<HeadlessTab.List className="flex gap-6 border-0 border-b border-solid border-bfc-border mb-2.5">
							{!!transactionData.transactions.length && <Tab>Transactions</Tab>}
							{!!transactionData.inputs.length && <Tab>Inputs</Tab>}
						</HeadlessTab.List>
						<HeadlessTab.Panels>
							{!!transactionData.transactions.length && (
								<HeadlessTab.Panel className="flex flex-col gap-2.5">
									{/* TODO: Rename components: */}
									{transactionData.transactions.map((command, index) => (
										<Command key={index} command={command} />
									))}
								</HeadlessTab.Panel>
							)}
							{!!transactionData.inputs.length && (
								<HeadlessTab.Panel className="flex flex-col gap-2.5">
									{transactionData.inputs.map((input, index) => (
										<Input key={index} input={input} />
									))}
								</HeadlessTab.Panel>
							)}
						</HeadlessTab.Panels>
					</HeadlessTab.Group>
				</div>
			) : (
				''
			)}
		</SummaryCard>
	);
}
