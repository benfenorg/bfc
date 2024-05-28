// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

<<<<<<< HEAD
import { type TransactionBlock } from '@benfen/bfc.js/transactions';
=======
import { useTransactionData } from '_src/ui/app/hooks';
>>>>>>> mainnet-v1.24.1
import { Tab as HeadlessTab, type TabProps } from '@headlessui/react';

import { SummaryCard } from '../SummaryCard';
import { Command } from './Command';
import { Input } from './Input';

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
	const { data: transactionData, isPending, isError } = useTransactionData(sender, transaction);
	if (transactionData?.transactions.length === 0 && transactionData.inputs.length === 0) {
		return null;
	}
	return (
		<SummaryCard header="Transaction Details" initialExpanded>
<<<<<<< HEAD
			{isLoading || isError ? (
				<div className="ml-0 text-bfc-text1 text-body font-medium">
					{isLoading ? 'Gathering data...' : "Couldn't gather data"}
=======
			{isPending || isError ? (
				<div className="ml-0 text-steel-darker text-pBodySmall font-medium">
					{isPending ? 'Gathering data...' : "Couldn't gather data"}
>>>>>>> mainnet-v1.24.1
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
