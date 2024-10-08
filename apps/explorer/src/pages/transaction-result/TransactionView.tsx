// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getExecutionStatusError, getExecutionStatusType } from '@benfen/bfc.js';
import { type SuiTransactionBlockResponse } from '@benfen/bfc.js/client';
import clsx from 'clsx';
import { type ReactNode, useState } from 'react';

import { Signatures } from './Signatures';
import { ErrorBoundary } from '~/components/error-boundary/ErrorBoundary';
import { useBreakpoint } from '~/hooks/useBreakpoint';
import { Events } from '~/pages/transaction-result/Events';
import { TransactionData } from '~/pages/transaction-result/TransactionData';
import { TransactionSummary } from '~/pages/transaction-result/transaction-summary';
import { PageHeader } from '~/ui/PageHeader';
import { LOCAL_STORAGE_SPLIT_PANE_KEYS, SplitPanes } from '~/ui/SplitPanes';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '~/ui/Tabs';

import styles from './TransactionResult.module.css';

function TabsContentContainer({ value, children }: { value: string; children: ReactNode }) {
	return (
		<TabsContent value={value}>
			<div className="mt-6 md:mt-10">{children}</div>
		</TabsContent>
	);
}

export function TransactionView({ transaction }: { transaction: SuiTransactionBlockResponse }) {
	const isMediumOrAbove = useBreakpoint('md');
	const [isCollapsed, setIsCollapsed] = useState(false);

	const hasEvents = !!transaction.events?.length;

	const transactionKindName = transaction.transaction?.data.transaction?.kind;

	const isProgrammableTransaction = transactionKindName === 'ProgrammableTransaction';

	const leftPane = {
		panel: (
			<div className="h-full overflow-y-auto border border-transparent md:h-full md:max-h-screen">
				<Tabs size="md" defaultValue="summary">
					<TabsList disableBottomBorder>
						<TabsTrigger value="summary">Summary</TabsTrigger>
						{hasEvents && <TabsTrigger value="events">Events</TabsTrigger>}
						{isProgrammableTransaction && <TabsTrigger value="signatures">Signatures</TabsTrigger>}
					</TabsList>
					<TabsContentContainer value="summary">
						<TransactionSummary transaction={transaction} />
					</TabsContentContainer>
					{hasEvents && (
						<TabsContentContainer value="events">
							<Events events={transaction.events!} />
						</TabsContentContainer>
					)}
					<TabsContentContainer value="signatures">
						<ErrorBoundary>
							<Signatures transaction={transaction} />
						</ErrorBoundary>
					</TabsContentContainer>
				</Tabs>
			</div>
		),
		minSize: 35,
		collapsible: true,
		collapsibleButton: true,
		noHoverHidden: isMediumOrAbove,
	};

	const rightPane = {
		panel: (
			<div
				className={clsx(
					'h-full w-full overflow-y-auto md:overflow-y-hidden',
					isCollapsed && isMediumOrAbove && 'pl-2',
				)}
			>
				<TransactionData transaction={transaction} />
			</div>
		),
		minSize: 40,
		defaultSize: isProgrammableTransaction ? 65 : 50,
	};

	return (
		<div className={clsx(styles.txdetailsbg)}>
			<div className="mb-10">
				<PageHeader
					type="Transaction"
					title={transaction?.digest}
					status={transaction?.effects?.status?.status}
					statusMessage={
						getExecutionStatusType(transaction) === 'failure'
							? getExecutionStatusError(transaction) || 'Transaction failed'
							: ''
					}
				/>
			</div>
			<div className="h-screen md:h-full">
				<SplitPanes
					autoSaveId={LOCAL_STORAGE_SPLIT_PANE_KEYS.TRANSACTION_VIEW}
					onCollapse={setIsCollapsed}
					dividerSize={isMediumOrAbove ? 'md' : 'lg'}
					splitPanels={[leftPane, rightPane]}
					direction={isMediumOrAbove ? 'horizontal' : 'vertical'}
				/>
			</div>
		</div>
	);
}
